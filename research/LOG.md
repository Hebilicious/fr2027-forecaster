# Research log

Dated entries for every assumption, parameter change, source added or dropped, and the reason. Newest first within a day.

## 2026-10-16

### Data: polls from 24 September to 14 October 2026

Five polls added, bringing `data/raw/polls/` to 33 polls: 15 round 1 scenarios and 6 head-to-heads. Indexed from the French and English Wikipedia tables (raw wikitext fetched 2026-10-16 about 06:50 UTC), then every number of every scenario compared with the notice or the pollster's report.

| File                              | Sponsor                                            | Fieldwork      | Checked against                                     | Entry   | R1 / R2 |
| --------------------------------- | -------------------------------------------------- | -------------- | --------------------------------------------------- | ------- | ------- |
| `odoxa-2026-10-07-10310`          | Backbone, Public Sénat, regional press, Nouvel Obs | 24 Sep – 7 Oct | Odoxa's full report (the notice has no results)     | primary | 2 / 2   |
| `toluna-harris-2026-10-08-10305`  | M6, RTL                                            | 5–8 Oct        | notice 10305                                        | primary | 5 / 2   |
| `ifop-2026-10-14-10314`           | LCI, Sud Radio                                     | 12–14 Oct      | notice 10314                                        | primary | 4 / 0   |
| `opinionway-2026-10-14-les-echos` | Les Echos, Radio Classique (Présitrack wave 3)     | 13–14 Oct      | OpinionWay's report; no notice yet                  | primary | 3 / 0   |
| `opinionway-2026-10-14-cnews`     | CNews                                              | 13–14 Oct      | OpinionWay's chart as printed by the JDD; no notice | index   | 1 / 3   |

