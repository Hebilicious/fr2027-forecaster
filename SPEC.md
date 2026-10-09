# French 2027 Presidential Forecaster — Build Spec

As of 2026-10-09 · Owner: @Hebilicious · Living version: https://claude.ai/code/artifact/5b77f2f2-9f89-452d-b845-c8d5a49b1b3a

## Purpose and scope

Build a local, self-updating forecaster that outputs calibrated win probabilities for every plausible 2027 French presidential candidate, for round 1 qualification and round 2 victory. Polls are the backbone; prediction markets, X trends, news and candidate programs are secondary layers whose weight is earned through backtesting, not assumed.

**Deliverables, all in this repo:**

1. Data collectors for each source, writing versioned raw and clean data into the repo.
2. A probabilistic model (two-round Monte Carlo) with documented assumptions.
3. A local web UI showing current probabilities, their history, and what moved them.
4. A research log: sources consulted, decisions made, and why.
5. Backtests on 2017 and 2022 with calibration results.

**Success criteria:**

- Calibration: in backtests, events forecast at ~70% happen roughly 70% of the time (Brier score and reliability plot reported).
- Beats a polls-only baseline and the Polymarket price on log-loss in backtests, or the extra layers get their weight cut to zero.
- Every number in the UI traces back to a dated source row.
- Runs end to end on a laptop with one command.

**Out of scope:** telling anyone how to vote, scraping private data about individuals, automated trading on Polymarket.

**Instructions to the implementing session:** Work milestone by milestone (see last section). Verify every current fact (declared candidates, polling firms, API endpoints, legal rules) with a fresh search before coding against it; this spec reflects knowledge as of the date above. When the spec and reality disagree, follow reality and record the change in the research log. Prefer simple, auditable models over clever ones.

## Architecture

The repo is the single meeting point: every source, including the Grok bot, writes files into it, and the model and UI only ever read from it.

```
 Polls      Polymarket     Grok drops        News + Trends   Programs
 (firms,    (Gamma/CLOB    (data/raw/grok)   (RSS, GDELT)    (+ past results)
  CSV)       API)
   |            |               |                 |               |
   +------------+-------+-------+-----------------+---------------+
                        v
          Collectors + schema validation
          raw kept unchanged, clean rows dated, invalid -> quarantine
                        |
                        v
          Model: two-round Monte Carlo            <---  Weekly review
          field draw, R1 aggregation, transfers,        (Claude session updates
          R2; news/social, market blend, prior           candidates + research log)
                        |
                        v
          data/forecasts (hourly JSON)  ---->  Local UI (make up)
```

Collectors run on a schedule and never feed the model directly; the weekly Claude review edits configuration through pull requests, so every change to the field of candidates is reviewable.

## Data sources

