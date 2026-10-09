# fr2027-forecaster

Probabilistic forecaster for the 2027 French presidential election (round 1: 18 April 2027, round 2: 2 May 2027).

Polls are the backbone. Polymarket, X trends (via a Grok bot dropping files into `data/raw/grok/`), news and candidate programs are secondary layers that must earn their weight in backtests. See [SPEC.md](SPEC.md) for the build spec and milestones, and [research/LOG.md](research/LOG.md) for every decision, source and parameter change.

## Run it

[proto](https://moonrepo.dev/proto) pins every tool in `.prototools` (moon, Node, pnpm, Rust), and [moon](https://moonrepo.dev/moon) runs every task. With proto installed:

```sh
proto install          # moon, node, pnpm, rust at the pinned versions
moon run repo:up       # ingest, forecast, build the UI, serve http://127.0.0.1:8027
```

Other tasks:

| Task                    | What it does                                                                    |
| ----------------------- | ------------------------------------------------------------------------------- |
| `moon run cli:ingest`   | Validate `data/raw/`, rebuild `data/clean/`, quarantine rejected files          |
| `moon run cli:forecast` | Ingest, fit the model, write `data/forecasts/<YYYY-MM-DDTHH>.json`              |
| `moon run cli:validate` | Check config and every raw file; fails on rejections or stale clean data        |
| `moon run web:dev`      | Vite dev server for the UI (proxies `/api` to a running `fr2027 serve`)         |
| `moon ci`               | Everything CI runs: tests, clippy, type checks, lint, format checks, validation |
| `moon run repo:format`  | Format Rust, TypeScript, JSON, Markdown and Vue                                 |

The binary also runs on its own: `target/release/fr2027 forecast --as-of 2026-09-01` replays the forecast with only the polls published by that date.

## How it is built

| Path              | Language        | Role                                                                                                          |
| ----------------- | --------------- | ------------------------------------------------------------------------------------------------------------- |
| `model/`          | Rust            | The model, pure computation: field draws, bloc-nested Kalman poll aggregation, round 2, two-round Monte Carlo |
| `collectors/`     | Rust            | Validates raw files against `schemas/`, writes `data/clean/`, quarantines rejects                             |
| `cli/`            | Rust            | The `fr2027` binary: `validate`, `ingest`, `forecast`, `serve` (JSON API and the built UI)                    |
| `app/web/`        | TypeScript, Vue | The local UI: Overview, Round 1, Round 2, Sources and health; French and English                              |
| `config/`         | YAML            | `candidates.yaml` (canonical ids, blocs, who runs) and `model.yaml` (parameters)                              |
| `schemas/`        | JSON Schema     | Contracts for every file sources write and the model outputs                                                  |
| `data/raw/`       |                 | Source files, never edited: `polls/<poll_id>.yaml`, `grok/<window>.json`                                      |
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
| 4–7                           | Not started                                                                                                         |