- **Indexes vs sources.** Both indexes match the Harris, Ifop, Odoxa and Présitrack sources exactly. For the OpinionWay/CNews poll, the English index's only round 1 row differs from OpinionWay's chart on Arthaud (0.5 vs 1), Roussel (2 vs 1) and Tondelier (1 vs 2); the file uses the chart.
- **No notice yet for either OpinionWay poll.** The Commission's list on 2026-10-16 ends at notice 10316. The two files are named `<firm>-<field_end>-<sponsor>` instead of a notice number, following the Odoxa May file, which has none. A later file can supersede each one once its notice is filed.
- **OpinionWay/CNews is `entry: index`.** No pollster publication was found; CNews and Europe 1 serve a bot challenge, which was not worked around. The JDD, which belongs to the same group as CNews, printed OpinionWay's round 1 chart and all three runoff figures (Le Pen 68 v Mélenchon, 56 v Philippe, 56 v Glucksmann); the runoffs agree with the French index. Its population base is not established (`population: null`).
- **Unpublished runoffs.** Harris asked six runoffs but published only Mélenchon and Philippe v Le Pen. Odoxa tested Glucksmann and Attal v Le Pen and did not publish them, because neither qualifies on its round 1 figures. Neither set is available, so neither is entered.
- **Conventions.** `published_at`: Odoxa's notice gives 14 Oct; Sud Radio published the Ifop poll on 15 Oct; both OpinionWay polls appeared on 15 Oct; Harris follows the bootstrap rule (deposit date, 12 Oct). Ifop prints Arthaud as "-" in two scenarios, entered as 0 (the model floors shares at 0.5 anyway). "<1%" is entered as 0.5. `sample_size` is the total sample.
- **Odoxa's sample is 12,005**, about 1,000 per region reweighted to national weights. The model's sampling variance is a design effect over sample size (3 in today's round 1 fit, 1.5 in round 2), so this poll counts for about ten ordinary ones. Whether its regional design deserves a larger design effect is a modelling question, not settled here.
- **No interested-party sponsors** among the five. Backbone, an Odoxa partner for this poll, is a social-media analytics firm, not a party or candidate.
- **Considered and left out** (no voting intention for the 2027 presidential election in the notice): Verian stature barometer (10297, which uses vote intention only as a breakdown variable), Harris electoral potential (10300, "ne constitue nullement une intention de vote"), Harris PS primary (10304), Ipsos BVA barometer (10303), YouGov popularity (10288), Cluster17 personalities (10296), CSA for CNews (10301, 10316), Ifop on Bardella's remarks (10299), Ifop for Franc-Tireur (10309), Ifop "peuple de droite" (10312), and notices 10271–10285 rechecked for late September. No other poll from 30 September to 14 October appears in either index or in the notice list.
- **Glucksmann won the PS-PP primary on 11 October** (66% in the first round, per the French index). `config/candidates.yaml` still holds the provisional primary slot; updating it is a config change for a separate entry.

## 2026-10-09

### Live pipeline: GitHub Actions, a public page on Cloudflare, and Grok Bot through an inbox

The owner made the repository public (so GitHub Actions runs free on standard runners) and asked for a public page.

- **Schedule.** One hourly workflow (`.github/workflows/live.yml`, at :17 to stay off the top of the hour) runs every deterministic collector, pulls the Grok Bot inbox, refits with `fr2027 forecast --if-changed`, validates, commits to `main` as `github-actions[bot]`, and deploys. The forecast's inputs hash covers polls, config and the as-of date, so the model refits when polls or config change and once a day, not every hour. A merge that changes polls, config or code triggers the same workflow.
- **Public page.** `fr2027 export` writes the built UI plus every document the UI reads as `api/<name>.json`; `fr2027 serve` computes the same documents per request, so the local app and the page can't drift. A Cloudflare Worker (`app/worker`) serves the files with Workers static assets.
- **Prediction markets** (`config/sources.yaml`): Polymarket (Gamma API) and Kalshi (public market data) for three questions, matching the model's three numbers: wins (`p_win`), reaches round 2 (`p_qualify_r1`), is on the ballot (`p_run`). Implied price is the bid–ask midpoint when the spread is at most 0.10, else the last trade. Contracts are matched to candidates by exact name, ignoring case and accents; the rest are kept with their label (Lisnard, Darmanin, Lagarde…). The clean file keeps every hourly snapshot of the last seven days and the last snapshot of each earlier day. The model does not read markets: the spec's blend needs the backtests first.
- **News.** 13 RSS feeds that answered on 2026-10-09: Le Monde (2), Le Figaro (2), Libération, franceinfo (2), BFMTV (2), Le Parisien, 20 Minutes, Public Sénat, France 24. Les Échos (403 to non-browsers) and Ouest-France (no political feed) are left out rather than worked around. Only the headline, link and time of items naming a candidate are kept, matched on title and summary with case-sensitive whole-word names, ignoring accents; surnames that are common words ("Le Maire", "Royal", "Philippe", "Faure", "Bertrand") need the full name. The first snapshot matched 159 items from 13 feeds; a read-through found no false positives.
- **Attention.** Daily user page views of each candidate's French Wikipedia article (Wikimedia API, agent=user), fetched once a day for the last week; a later fetch of a day replaces an earlier one. Wikimedia rate-limits this environment's shared egress (HTTP 429), so the first fetch happens in GitHub Actions.
- **Grok Bot.** Research on 2026-10-09 (docs.x.ai/grok-bot and Cursor's Grok Bot help pages):
  - Grok Bot is xAI's hosted agent product, separate from the Grok chat app and the xAI API: named Bots on a persistent cloud computer, with routines that run on a schedule (at least five minutes apart) or on events and webhooks, write-only secrets, and native read-only X search, including "count recent posts on a topic".
  - Its docs don't say whether those counts are complete or sampled. Each drop now records `mentions_method` and `sample_size` (new optional fields in the drop schema), and X numbers stay out of the model until checked against polls.
  - GitHub cannot limit a token to one folder of a public repository (path rules exist only for private repositories), and all Bots on an account share one computer. So the Bot gets no GitHub credential: it posts to an inbox on the Worker (a Durable Object), and the hourly workflow validates each item with the ingest code. X drops and events are committed; polls go into a pull request for a person to check against the notice; rejections go back to the Bot with reasons. The routines are in `docs/grok-bot.md`.
  - Post ids are dropped before commit, so a post deleted on X doesn't stay in a public repository.
- **Publication blackout.** The page is public, so `fr2027 export` refuses to build during the eve and day of each round (loi n° 77-808, art. 11; windows in `config/sources.yaml`). Collection carries on.
- **Wording.** The model was described as "polls only", but `p_run` in `config/candidates.yaml` partly follows Polymarket's ballot market. The UI now says that the model fits only polls, and that the chance each person runs is set by hand, partly from market prices.

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

**Field.** Each candidate belongs to one _slot_ in `config/candidates.yaml`. A slot draws at most one of its options, and an option lists the candidates who run together. This expresses "RN fields exactly one of Le Pen or Bardella", "LR picks one nominee" and joint outcomes of a left-wing union. Slots are drawn independently.

**Round 1 aggregation.** The latent state on the log scale holds:

- each bloc's strength, following a random walk;
- each candidate's support within their bloc, following a random walk;
- each candidate's _presence effect_, how much their being on the ballot adds to their bloc's total (static);
- each pollster's house effect on each bloc (static).

A candidate who drops out passes support to the rest of their bloc first. The presence effects are learned from scenario polls that include and exclude the candidate, which is the spec's "transfer matrix estimated from scenario polls, falling back on ideological proximity", with the bloc as the proximity.

Assumptions:

- **Scenarios of one poll ask the same respondents**, so all of a poll's scenarios enter the filter as one observation. Its sampling error has two parts. One is shared by every scenario, on the bloc and candidate levels, with variance `1 / (n p)` per level. The other is scenario-specific, a share `λ` of the multinomial covariance. A poll then counts once for the levels, while the contrasts between its scenarios, which identify the presence effects, keep their precision. A lone scenario gets exactly the multinomial covariance (unit-tested).
  - The first version divided each scenario's information by the number of scenarios instead. On the real polls the marginal likelihood then ran to the edge of its grid: the largest random walk and the smallest design effect. That throws away the within-sample contrasts, so it was replaced.
- Method effects are omitted: almost every French voting-intention poll is now an online quota sample. Revisit if phone or mixed-mode polls appear.
- The daily random-walk standard deviation, the design effect and `λ` are chosen per run by marginal likelihood over the grids in `config/model.yaml`. On the 28 polls of 2026-10-09 the choice is interior on all three: random walk 0.012 a day, design effect 3, `λ` 0.4.
- Priors on the log scale: levels 2.0 (diffuse), presence effects 0.3, house effects 0.08.

**Round 1 error.** On election day, each bloc's strength and each candidate's within-bloc support get a Student-t error (5 degrees of freedom). The standard deviations, 0.12 for blocs and 0.10 for candidates, are **provisional** until milestone 3 fits them. The random-walk projection from the last poll to election day already widens the intervals as distance grows.

A first check against `research/sources/round1-final-poll-errors-2002-2022.csv` supports their size. Over 34 candidate-elections from 2002 to 2022, final-week poll averages missed results by an RMS of 0.18 on the log scale, and by 0.12 for candidates polling 10% or more. The settings above give about 0.11 for a candidate at 30%. The largest misses are Mélenchon 2022 (−4.8 points against the polls), Jean-Marie Le Pen 2002 (−3.4) and Jean-Marie Le Pen 2007 (+3.3).

**Round 2.** A matchup with head-to-head polls is aggregated with a local-level filter on the logit of one finalist's share, then projected to 2 May 2027. Every matchup also gets a transfer estimate from the simulated round 1 vote, using the bloc transfer table in `config/model.yaml`. The two are combined by precision; the transfer estimate's standard deviation is 0.35 on the logit scale, so polls dominate where they exist.

**Transfer table.** For a centre-versus-far-right runoff, the rows reproduce the 2022 election-day surveys of Ipsos, Ifop and Elabe (`research/sources/round2-transfers-2017-2022.csv`), which agree with each other within a few points:

| First-round electorate | Survey                    | To Macron    | To Le Pen    | Abstain or blank |
| ---------------------- | ------------------------- | ------------ | ------------ | ---------------- |
| Mélenchon              | 2022 Ipsos / Ifop / Elabe | 42 / 42 / 38 | 17 / 13 / 18 | 41 / 45 / 44     |
| Pécresse               | 2022 Ipsos / Ifop / Elabe | 53 / 52 / 51 | 18 / 18 / 22 | 29 / 30 / 27     |
| Zemmour                | 2022 Ipsos / Ifop / Elabe | 10 / 9 / 11  | 73 / 78 / 80 | 17 / 13 / 9      |

The `left` bloc row is weighted toward Mélenchon's electorate, the largest in it. OpinionWay records much less abstention (Mélenchon's voters: 54 / 24 / 22) and was not used for the level: its base differs. Finalist loyalty is 0.95 (Macron 95–98, Le Pen 90–94 across the three surveys).

Every other cell is a proximity assumption, with no survey behind it: the `far_left` and `other` rows, and the columns for left, right and far-left finalists. Head-to-head polls outweigh those cells wherever they exist. Milestone 6 (programs) is where they get evidence.

**Not yet in the model:** the fundamentals prior, the market blend, and news or social signals. Those layers come in later milestones and must earn their weight in backtests.

### Data: candidate field (config/candidates.yaml)

Established on 2026-10-09 from news reports, party announcements and the French and English Wikipedia candidacy pages. Each candidate's sources are listed in the file, and a source cited by another page but not fetched is marked as such. The facts that shape the forecast:

- **Le Pen is eligible and running.** On 7 July 2026 the Paris court of appeal gave her 45 months of ineligibility, 30 of them suspended. The 15 firm months were already served under the March 2025 judgment, so she can run in 2027. She declared that evening; prosecutors did not appeal. Her own appeal to the Cour de cassation is pending, with a ruling expected "at the latest early April 2027". Bardella is not running and backs her.
- **LR designated Retailleau** on 19 April 2026, with 73.4% of members' votes and no primary. Lisnard left LR and is running; Bertrand says he will run as an independent; Wauquiez backs Philippe.
- **Centre:** Philippe (declared September 2024) and Attal (May 2026) are both running, with no tie-break.
- **Left:** a closed PS–Place publique–GRS primary runs on 9–10 and 16–17 October 2026, with Glucksmann, Faure, Royal, Guedj and Maurel. The left-unity primary collapsed in July. Mélenchon, Roussel, Ruffin and Tondelier run outside it. Hollande will decide in December.

Probabilities of running (`slots` in the file, each with its rationale):

- Where a Polymarket market exists, it is used: the "who will be on the ballot" market for Le Pen 0.91 and Bardella 0.07, and the thin "Attal vs Philippe" market for the centre.
- Elsewhere they are judgement calls from declarations, past ballot access and sponsorship odds.
- The PS primary slot (Glucksmann 0.55, Faure 0.25, Royal 0.07, Guedj 0.05, Maurel 0.03) is **provisional until 17 October**: no poll of the primary electorate exists.

Slots are independent; a left union that withdraws several candidates together is not modelled.

### Data: polls

The 28 polls in `data/raw/polls/` have fieldwork ending between 30 April and 29 September 2026: 129 round 1 scenarios and 52 head-to-heads. They were indexed from the French and English Wikipedia poll tables and checked against each pollster's notice filed with the Commission des sondages. Every scenario was compared, not a sample. `research/sources/polls-2026-bootstrap-notes.md` records the method, the 11 places where the English index was wrong, and the corrections.

- 27 polls are `entry: primary`. Odoxa is `entry: index`, because its own page quotes only some figures.
- Where a notice gives no publication date, `published_at` is the later of the day after fieldwork and the notice's deposit date. Each file says so in `notes`.
- Left out:
  - the OpinionWay/JDD poll of 10–11 June, whose scenarios all tested an unnamed "RN candidate";
  - the OpinionWay/Fondapol runoffs, given as % of all registered voters;
  - an Ifop/Marianne question on _wished-for_ runoffs;
  - the Ifop hypothesis that counts blank votes.
- Two polls were paid for by interested parties: the Parti du vote blanc, and candidate Sébastien Bonnal. Both are kept and flagged.
- The ingest check on scenario sums was widened to 85–103 because YouGov's notice prints integer shares summing to 102.

### Polymarket (for milestone 4)

- **Winner event:** slug `next-french-presidential-election`, Gamma event id 79987, `https://gamma-api.polymarket.com/events?slug=next-french-presidential-election`. 128 Yes/No markets, of which 42 are priced; about USD 148 million traded.
- **Price history:** works with a market's Yes **token id**, not its conditionId: `https://clob.polymarket.com/prices-history?market=<token id>&interval=max&fidelity=1440` returns `{"history":[{"t":unix_seconds,"p":price}]}`.
- **Snapshot, 2026-10-09 14:39 UTC:**

  | Candidate | Yes price |
  | --------- | --------- |
  | Le Pen    | 0.41      |
  | Philippe  | 0.235     |
  | Lisnard   | 0.13      |
  | Mélenchon | 0.125     |
  | Hollande  | 0.027     |
  | Attal     | 0.019     |

- **Lisnard's price** rose from about 0.03 to 0.13 in a month while he polls 2–5%. A press report ties the move to a social-media campaign. Treat it as noise until the blend is fitted.
- **Access:** the API was read from this environment without trading. Check Polymarket's terms and its status for French users before relying on it (see Guardrails).

### First forecast (polls only), 2026-10-09

| Candidate | Wins | Reaches round 2 | Round 1 share if running (80%) |
| --------- | ---- | --------------- | ------------------------------ |
| Le Pen    | 84%  | 91%             | 34% (27–42)                    |
| Philippe  | 7%   | 51%             | 18% (12–24)                    |
| Bardella  | 6%   | 7%              | 36% (29–44)                    |
| Mélenchon | <1%  | 40%             | 17% (11–22)                    |

Le Pen leads every head-to-head poll since July: Philippe trails by 7 points on average, Mélenchon by 34. A polls-only model therefore makes her a clear favourite, while Polymarket prices her at 0.41. The model does not know three things the market may be pricing:

- Runoff polls months out may move toward the eventual winner's opponent.
- The cassation appeal.
- The campaign itself.

The round 2 error parameters are the least grounded in the model and are provisional. Milestone 3 must fit them on the 2017 and 2022 runoff polls before this number is read as more than "what the polls say today".

### Sources added

- `research/sources/round2-transfers-2017-2022.csv`: second-round transfer surveys, 2017 and 2022. Each row has its source URL and base; see `research/sources/README.md`.
- `research/sources/results-2017-2022.csv`: official results for 2017 and 2022, checked against the Conseil constitutionnel decisions.
- `research/sources/round1-final-poll-errors-2002-2022.csv`: results and final-week poll averages for round 1, 2002–2022, from the English Wikipedia result and poll tables. The poll averages are not checked against primary sources yet; milestone 3 should check them before fitting.
- `research/sources/polls-2026-bootstrap-notes.md`: how the 2026 polls were indexed and checked.
