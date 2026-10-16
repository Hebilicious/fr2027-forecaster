# Worker

One Cloudflare Worker, `fr2027`, serves two things:

- **The public site.** Static assets from `.site/` at the repository root, which `moon run cli:export` writes (the built UI plus `api/*.json`). Unknown paths fall back to `index.html` for the single-page app. No auth, and the Worker script never runs for these requests.
- **The inbox** at `/inbox/*`, for the Grok bot, which cannot hold a GitHub credential. The bot posts items; a scheduled GitHub Actions job pulls the pending ones, validates them with `fr2027`, commits the accepted ones and resolves each item. Items live in one SQLite-backed Durable Object (`Inbox`).

The site is deployed by the hourly GitHub Actions workflow, which runs `moon run worker:deploy` (it exports the site first) with `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` set.

| Path               | Role                                                                                             |
| ------------------ | ------------------------------------------------------------------------------------------------ |
| `src/inbox.ts`     | The whole inbox contract, pure: routing, auth, validation, limits, state changes, retention      |
| `src/sql-store.ts` | The inbox's storage in SQLite                                                                    |
| `src/index.ts`     | The entry point (`/inbox/*` goes to the Durable Object, anything else to the assets) and `Inbox` |
| `wrangler.jsonc`   | Name, assets, Durable Object binding and migration; no account id (it comes from CI)             |

## Inbox contract

Every request carries `Authorization: Bearer <token>`: `INBOX_BOT_TOKEN` for the bot's routes, `INBOX_PIPELINE_TOKEN` for the pipeline's. Every answer is JSON with `cache-control: no-store`; errors are `{"error": "<message>"}`.

| Route                            | Token    | Body                                                                                  | Answer                                                                                                                                |
| -------------------------------- | -------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `POST /inbox/items`              | bot      | `{"kind": "x_drop" \| "poll" \| "event", "payload": {…}}`, at most 256 KiB            | 201 `{"id", "status": "pending"}`                                                                                                     |
| `GET /inbox/items`               | bot      |                                                                                       | 200 `{"items": [{"id", "kind", "received_at", "status", "note", "ref", "resolved_at"}]}`: the last 30 days, newest first, no payloads |
| `GET /inbox/pending`             | pipeline |                                                                                       | 200 `{"items": [{"id", "kind", "received_at", "payload"}]}`: pending items, oldest first, at most 100                                 |
| `POST /inbox/items/{id}/resolve` | pipeline | `{"status": "accepted" \| "rejected" \| "proposed", "note": "…", "ref": "…" \| null}` | 200 the item, without its payload                                                                                                     |

- Ids are `<received_at as YYYYMMDDTHHMMSSZ>-<16 hex digits>`, so they sort by arrival to the second. `received_at` and `resolved_at` are UTC, `YYYY-MM-DDTHH:MM:SSZ`.
- An item is resolved once, from `pending`. Accepted and proposed items drop their payload; rejected items keep it. `note` is at most 2000 characters; `ref` (the committed path or a pull request URL) is optional.
- Items older than 30 days are deleted on the next write.
- Statuses: 400 the body is not JSON or has the wrong shape (the message says what; unknown fields are refused), 401 missing or wrong token, 404 unknown route or item, 405 wrong method (the `allow` header lists the right ones), 409 item already resolved, 413 body over 256 KiB, 429 already 200 items pending, 503 the route's token is not configured.

```sh
INBOX=https://fr2027.<account subdomain>.workers.dev/inbox

# The bot: submit an item (item.json is {"kind": ..., "payload": {...}}), then see what became of its items
curl -sS -X POST "$INBOX/items" -H "Authorization: Bearer $INBOX_BOT_TOKEN" \
  -H "Content-Type: application/json" --data-binary @item.json
curl -sS "$INBOX/items" -H "Authorization: Bearer $INBOX_BOT_TOKEN"

# The pipeline: pull pending items, resolve one
curl -sS "$INBOX/pending" -H "Authorization: Bearer $INBOX_PIPELINE_TOKEN"
curl -sS -X POST "$INBOX/items/20261009T142233Z-9f3a1c0b7e2d4a51/resolve" \
  -H "Authorization: Bearer $INBOX_PIPELINE_TOKEN" -H "Content-Type: application/json" \
  -d '{"status": "rejected", "note": "fieldwork_end is missing"}'
```

## Secrets

The two tokens are Worker secrets, never in the repository. Until one is set, its routes answer 503 and the rest of the Worker, the site included, works. After the first deploy, from `app/worker/`:

```sh
openssl rand -hex 32                                  # make a token; keep it for the bot or for GitHub
pnpm exec wrangler secret put INBOX_BOT_TOKEN         # prompts for the value
pnpm exec wrangler secret put INBOX_PIPELINE_TOKEN
```

From CI, pipe the value in instead: `printf %s "$TOKEN" | pnpm exec wrangler secret put INBOX_BOT_TOKEN`. Give the bot its token, and store the pipeline's as a GitHub Actions secret for the pull job. Putting a new value rotates a token at once.

## Tasks

| Task                        | What it does                                                                                       |
| --------------------------- | -------------------------------------------------------------------------------------------------- |
| `moon run worker:typecheck` | Generate `worker-configuration.d.ts` with `wrangler types` (git-ignored), then type-check with tsc |
| `moon run worker:test`      | The inbox tests, in plain Node, on an in-memory store and on the SQL store over `node:sqlite`      |
| `moon run worker:dev`       | `wrangler dev`: the site from `.site/` and the inbox, tokens from `app/worker/.dev.vars`           |
| `moon run worker:deploy`    | Export the site, then `wrangler deploy`                                                            |

For `worker:dev`, run `moon run cli:export` once and put throwaway tokens in `app/worker/.dev.vars` (git-ignored):

```sh
INBOX_BOT_TOKEN=dev-bot
INBOX_PIPELINE_TOKEN=dev-pipeline
```
