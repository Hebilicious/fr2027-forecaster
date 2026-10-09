# Research log

Dated entries for every assumption, parameter change, source added or dropped, and the reason. Newest first within a day.

## 2026-10-09

### Implementation language: Rust for data and model, TypeScript for the UI

The first scaffold used Python because the spec named PyMC or NumPyro for the poll aggregation. That was the only hard reason for Python, and the model the spec describes does not need it:

- **The aggregation is linear-Gaussian.** Written as log-ratios (each bloc's total against the largest bloc, each candidate against the largest candidate of their bloc), every number a poll reports is a linear function of the latent state, with a multinomial sampling covariance from the delta method. A Kalman filter fits that exactly and gives the marginal likelihood used to choose the hyperparameters. No MCMC is needed, which is also what "prefer simple, auditable models" asks for. If a later milestone does need a sampler, `nuts-rs` (the engine behind nutpie) is Rust, and Stan can run through CmdStan from any language.
- **Rust** covers the rest of the headless work well: `nalgebra` for the linear algebra, serde and `jsonschema` for validated data boundaries, `axum` for the local API. Collectors, model and server ship as one binary (`fr2027`), which keeps "one command on a laptop" literal. The model owns its random number generator and samplers (xoshiro256++, polar normals, Marsaglia–Tsang gamma), so a seeded run reproduces exactly across dependency upgrades.
- **TypeScript** is needed for the browser UI anyway: Vue 3 with Vite. Payloads are decoded with Effect Schema at the API boundary, Vue files are checked with vize, and TypeScript is linted with oxlint and formatted with oxfmt.
- **proto** pins every tool (`.prototools`: moon, node, pnpm, rust) and **moon** defines every operation (`moon.yml` per project). There are no `package.json` scripts and no Makefile, so the spec's `make up` is `moon run repo:up`.
- **What Python would have given and we lose:** pandas convenience, and `pytrends` for Google Trends. `pytrends` is an unofficial scraper that breaks often. Milestone 5 will use the official Trends API if access is available, or drop the signal; it carries little weight either way.

### Deviations from the spec, and why

- **Clean data is CSV, not Parquet.** Volumes are small (polls are about 1,000 rows a year), and CSV diffs are readable in pull requests, so every number stays traceable to a dated row in Git. If markets or news outgrow this, partition them by month before switching format.
- **Poll entry format.** Each poll is one YAML file, `data/raw/polls/<poll_id>.yaml` (schema: `schemas/poll.schema.json`), entered by hand or by a collector. It is never edited after merge: a correction is a new file with `supersedes: <poll_id>`. Each file records `entry: primary` when its numbers were checked against the pollster's own publication, or `entry: index` when they were copied from a poll index (the Wikipedia tables). The UI shows which.
- **Round 1 chart.** Poll dots are drawn in one neutral color, with a control to highlight one pollster, rather than one color per pollster. With four or more categorical colors in a scatter plot, some pairs can't be told apart under color-vision deficiency.
- **Candidate colors.** Every candidate is drawn in the same color, and identity comes from labels and panel titles. This is the strictest reading of "identical treatment and color logic for every candidate".

### Model 0.1.0 (polls only, milestone 2)

**Field.** Each candidate belongs to one *slot* in `config/candidates.yaml`. A slot draws at most one of its options, and an option lists the candidates who run together. This expresses "RN fields exactly one of Le Pen or Bardella", "LR picks one nominee" and joint outcomes of a left-wing union. Slots are drawn independently.

**Round 1 aggregation.** The latent state on the log scale holds:

- each bloc's strength, following a random walk;
- each candidate's support within their bloc, following a random walk;
- each candidate's *presence effect*, how much their being on the ballot adds to their bloc's total (static);
- each pollster's house effect on each bloc (static).

A candidate who drops out passes support to the rest of their bloc first. The presence effects are learned from scenario polls that include and exclude the candidate, which is the spec's "transfer matrix estimated from scenario polls, falling back on ideological proximity", with the bloc as the proximity.

Assumptions:

- Scenarios from one poll share its respondents, so each scenario's sampling covariance is multiplied by the number of scenarios in the poll. One poll counts once, however many ballots it tests.
- Method effects are omitted: almost every French voting-intention poll is now an online quota sample. Revisit if phone or mixed-mode polls appear.
- The daily random-walk standard deviation and the design effect are chosen per run by marginal likelihood over a grid (`config/model.yaml`).
- Priors: level 2.0 (diffuse), presence effect 0.3, house effect 0.08 on the log scale.

**Round 1 error.** On election day, each bloc's strength and each candidate's within-bloc support get a Student-t error (5 degrees of freedom). The standard deviations, 0.12 for blocs and 0.10 for candidates, are **provisional** until milestone 3 fits them on 2002–2022 final-poll misses. The random-walk projection from the last poll to election day already widens the intervals as distance grows.

**Round 2.** A matchup with head-to-head polls is aggregated with a local-level filter on the logit of one finalist's share, then projected to 2 May 2027. Every matchup also gets a transfer estimate from the simulated round 1 vote, using the bloc transfer table in `config/model.yaml`. The two are combined by precision; the transfer estimate's standard deviation is 0.35 on the logit scale, so polls dominate where they exist.

**Transfer table.** For a centre-versus-far-right runoff, the rows reproduce the 2022 election-day surveys of Ipsos, Ifop and Elabe (`research/sources/round2-transfers-2017-2022.csv`), which agree with each other within a few points:

| First-round electorate | Survey | To Macron | To Le Pen | Abstain or blank |
| --- | --- | --- | --- | --- |
| Mélenchon | 2022 Ipsos / Ifop / Elabe | 42 / 42 / 38 | 17 / 13 / 18 | 41 / 45 / 44 |
| Pécresse | 2022 Ipsos / Ifop / Elabe | 53 / 52 / 51 | 18 / 18 / 22 | 29 / 30 / 27 |
| Zemmour | 2022 Ipsos / Ifop / Elabe | 10 / 9 / 11 | 73 / 78 / 80 | 17 / 13 / 9 |

The `left` bloc row is weighted toward Mélenchon's electorate, the largest in it. OpinionWay records much less abstention (Mélenchon's voters: 54 / 24 / 22) and was not used for the level: its base differs. Finalist loyalty is 0.95 (Macron 95–98, Le Pen 90–94 across the three surveys).

Every other cell is a proximity assumption, with no survey behind it: the `far_left` and `other` rows, and the columns for left, right and far-left finalists. Head-to-head polls outweigh those cells wherever they exist. Milestone 6 (programs) is where they get evidence.

**Not yet in the model:** the fundamentals prior, the market blend, and news or social signals. Those layers come in later milestones and must earn their weight in backtests.

### Sources added

- `research/sources/round2-transfers-2017-2022.csv`: second-round transfer surveys, 2017 and 2022. Each row has its source URL and base; see `research/sources/README.md`.
- `research/sources/results-2017-2022.csv`: official results for 2017 and 2022, checked against the Conseil constitutionnel decisions.