Polls carry the forecast; every other source is a challenger that must prove its value in backtests. Election dates are fixed: round 1 on 18 April 2027, round 2 on 2 May 2027 ([service-public.gouv.fr](https://www.service-public.gouv.fr/particuliers/actualites/A15053?lang=fr)).

| Source | What to collect | Access | Refresh | Role in model |
| --- | --- | --- | --- | --- |
| Voting-intention polls | Every published R1 and R2 poll: firm, fieldwork dates, sample, method, scenario (which candidates were tested), toplines | Pollster sites (Ifop, Elabe, Odoxa, OpinionWay, Ipsos, Harris/Toluna, Cluster17, BVA/Verian), Commission des sondages notices, Wikipedia poll tables as an index only | Daily | Primary signal |
| Polymarket | Price and volume per candidate in the "Next French Presidential Election" event, full price history | Public Gamma API (event/market metadata) and CLOB price-history endpoint; verify current endpoints | Hourly | Benchmark and optional blend input |
| X trends (Grok bot) | Mention volume, sentiment, engagement, top topics per candidate | Grok bot writes JSON files into `data/raw/grok/` (contract below) | Whenever the bot pushes | Low-weight momentum signal |
| News | Article counts and headlines per candidate from French national outlets | RSS feeds (Le Monde, Le Figaro, Libération, France Info, BFMTV, Les Échos, Ouest-France) plus GDELT | Hourly | Event detection, uncertainty widening |
| Search interest | Google Trends per candidate, France | pytrends or the official Trends API if available | Daily | Low-weight momentum signal |
| Candidate programs | Declared program, or the previous program and recent statements | Official campaign sites, party platforms, 2022 programs | On release | Transfer modeling (see Programs) |
| Fundamentals | Approval ratings, unemployment, inflation, consumer confidence | INSEE, Banque de France, approval barometers (Ifop-JDD, Elabe, Odoxa) | Monthly | Priors far from election day |
| Historical results | 2002–2022 presidential results R1 and R2, plus 2024 European and legislative results | data.gouv.fr (Ministère de l'Intérieur) | Once | Backtests, transfer matrices, error model |
| Official candidate list | Sponsorship (parrainages) counts and final list | Conseil constitutionnel publications | Daily from Jan 2027 | Field uncertainty |

**Collection rules:**

- Respect robots.txt and terms of use. Where a pollster forbids scraping, enter polls manually through a small CSV form; volume is ~10–30 polls a month.
- Store raw responses unchanged in `data/raw/`, parsed rows in `data/clean/`. Never overwrite history.
- Every row carries `source_url`, `retrieved_at` and a content hash.

## Data contracts

The Grok bot and the forecaster talk only through files in the repo, so the bot can be swapped for an MCP connection later without touching the model. All schemas live in `schemas/` as JSON Schema and are validated on ingest; invalid files go to `data/quarantine/` with an error note.

**Grok drop (`data/raw/grok/YYYY-MM-DDTHH-MM.json`):**

```json
{
  "schema_version": "1.0",
  "collected_at": "2026-10-09T14:00:00Z",
  "window": {"start": "2026-10-09T13:00:00Z", "end": "2026-10-09T14:00:00Z"},
  "query_method": "free text: how the bot searched",
  "candidates": [
    {
      "candidate_id": "bardella",
      "mentions": 18250,
      "unique_authors": 9100,
      "engagement": 412000,
      "sentiment": {"pos": 0.31, "neg": 0.44, "neu": 0.25, "method": "grok-classifier"},
      "top_topics": ["immigration", "pouvoir d'achat"],
      "bot_share_estimate": 0.12,
      "sample_post_ids": ["..."]
    }
  ]
}
```

Rules for the drop: one file per window, never edit a pushed file, commit with message `grok: <window end>`. The bot must use the canonical `candidate_id` list in `config/candidates.yaml`. If the bot can't fill a field, it sends `null`, not a guess.

**Poll row (`data/clean/polls.parquet`):** `poll_id, firm, sponsor, field_start, field_end, published_at, sample_size, method (online/phone/mixed), population (registered/likely), round (1/2), scenario_id, candidate_id, share, source_url, notice_url`. One row per candidate per scenario; a scenario is the exact set of candidates tested.

**Market row (`data/clean/markets.parquet`):** `ts, candidate_id, market_id, mid_price, best_bid, best_ask, volume_24h, liquidity`.

**News row (`data/clean/news.parquet`):** `ts, outlet, url, title, candidate_ids[], event_tags[]`. Store titles and links only, not article bodies.

**Forecast output (`data/forecasts/YYYY-MM-DDTHH.json`):** per candidate: `p_run`, `p_qualify_r1`, `p_win`, R1 vote share mean and 80% interval; per pair: `p_matchup`, `p_win_given_matchup`; plus `model_version`, `inputs_hash`, and the top drivers of change since the previous run.

## Modeling

The forecast is a Monte Carlo over 50,000 simulated elections, each one drawing who runs, the round 1 vote, who qualifies, and the round 2 result. Every layer below is a separate, testable module with its own parameters file.

1. **Field model.** Each potential candidate gets `p_run` (declared, rumored, needs sponsorships, likely to withdraw for an ally). Set from declarations, sponsorship counts and Polymarket where informative, and updated by hand-reviewed rules logged in the research log. Each simulation first draws a field.
2. **Poll aggregation (round 1).** Bayesian state-space model of each candidate's latent share over time (random walk, logit or log-ratio scale so shares sum to 100%). Polls enter with sampling variance from sample size, plus a firm house effect and a method effect, both estimated. Scenario polls that test different fields are all used: the model works with latent support, and a scenario's shares are the latent shares renormalized over who was tested.
3. **Missing-candidate redistribution.** When a simulated field differs from what pollsters tested, a dropped candidate's support is reallocated using a transfer matrix estimated from scenario polls that include and exclude them, falling back on ideological proximity (see Programs).
4. **Round 1 error model.** Add a national polling error drawn from a fat-tailed distribution (Student-t), fit to 2002–2022 final-poll misses. Errors are correlated across candidates of the same bloc. Error variance shrinks as election day approaches, following the historical curve.
5. **Round 2.** For matchups with direct polls, aggregate them as in step 2. For untested matchups, build the vote from round 1 shares times a transfer matrix (R1 vote → R2 choice or abstention), estimated from 2017/2022 post-election surveys and current second-round polls. Add its own correlated error.
6. **News and social layer.** Not a vote-share predictor by default. It does two things: (a) event detection, where a spike in news or X volume for a candidate flags a possible shock, temporarily widening that candidate's latent-share variance; (b) an optional short-horizon momentum term with weight fit in backtests, decaying with a half-life estimated from past shocks (start with ~10 days). If backtests show no gain, weight = 0 and the UI still displays the signals.
7. **Market blend.** Report three numbers side by side: model-only, Polymarket-implied (normalized, fees and long-shot bias noted), and a blend. Blend weight is fit on log-loss in backtests where market history exists; otherwise default to model-only and show the market as a comparison.
8. **Fundamentals prior.** Far from election day, shrink round 1 shares toward a prior built from bloc strength in 2022 and 2024 results, adjusted by government approval. Prior weight decays to near zero by March 2027.

```
p_blend = softmax( w * log p_model + (1 - w) * log p_market )
```

**Implementation:** Python 3.12, PyMC or NumPyro for the state-space model (refit nightly), NumPy for simulation (runs hourly from posterior draws). Seed every run; store posterior summaries, not full traces.

**Explainability:** each run computes a change attribution: how much each new poll, market move or flagged event shifted `p_win` since the last run, by re-running with that input removed.

## Candidate programs

Programs feed the model through vote transfers, not through a "good program" score. A voter whose candidate is out tends to move to the closest remaining candidate, so positions sharpen the transfer matrices in steps 3 and 5.

1. **Source hierarchy per candidate:** official 2027 program → 2022 program (or party platform) → recent public statements, each tagged with date and URL. Programs are generally published only weeks or months before round 1, so expect the fallback most of the time.
2. **Position coding:** score each candidate on ~12 issue axes (e.g. immigration, retirement age, EU integration, purchasing power and taxes, security, energy and nuclear, ecology, public spending, Ukraine and defense, institutions, secularism, wealth taxation) on a −2 to +2 scale. Use LLM-assisted extraction with the quoted passage stored beside each score; a human spot-checks every score before it's used.
3. **Issue salience:** weight axes by what voters say matters most, from regular "most important issue" survey questions (e.g. Ipsos Fractures françaises, Elabe barometers).
4. **Use in the model:** salience-weighted distance between candidates becomes the prior for transfer matrices, updated by actual scenario and round 2 polls where available.
5. **UI:** a comparison table of positions with source quotes, kept strictly descriptive.

Expected effect is modest. Backtests must show whether program-based transfers beat a simple bloc-based transfer prior; if not, keep the simpler one.

## Backtesting and calibration

No layer ships with nonzero weight unless it improves out-of-sample log-loss on past French elections. Backtests replay history as of each date, using only data published before that date.

- **Elections:** 2017 and 2022 presidential as primary tests (both had rich polling and, for 2022, some market data); 2012 and 2007 for the poll model and error curve only.
- **Replay dates:** monthly from 12 months out, weekly in the final 3 months.
- **Metrics:** Brier score and log-loss for R1 qualification and final winner; MAE of R1 shares; 80% interval coverage (target 75–85%).
- **Baselines to beat:** latest-poll average; polls-only model without extra layers; market price where available.
- **Ablations:** report the metric delta of each layer (news, social, Google Trends, programs, fundamentals, market blend).
- **Leakage checks:** assert no row with `published_at` after the replay date reaches the model.

Two past elections are a thin sample, so treat backtest weights as rough and prefer conservative settings. Historical X data is limited; the social layer will mostly be validated forward, by logging its predictions now and scoring them as polls arrive.

## Local UI

A single local web app, started with `make up` (or `docker compose up`), reading only from `data/forecasts/` and `data/clean/`. Suggested stack: FastAPI backend + a lightweight frontend (React + Vite, or Streamlit for a faster first version). Auto-refreshes when a new forecast file lands.

| View | Shows |
| --- | --- |
| Overview | `p_win` and `p_qualify_r1` per candidate, with model, market and blend side by side; last updated time; days to round 1 |
| History | `p_win` over time per candidate, with event markers (flagged news spikes, major polls, declarations) |
| Round 1 | Latent vote share per candidate with 80% bands, individual polls as dots, colored by firm |
| Round 2 | Matrix of matchups: probability each pairing happens and who wins it |
| What moved | Change attribution since the previous run and over 7 days |
| Signals | X volume and sentiment, news volume, Google Trends per candidate, with the weight each currently carries |
| Programs | Issue-position table with source quotes |
| Sources and health | Last successful run per collector, quarantined files, data freshness warnings |

Every chart point links to its source row. Language toggle FR/EN. Candidates are shown in neutral, consistent colors not tied to party branding.

## Scheduling, ops and research log

Collectors and the model run on a schedule inside the repo, so the forecast stays live without any agent watching it.

| Job | Frequency | Runs where |
| --- | --- | --- |
| Polymarket, news RSS | Hourly | Local scheduler (APScheduler in the app, or cron) and/or a GitHub Actions workflow |
| Polls, Google Trends | Daily | Same |
| Ingest Grok drops | On each push to `data/raw/grok/` | GitHub Actions on push, plus local poll of the folder |
| Model refit | Nightly | Local or Actions (budget ~10 min) |
| Simulation + forecast file | Hourly, and after any refit | Local or Actions |
| Research review by a Claude session | Weekly | Scheduled task: check new declarations, programs, alliances; update `config/candidates.yaml` and the research log via pull request |

**Research log (`research/LOG.md`):** dated entries for every assumption, parameter change, source added or dropped, and reason. Model changes go through pull requests with a backtest diff in the description.

**Repo layout:** `collectors/`, `model/`, `backtest/`, `app/`, `config/`, `schemas/`, `data/{raw,clean,forecasts,quarantine}/`, `research/`, `tests/`. Large raw files go to Git LFS or are pruned to summaries after 30 days.

**Tests:** schema validation, collector parsers against saved fixtures, a simulation sanity test (probabilities sum to 1, deterministic with seed), and the leakage test from Backtesting.

## Guardrails

The tool forecasts; it never advocates. These rules are requirements, not suggestions.

- **Neutrality:** no persuasive text, no ranking of programs as better or worse, identical treatment and color logic for every candidate.
- **French publication law:** the law on election polls (loi du 19 juillet 1977, as amended) bans publishing polls on the eve and day of the vote, and regulates how polls are presented. The app is for private use; if any output is ever shared publicly, freeze public outputs during the blackout and have the rules checked by someone qualified. Verify current rules before launch.
- **Polymarket in France:** French regulators restricted Polymarket access for French users in late 2024. Reading public price data is a separate question from trading; check the platform's terms and current status before relying on it. No trading code in this repo.
- **Privacy:** aggregate counts and public post IDs only; no profiles of private individuals, no storing personal data beyond what's needed for de-duplication.
- **Honest uncertainty:** the UI always shows intervals and a short note that a 70% favorite loses about 3 times in 10.
- **Secrets:** API keys in `.env`, never committed; the Grok bot gets a scoped token that can only write to `data/raw/grok/`.

## Milestones and acceptance criteria

Build in this order; each milestone ends with a working, tested state committed to `main`.

1. **Skeleton.** Repo layout, schemas, `config/candidates.yaml`, CI running tests, empty UI that reads a dummy forecast file. *Done when:* `make up` shows the dummy forecast.
2. **Polls + polls-only model.** Poll collector or manual CSV entry, state-space aggregation, R1/R2 Monte Carlo with field uncertainty. *Done when:* a real forecast file is produced and shown in Overview and Round 1.
3. **Backtest harness.** 2017 and 2022 replays, baselines, calibration report. *Done when:* the polls-only model's report is committed to `research/`.
4. **Polymarket.** Collector, comparison view, blend fit. *Done when:* model vs market vs blend shown, with blend weight justified in the log.
5. **Grok ingest + news + Trends.** Drop-folder ingest with validation, RSS/GDELT collector, event detection, Signals view. *Done when:* a sample Grok file flows through to the UI, and ablation results are logged.
6. **Programs.** Position coding with quotes, transfer priors, Programs view. *Done when:* backtest shows the effect of program-based vs bloc-based transfers.
7. **Ops hardening.** Schedules, health view, What-moved attribution, weekly research task. *Done when:* the app updates for 7 days unattended with no stale-data warnings.

**Open questions for the owner:**

- [ ] Which Grok bot fields are realistic to collect (sentiment, bot share)? Adjust the contract before milestone 5.
- [x] Should the GitHub repo be private? Yes — the repo is private.
- [ ] Run schedules locally, on GitHub Actions, or both?
- [ ] Is the output ever going to be shared publicly? This changes the legal checks.
