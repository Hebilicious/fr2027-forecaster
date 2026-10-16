# Running the live forecaster

The forecaster runs itself from GitHub Actions and publishes a public page on Cloudflare. Nothing
needs a server or a computer left on.

## What runs where

| When                                                    | Where                                                       | What                                                                                                                                                                                                                                                             |
| ------------------------------------------------------- | ----------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Every hour at :17                                       | GitHub Actions, [`live.yml`](../.github/workflows/live.yml) | Prediction-market prices (Polymarket, Kalshi), new headlines from 13 RSS feeds, Wikipedia page views (once a day, after 06:00 UTC), the Grok Bot inbox. Then ingest, refit when the inputs changed, validate, commit the new data to `main`, and deploy the site |
| On a merge to `main` that changes polls, config or code | GitHub Actions, `live.yml`                                  | The same, so the page follows `main` right away                                                                                                                                                                                                                  |
| On every pull request and push to `main`                | GitHub Actions, [`ci.yml`](../.github/workflows/ci.yml)     | `moon ci`: tests, lint, format and type checks, and `fr2027 validate`                                                                                                                                                                                            |
| Every 6 hours, twice a day, every evening               | Grok Bot routines ([grok-bot.md](grok-bot.md))              | X activity, new polls, campaign events, sent to the inbox                                                                                                                                                                                                        |
| Always                                                  | Cloudflare Worker `fr2027` ([`app/worker`](../app/worker))  | The public page (static files, rebuilt hourly) and the Grok Bot inbox                                                                                                                                                                                            |

The model refits once a day (the forecast date changes) and whenever polls or config change; an
hour with no new input doesn't add a forecast file. Every collector writes one new file per run
under `data/raw/` and never edits an old one. A source that fails is recorded with its error and
warned about in the run's summary; it never stops the others.

## One-time setup

### 1. GitHub settings

In the repository's **Settings → Actions → General**:

- **Workflow permissions:** keep "Read repository contents and packages permissions". Each
  workflow asks for exactly what it needs: `live.yml` writes to the repository, `ci.yml` only reads.
- **Allow GitHub Actions to create and approve pull requests:** on. The Live workflow opens a pull
  request for each batch of polls Grok Bot finds. Without it, the branch is still pushed, and the
  run's summary links to it so you can open the pull request yourself.
- **Fork pull request workflows:** "Require approval for all external contributors".
- **Require actions to be pinned to a full-length commit SHA:** on. Every action in the workflows
  is pinned.

If you protect `main` with a ruleset, block force pushes and deletions, but don't require pull
requests: the Live workflow commits data straight to `main`.

### 2. Cloudflare

Create an API token at **dash.cloudflare.com → My Profile → API Tokens → Create Token** from the
**Edit Cloudflare Workers** template, limited to your account. Note your **Account ID** (Workers &
Pages overview, right-hand side).
If the account has never used Workers, open **Workers & Pages** once and pick a `workers.dev`
subdomain; the page will live at `https://fr2027.<subdomain>.workers.dev`. A custom domain can be
added to the Worker later.

### 3. Secrets and variables

In **Settings → Secrets and variables → Actions**:

| Name                    | Kind     | Value                                                                                             |
| ----------------------- | -------- | ------------------------------------------------------------------------------------------------- |
| `CLOUDFLARE_API_TOKEN`  | secret   | The token from step 2                                                                             |
| `CLOUDFLARE_ACCOUNT_ID` | secret   | Your account ID                                                                                   |
| `INBOX_BOT_TOKEN`       | secret   | A long random string, e.g. from `openssl rand -hex 32`. Grok Bot gets the same value              |
| `INBOX_PIPELINE_TOKEN`  | secret   | Another long random string. Only GitHub Actions uses it                                           |
| `FR2027_INBOX_URL`      | variable | The Worker's address, `https://fr2027.<your-subdomain>.workers.dev` (printed by the first deploy) |

### 4. First run

Schedules only run from the default branch, so the pipeline starts once this code is on `main`.

1. Merge the pull request that adds the Live workflow.
2. **Actions → Live → Run workflow** with `attention_start` set to `2026-01-01`. It collects
   everything, backfills Wikipedia page views since January, deploys, and prints the Worker's
   address in the "Publish the site" step.
3. Set the `FR2027_INBOX_URL` variable to that address, then run Live once more with
   **sync_worker_secrets** ticked. That copies the two inbox tokens to the Worker.
4. Set up Grok Bot: [grok-bot.md](grok-bot.md).

From then on it runs every hour.

## Keeping an eye on it

- The **Sources** page of the site shows when each source last updated, and warns when markets or
  news are more than 3 hours old, page views more than 50 hours, X data more than 48 hours, or the
  forecast more than 36 hours.
- GitHub turns off scheduled workflows in a public repository after 60 days without activity. The
  hourly data commits should count as activity, but GitHub doesn't say so. If the Sources page
  goes stale, re-enable the workflow under **Actions → Live**.
- A failed run sends GitHub's usual email to the repository owner.
- `ubuntu-24.04` is pinned, so the move of `ubuntu-latest` to Ubuntu 26 (from 19 October 2026)
  doesn't change anything here.

## Publication blackout

French law bans publishing or commenting on election polls on the day before and the day of each
round (loi n° 77-808 du 19 juillet 1977, article 11). The page is public, so during the windows in
`config/sources.yaml` (`publication_blackouts`) `fr2027 export` refuses to build it and the Live
workflow keeps the last version online. Collection, the model and the data commits carry on; the
repository itself stays public during those windows, so if that matters, also pause the Live
workflow then.

## Running it by hand

Every step is a moon task (see the README): `moon run cli:collect-markets`,
`cli:collect-news`, `cli:collect-attention`, `cli:forecast`, `cli:export`, `worker:deploy`.
`fr2027 inbox pull` and `fr2027 inbox resolve` need `FR2027_INBOX_URL` and
`INBOX_PIPELINE_TOKEN` in the environment.
