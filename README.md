# fr2027-forecaster

Probabilistic forecaster for the 2027 French presidential election (round 1: 18 April 2027, round 2: 2 May 2027).

Polls are the backbone. Prediction markets, X activity (measured by Grok Bot), news, Wikipedia attention and candidate programs are secondary layers that must earn their weight in backtests; for now they are collected and shown, not used by the model. See [SPEC.md](SPEC.md) for the build spec and milestones, and [research/LOG.md](research/LOG.md) for every decision, source and parameter change.

It runs itself: a GitHub Actions workflow collects every hour, refits when polls or config change, commits the data, and publishes a public page on Cloudflare. Grok Bot routines send X measurements, new polls and campaign events to an inbox the workflow checks. Setup and operations: [docs/operations.md](docs/operations.md); the Grok Bot routines: [docs/grok-bot.md](docs/grok-bot.md).

## Run it

[proto](https://moonrepo.dev/proto) pins every tool in `.prototools` (moon, Node, pnpm, Rust), and [moon](https://moonrepo.dev/moon) runs every task. With proto installed:

```sh
proto install          # moon, node, pnpm, rust at the pinned versions
moon run repo:up       # ingest, forecast, build the UI, serve http://127.0.0.1:8027
```

Other tasks:

| Task                                                                        | What it does                                                                    |
| --------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| `moon run cli:ingest`                                                       | Validate `data/raw/`, rebuild `data/clean/`, quarantine rejected files          |
| `moon run cli:forecast`                                                     | Ingest, fit the model, write `data/forecasts/<YYYY-MM-DDTHH>.json`              |
| `moon run cli:validate`                                                     | Check config and every raw file; fails on rejections or stale clean data        |
| `moon run cli:collect-markets`, `cli:collect-news`, `cli:collect-attention` | Read one live source into a new file under `data/raw/`                          |
| `moon run cli:export`                                                       | Build the public site into `.site/`: the UI plus `api/*.json`                   |
| `moon run worker:deploy`                                                    | Export, then deploy the site and the inbox to Cloudflare                        |
| `moon run web:dev`                                                          | Vite dev server for the UI (proxies `/api` to a running `fr2027 serve`)         |
| `moon ci`                                                                   | Everything CI runs: tests, clippy, type checks, lint, format checks, validation |
| `moon run repo:format`                                                      | Format Rust, TypeScript, JSON, Markdown and Vue                                 |

The binary also runs on its own: `target/release/fr2027 forecast --as-of 2026-09-01` replays the forecast with only the polls published by that date.

## How it is built

| Path              | Language        | Role                                                                                                          |
| ----------------- | --------------- | ------------------------------------------------------------------------------------------------------------- |
| `model/`          | Rust            | The model, pure computation: field draws, bloc-nested Kalman poll aggregation, round 2, two-round Monte Carlo |
| `collectors/`     | Rust            | Validates raw files against `schemas/`, writes `data/clean/`, quarantines rejects                             |
| `fetch/`          | Rust            | Network readers: Polymarket, Kalshi, RSS feeds, Wikipedia page views, the Grok Bot inbox                      |
| `cli/`            | Rust            | The `fr2027` binary: `validate`, `ingest`, `forecast`, `collect`, `inbox`, `export`, `serve`                  |
| `app/web/`        | TypeScript, Vue | The UI: Overview, Round 1, Round 2, Signals, Sources and health; French and English                           |
| `app/worker/`     | TypeScript      | The Cloudflare Worker: serves the public site and the Grok Bot inbox                                          |
| `config/`         | YAML            | `candidates.yaml` (canonical ids, blocs, who runs), `model.yaml` (parameters), `sources.yaml` (live sources)  |
| `.github/`        | YAML            | `ci.yml` (checks) and `live.yml` (the hourly pipeline)                                                        |
| `docs/`           |                 | Operations and the Grok Bot routines                                                                          |
| `schemas/`        | JSON Schema     | Contracts for every file sources write and the model outputs                                                  |
| `data/raw/`       |                 | Source files, never edited: `polls/`, `markets/`, `news/`, `attention/`, `grok/`, `events/`                   |
| `data/clean/`     | CSV             | Rebuilt from raw on every ingest                                                                              |
| `data/forecasts/` | JSON            | One file per run, plus `series.json` (latent share history)                                                   |
| `research/`       |                 | `LOG.md` and the sourced reference data in `sources/`                                                         |

Why Rust and TypeScript rather than Python: see [research/LOG.md](research/LOG.md), entry "Implementation language".

## Adding a poll

Create `data/raw/polls/<poll_id>.yaml` following `schemas/poll.schema.json` (copy a recent file), using candidate ids from `config/candidates.yaml`. Set `entry: primary` only if you checked the numbers against the pollster's own publication or its Commission des sondages notice. Then run `moon run cli:forecast`. A file that fails validation is copied to `data/quarantine/polls/` with a note saying why, and the rest still flow through.

## Status

| Milestone                     | State                                                                                                               |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| 1. Skeleton                   | Done: layout, schemas, config, CI, UI                                                                               |
| 2. Polls and polls-only model | Done: 28 polls (April–September 2026) checked against their notices; real forecast in Overview, Round 1 and Round 2 |
| 3. Backtest harness           | Next: 2017 and 2022 replays, which fit the provisional error parameters                                             |
| 4. Markets                    | Collector and comparison view done; the blend waits for the backtests                                               |
| 5. Grok, news, attention      | Inbox, collectors and Signals view done; event detection and ablations wait for the backtests                       |
| 6. Programs                   | Not started                                                                                                         |
| 7. Ops hardening              | Hourly schedule, public page and health view done; needs 7 unattended days                                          |
