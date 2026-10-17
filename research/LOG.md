# Research log

Dated entries for every assumption, parameter change, source added or dropped, and the reason. Newest first within a day.

## 2026-10-17

### Model 0.3.0: an error that grows with the horizon

Six months out, the backtest's 80% intervals held the result far less often than 80% (59% at 182 days with 2017 and 2022), because the round 1 error was the same at any distance from the vote. SPEC.md (model step 4) asks for an error that shrinks as election day approaches; this adds it and fits it on the five elections now replayed.

**Model.** `round1_error` gains `horizon_bloc_sd` and `horizon_candidate_sd`, and `round2` gains `horizon_error_sd`: further error per square root of 30 days before the round, added in variance to the election-day error, drawn with the same Student-t. It sits on top of the aggregation's random walk, whose scale is fitted to poll-to-poll movement and so misses what the campaign itself still changes (2022: Zemmour from 19% in November to 7.1%; 2017: Fillon from 29% in December to 20.0%). Round 2 gets its own `student_t_df` (5, not fitted), so that fitting round 1's tails no longer moves round 2: in a first run with a shared value, round 1 chose 3 and the runoff score fell (1.54 → 1.15 on 2017–2022).

**Fit.** Three stages, each a grid over a few parameters with the rest held: round 1's horizon errors and tails (`horizon_bloc_sd` × `horizon_candidate_sd` × `student_t_df`), round 1's election-day errors, then round 2's two errors. The rule of 2026-10-16 is kept in each stage (adopt the best total among points that score at least as well as the stage's start on every election), and the stages repeat until a pass changes nothing, so the result scores at least as well as 0.2.0 on every election. Leave-one-out now reruns the whole fit without each election. The final-week averages no longer enter (see the entry below).

**Parameter changes** (`config/model.yaml`):

| Parameter                           | Was           | Now        |
| ----------------------------------- | ------------- | ---------- |
| `round1_error.horizon_candidate_sd` | – (0)         | 0.20       |
| `round1_error.student_t_df`         | 5             | 3          |
| `round1_error.horizon_bloc_sd`      | – (0)         | 0 (fitted) |
| `round2.horizon_error_sd`           | – (0)         | 0 (fitted) |
| `round2.student_t_df`               | – (shared, 5) | 5          |
| `model_version`                     | 0.2.0         | 0.3.0      |

Six months out, a candidate's within-bloc error goes from 0.20 to 0.53 on the log scale; a week out, from 0.20 to 0.21. Starting from 0.2.0 the fit converged in two passes (`research/backtest/fit-0.2.0-to-0.3.0.md`, the report of that run); rerun from 0.3.0 it keeps every value (`research/backtest/report.md`), so 0.3.0 is a fixed point.

**Scores, 20 replays (2002–2022 × 182, 91, 30, 7 days)**, 0.2.0 → 0.3.0:

|                                            | 0.2.0                       | 0.3.0                       |
| ------------------------------------------ | --------------------------- | --------------------------- |
| 80% coverage, all / 182 / 91 / 30 / 7 days | 67% / 64% / 70% / 61% / 74% | 69% / 73% / 73% / 62% / 67% |
| Share log score (contenders)               | −8.1                        | −7.0                        |
| Qualification log-loss                     | 0.173                       | 0.165                       |
| Win log-loss                               | 0.073                       | 0.073                       |
| Runoff log score                           | −8.06                       | −7.74                       |

**What the fit would not adopt, and why.** The best totals on the grid are wider still (`horizon_bloc_sd` 0.07, `horizon_candidate_sd` 0.33–0.40, `student_t_df` 2.5, total −5.5 against −7.0 chosen), but each scores slightly worse on at least one quiet election (2002, 2007 or 2012, by 0.03 to 1.1). Under the rule, a value one election disfavours is not adopted, so coverage at six months stays below 80% (73%). Heavier tails also narrow the middle of the distribution, which is why one-week coverage falls (74% → 67%) while the log score there improves (7.83 → 7.87). The 182-day log score alone is slightly worse (−2.26 → −2.39); the gain is at 91 and 30 days. Relaxing the rule to plain maximum likelihood would take these wider values; that is a decision for review, not made here.

**Leave one election out** (in the run from 0.2.0). Without 2007, 2017 or 2022 the fit picks exactly 0.3.0; without 2002 it picks `horizon_bloc_sd` 0.07, `horizon_candidate_sd` 0.33, `student_t_df` 2.5, and without 2012 `horizon_bloc_sd` 0.02, `horizon_candidate_sd` 0.33. The held-out election's round 1 score improves over 0.2.0 for 2007, 2017 and 2022 and worsens slightly for 2002 (−1.81 vs −1.61) and 2012 (4.72 vs 5.27).

**Effect on the 2027 forecast** (polls to 16 October, 50,000 simulations, 0.2.0 → 0.3.0): Le Pen wins 81.5% → 80.2%, Philippe 6.7% → 6.6%, Glucksmann 2.6% → 3.2%, Zemmour 1.0% → 1.6%. Qualification: Glucksmann 16.9% → 20.8%, Mélenchon 43.8% → 40.9%, Philippe 35.2% → 32.7%, Zemmour 2.1% → 3.7%. A Le Pen–Zemmour runoff goes from 0.4% to 1.4% likely, and Zemmour wins it 3% → 18%: the horizon error moves candidates within a bloc, which is where 2022's swing happened.

**Known gaps.** The 2002 runoff (Chirac–Le Pen, never polled before round 1) is scored on the transfer table alone and is the worst runoff evidence by far (−13); no round 2 error setting fixes it. The error is still constant on the log scale whatever a candidate's size.

### Data: 2002, 2007 and 2012 polls for the backtest

The backtest's 80% intervals six months out covered the result only 59% of the time with two elections; a model error that grows with the horizon needs more elections to be fitted on. `data/backtest/2002`, `2007` and `2012` now hold every national poll of those campaigns, to the standard of 2017 and 2022, and `config/backtest.yaml` replays all five elections at 182, 91, 30 and 7 days. Model parameters and the grid are unchanged here.

- **Polls.** 2002: 210 polls, 333 scenarios; 2007: 204 polls, 574 scenarios; 2012: 288 polls, 659 scenarios, extracted by `data/backtest/extract-wikipedia-polls.mts` from the English Wikipedia revisions already pinned for the final-week averages (2002 `oldid=1329460026`, 2007 `oldid=1341388824`, 2012 `oldid=1279812574`), which reach back well over a year before round 1. In the replay window (fieldwork from a year before round 1): 128, 189 and 241 polls. The extractor gained rules for the older tables, listed in its header: sub-national sections ("By department", "By commune") are skipped; Louis Harris and LH2 are one firm, `lh2`, distinct from Harris Interactive; the 2012 NPA "(nominee)" column, before Poutou was chosen, is read as Poutou; the TNS Sofres 20–21 April 2007 internal poll, which the page says was not published, is left out. The 2017 and 2022 outputs are byte-identical to before.
- **Primary-source check.** 149 polls, by the 2017–2022 rule (all final-week polls before either round, the latest poll of the three busiest firms at each replay date, and 15 per year drawn with `random.Random(<year>)`), nearly all through Wayback captures. 1,863 shares compared, 3 differ (TNS Sofres 18–19 April 2007, Bayrou–Sarkozy 54/46 not 53/47; Ifop June 2011, Dupont-Aignan not tested in one hypothesis); 144 polls become `entry: primary`; 5 stay `index` (three fieldwork-date differences, two partial checks). Nothing was unreachable. Details: `research/sources/README.md`.
- **Results.** `research/sources/results-2017-2022.csv` becomes `results-2002-2022.csv`, with the 2002–2012 rounds computed from the vote counts of the Conseil constitutionnel decisions (2002-109/111, 2007-139/141, 2012-152/154 PDR, Wayback captures). They match the values checked on 2026-10-16 to 0.01.
- **Blocs** (`data/backtest/<year>/candidates.yaml`, judgement calls on the scale of `config/candidates.yaml`). The 16 names already mapped in `config/backtest.yaml` keep their blocs (Bayrou `centre`, Chevènement and Hue `left`, Saint-Josse `other`, …). New calls: Gluckstein, Schivardi, Myriam Martin and Bové (2002, 2007; the antilibéral candidacy) `far_left`, but Bové in 2012, an EELV primary candidate, `left`; Buffet, Autain and Braouezec `left` with the PCF, as Roussel in 2022; Taubira, Voynet, Lipietz, Larrouturou, Delanoë, Joly `left`; Lepage, Léotard (UDF), Morin (Nouveau Centre) and Borloo (Parti radical) `centre`; Madelin, Boutin, Séguin, Balladur, Villepin `right`; Mégret and Jean-Marie Le Pen `far_right`; Pasqua and de Villiers (sovereigntist, as `config/candidates.yaml` places de Villiers), Dupont-Aignan, Nihous, Waechter, Lalonde, Dieudonné, Miguet, Cheminade and Tapie `other`. Jean-Marie Le Pen has his own id, `jm_le_pen`.
- **Final-week averages no longer used.** The fit rule (`backtest/src/report.rs`) requires a grid point to score at least as well as the current setting on every source and then takes the best total. With 2002–2012 replayed, keeping them in `final_polls.years` would score the same election-day miss twice (once in the 7-day replay, once in the final-week average), giving those elections double weight in the total and an extra veto in the every-source test. `final_polls.years` is now empty; the files and the bloc mapping stay as checked reference data, and the report no longer prints final-week rows when none is used.
- **Leakage and gaps.** Before February 2002 the law banned publishing polls in the week before each round, and the 2002 page lists polls only up to the Friday before; the replays at 7 days are unaffected. In 2002 the runoff (Chirac–Le Pen, 82–18) was polled only after round 1, so every 2002 replay predicts that matchup from the transfer table alone. The transfer table was set from 2022 election-day surveys, so it leaks into all five elections, as before.
- **Deviation from SPEC.md**, which names 2007 and 2012 "for the poll model and error curve only": they are now full replays, scored and used by the fit like 2017 and 2022, because the horizon-dependent error needs replays at every horizon, not only final-week errors.
- **Quick run** (200 simulations per setting, scratch output, not committed; the report is regenerated with the horizon-dependent error in the next change): 80% coverage 61.8% at 182 days, 62.7% at 91, 57.4% at 30 and 68.9% at 7; by election 2002 56.7%, 2007 68.1%, 2012 65.8%, 2017 67.4%, 2022 58.3%. With the current error settings no grid point beats them on every election, so the fit keeps them.

## 2026-10-16

### Field: 2026-10-14-polony-declares (via Grok Bot field_change `2026-10-14-polony-declares`)

Natacha Polony declared her candidacy on France Inter on 14 October 2026; she is added as a declared candidate in her own slot.

Added Natacha Polony (Nous sommes la France), who declared on France Inter on 14 October 2026 (franceinfo, Le Figaro). New slot polony at p 0.15: a first-time candidate who must still collect 500 sponsorships and has not been on a presidential ballot.

- `config/candidates.yaml`: new candidate `polony` (Natacha Polony, other, declared) in slot `polony` at p 0.15; `as_of` 2026-10-16.
- Sources: [L'essayiste Natacha Polony annonce sa candidature à l'élection présidentielle](https://www.franceinfo.fr/elections/presidentielle/l-essayiste-natacha-polony-annonce-sa-candidature-a-l-election-presidentielle_8236714.html), [Présidentielle 2027 : «Je suis candidate», annonce la journaliste Natacha Polony](https://www.lefigaro.fr/elections/presidentielles/presidentielle-2027-je-suis-candidate-annonce-la-journaliste-natacha-polony-20261014).
- Events: none.
- Sent by Grok Bot fr2027 (checks-1, run `checks-2026-10-16T16:39Z`); checked and applied by the Live workflow.

### UI in ten languages, checked against Wikipedia

The UI is written in French, the reference, and in English (UK), German, Italian, Spanish (Spain), Portuguese (Portugal), Japanese, Korean, Chinese (simplified) and Thai, with Intlayer (rules in `AGENTS.md`, "UI languages").

- **Terms.** The election vocabulary of each translation (rounds, pollster, fieldwork, parrainages, primaries, political families, being on the official ballot) was compared with that language's Wikipedia articles on the 2027 and 2022 French presidential elections and on French presidential elections in general, found through the French articles' interlanguage links. Korean and Portuguese have no 2027 article; Portuguese Wikipedia's 2022 article is in Brazilian Portuguese, so its "turno", "pesquisa" and "cédula" were not taken, and Spanish Wikipedia's Latin American "boleta" and "balotaje" were not either. Changed to match the articles: English says "first round" and "second round", never "round 1", and "scenarios" for the line-ups a poll tests; German "Befragungszeitraum" for fieldwork and "Kandidatur angekündigt"; Thai "ระยะเวลาการสำรวจ" for fieldwork; Japanese "正式候補" and Korean "후보 등록" for being on the official ballot; Korean "추천인 서명" for parrainages. Italian "sondato in sfida diretta" replaces "testa a testa", which reads as "neck and neck" (a judgment call, not from the articles). Chinese, Spanish and Portuguese needed no change. The articles do not settle the poll sponsor, the methodology notice or vote transfers in most languages; those stay as translated.
- **Candidates' articles.** Each candidate card links to the candidate's article in the reader's language when that Wikipedia has one, found through the French article's interlanguage links, and otherwise to the French article, labelled as French. `app/web/scripts/wikipedia-articles.ts` writes the links to `app/web/src/articles.ts` (`moon run web:articles`). English Wikipedia has all 29 candidates who have a French article; Thai has 8. "Selma Labib", the title `config/sources.yaml` gives for Selma Labib, does not exist on French Wikipedia, so her card has no link and her page views cannot be collected.

### Grok Bot: X stance from random samples, with intervals and double-labelling

The owner's design for better X data, still collected by routine 1 (`x-activity-3`), replacing `x-activity-2`'s rule of 25 posts, mixing recent and most engaged, for the 10 most-mentioned candidates.

- **Representative sample.** For each candidate with at least 100 mentions in the 12-hour window, 200 posts drawn at random across the window (not top or most relevant), reposts and accounts that look automated left out, at most 2 posts per author; below 100 mentions, every eligible post. The drop records this in `sampling`. Top or most-engaged posts over-represent the loudest accounts, so their mix says little about what X writes as a whole.
- **Stance, not tone.** Each post is checked for relevance to the candidate (namesakes such as Jean-Marie Le Pen, short surnames, a country called Hollande) and, if relevant, labelled `support`, `oppose` or `neutral` toward the candidate, following the versioned rubric `docs/x-stance-rubric.md` (`stance-1`: definitions, ten decision rules, 22 constructed French examples). Generic sentiment mixed the tone of a post with its stance: a supporter angry at the media read as negative.
- **Counts in, shares out.** The Bot sends integer counts (`posts_drawn`, `off_topic`, `support`, `oppose`, `neutral`); the pipeline rejects counts that do not add up, and computes in `data/clean/grok.csv` the shares among relevant posts with 95% Wilson score intervals (z = 1.95996). Wilson rather than the normal interval because shares near 0 and samples of a few dozen posts are common. The `sentiment_*` columns are derived from the shares (pos = support, neg = oppose, neu = neutral), which keeps the Signals view unchanged; the Bot sends `sentiment: null` with stance, so there is one source for each number. Old drops keep their sentiment and have empty stance columns.
- **Reliability.** 20% of each candidate's posts are labelled a second time in a pass that does not see the first labels. The drop carries the 4 × 4 table of (first, second) labels over all candidates, and the pipeline computes the observed agreement and Cohen's kappa per drop (`drop_agreement`, `drop_kappa`, repeated on each row of the drop). Kappa measures the Bot's run-to-run consistency, the open question it answers; it cannot detect a bias both passes share.
- **Cost.** About 4,700 labels a run (3,600 posts for the 18 candidates above 100 mentions in the first drop, a few hundred for the others, 20% relabelled), against about 250 for `x-activity-2`. Not used by the model, as before.

### UI redesign: obsidian glass, the tricolour and Delacroix

The UI is redesigned around three references from the owner: obsidian glass, the French colours, and Delacroix's _La Liberté guidant le peuple_.

- **One dark theme.** Panels are frosted glass over a near-black page; the light theme is gone.
- **The painting.** `app/web/src/assets/liberte.avif` (3,200 px wide, 242 kB) and `liberte.webp` (2,400 px, 311 kB, for browsers without AVIF) are made from the Wikimedia Commons file "La Liberté guidant le peuple - Eugène Delacroix - Musée du Louvre Peintures RF 129 - après restauration 2024.jpg", a photograph of the canvas after its 2024 restoration, public domain. Each view shows its own detail behind its heading (Liberty and the flag, the crowd behind her, the man kneeling in blue, white and red, the boy and Notre-Dame, the flag, the signature), and changing view pans across the canvas (`app/web/src/painting.ts`). The footer credits the work.
- **Fonts.** Bodoni Moda for headings and Inter for text and every number, both under the SIL Open Font License, bundled from the `@fontsource-variable` packages. The site's Content-Security-Policy allows only same-origin fonts, styles and scripts: every view, loaded in a browser under that policy, reports no violation. Files under `/assets/` are named after their content, so `_headers` now lets browsers cache them for a year.
- **Neutrality.** The flag's colours are ornament only: the rule along the top, the cocarde, the current view's marker. Neither colour is tied to a candidate or a bloc. Every candidate's numbers are drawn in one blue; polls and the pooled small candidates in grey; red marks only the pollster a reader highlights, and the negative share of X sentiment. The blue (#587cf1) and red (#e64343) pass the data-viz palette checks on the glass surface: colour-blind separation ΔE 26 (target 8), contrast over 3:1.
- **Overview.** A dot chart leads: one dot per percentage point of each candidate's chance of winning, with everyone below half a point pooled (`outcomeGroups` in `app/web/src/charts.ts`). Dots are rounded as the table rounds its percentages, so they can add up to 99 or 101, and only a certainty fills all 100 (a ">99%" chance gets 99). Sharing out exactly 100 by largest remainder was tried and dropped: it gave Le Pen 81 dots while the table shows 82%. The candidate table, sorted by chance of winning, now shows that column first, then reaching round 2, then running, so the first three stay on screen on a phone.
- **Round 2.** The matrix's colour goes from rouge for an unlikely pairing to bleu for a likely one, through violet, getting lighter as it goes; every step carries white text at 5:1 or more. Red and blue are also the left and the right in French politics: here they only say how likely a pairing is, by the same rule for every pair. With the current forecast every pairing with Le Pen is among the bluest, because she reaches round 2 in 89% of simulations. Each likely duel is a split bar in one blue, with a mark at 50%: where the gap falls says who is ahead.
- **Candidates.** A new view lists everyone in `config/candidates.yaml` by political family, far left to far right, then others, the likeliest to run first within each. LFI (Mélenchon) and PCF (Roussel) are listed with the far left, as RN and Reconquête are with the far right; the config's blocs put them with the left, and the model still uses those blocs for vote transfers and correlated errors. Each card gives the chance of running, reaching round 2 and winning, the round 1 share if running, and a link to the French Wikipedia article. Each card also links to the party's website (Wikidata's official website, each loaded on 2026-10-16; none found for La Convention) and to the 2027 programme once published: none was on 2026-10-16. LFI announces its 2027 edition of L'Avenir en commun for 6–7 November, and Philippe has presented proposals on television without publishing a programme.
- **Portraits.** Each candidate's portrait is the lead image of their French Wikipedia article (the titles in `config/sources.yaml`), taken only when the Wikipedia API lists it under a free licence: CC BY, CC BY-SA, CC0, public domain, or the European Parliament's reuse-with-credit terms. 29 of the 31 candidates have one; Selma Labib's article has no free image and Sébastien Bonnal has no article, so both show initials. One crop rule for everyone: OpenCV's frontal-face detector finds the face, and the square crop makes its box 58% of the side with its centre at mid-height, moved only as far as the photograph's edges require. 144 px WebP files, 85 kB together. The UI shows them in black and white under one vignette, so no party or EU-flag backdrop brings its colours and studio portraits and press photos look alike. The Candidates view lists each author, licence and Commons page. Vite no longer inlines small files as `data:` URLs, so the portraits stay separate, cacheable files.

### Pipeline: Grok Bot checks polls blind and changes the field

The owner's decision: Grok Bot is the only agent that collects and edits, and no person is in the loop. The two jobs a person did, checking polls against their notices and changing `config/candidates.yaml`, go through the inbox as two new item kinds that the Live workflow checks and commits like any other data (`docs/grok-bot.md`, routine 4).

**Blind double entry of polls (`poll_check`).** A poll from routine 2 is still proposed in a pull request, but it now counts only when a second entry agrees. Routine 4 lists the open proposals, reads only each poll's id and PDF links (the pull request lists them, with none of the numbers), enters the whole poll again from the notice or publication, and sends it as a `poll_check`. Live compares the two on the poll id, `supersedes`, fieldwork dates, sample size and, for every scenario matched by round and candidate set (not by scenario id or order), every share, exactly. Agreement commits the proposed file to `data/raw/polls/` unchanged, in the same run, which refits and republishes; the pull request gets a comment with the commit and is closed. Any difference rejects both items. Their notes, and a comment on the pull request, name only which fields and which candidates' shares differ, never a value from either entry: the notice or publication is the reference, no person reads it, and neither routine should settle a disagreement by taking the other's figure. The pull request stays open for another check. Routine 2 resends a corrected poll (it replaces the open proposal) or routine 4 re-reads the PDF and checks again. A check of a poll with no open proposal, or already on `main`, is rejected.

- **Why exact agreement of two independent entries.** It replaces the person who compared each number with the notice. Two transcriptions of the same PDF that agree to the decimal on every share are very unlikely to share a mistake, unless the second copied the first, which is why the check is blind. Urls, notes, sponsor, method, population and `published_at` are not compared: they are conventions more than figures, and the proposal's are kept.
- **Same-run rule.** Every `poll`, `poll_check` and `field_change` carries `agent: {name, skill_version, run_id}`, where `run_id` is the routine's name and the UTC minute its run started (`polls-2026-10-16T07:30Z`). Live refuses a check whose `run_id` equals the proposal's. This is the simplest rule that still lets routine 4 propose polls itself (late notices, below) and have a later run of routine 4 check them, which a rule on `skill_version` would not. It cannot stop a routine that reads the proposal's numbers; the instructions forbid it, and the pull request does not show them.
- **Superseding.** A poll may carry `supersedes: <poll on main>`; Live refuses one that names an unknown poll or one already superseded. Routine 4 looks for notices filed for polls whose file name has no notice number and sends them that way, so they go through the same blind check.

**Field changes (`field_change`).** A sourced change to the field, made of three closed operations: `set_status` (status, and optionally notes, declaration date and a source), `set_slot` (a slot's whole options and rationale) and `add_candidate` (the entry, the names news and X use, and a slot). Live applies them to the parsed field, checks the result like any config change (every candidate in exactly one slot, slot sums at most 1, the schema, `config/sources.yaml` naming every candidate), and only then rewrites the affected candidate and slot blocks of `config/candidates.yaml`, adds new blocks, and moves `as_of` to the day. Comments and every other line stay byte for byte; the edited file is read back and compared with the intended field before anything is written. The entry in this log, first under the day's heading, carries the summary, the agent's paragraph, the sources and the marker "via Grok Bot field_change `<id>`", which also makes a resent change a no-op. A change that fails any check is rejected whole, with every reason. Routine 4's rules: a confirmed fact in a primary source or a national outlet, probabilities changed only for confirmed facts, a loser stays in their slot at p 0, a polled candidate missing from the list gets `add_candidate`.

- **Deviation from the request: `config/sources.yaml`.** The request allowed the Live commit to touch only `config/candidates.yaml` and `research/LOG.md` for a field change. `fr2027 validate` requires every candidate to have an entry in `config/sources.yaml` (the names headlines and X are matched on), so `add_candidate` also adds that one line there; Live allows the three files only when a field change was applied in the same run.
- **SPEC.md** is updated to match: routine 4 replaces the person who reviewed polls and the weekly research review.

**Where it lives.** Schemas `schemas/poll_check.schema.json` and `schemas/field_change.schema.json` (polls gain an optional `agent`); comparison in `collectors/src/poll_check.rs`; block edits in `collectors/src/yaml_text.rs` and `collectors/src/field_change.rs`; the pull in `cli/src/inbox.rs`, which writes `.inbox/pr-actions.json` (comments and closures for the workflow) and `.inbox/field-changes.json`. The Worker accepts the two kinds, and lets a proposed item be settled once more as accepted or rejected. `docs/grok-bot.md` now holds each routine's full instruction, which the routines fetch at run time.

### Data: notices 10318 and 10319 for the two OpinionWay polls of 13–14 October

**Late notices.** The Commission's list now carries notices 10318 (OpinionWay, Présitrack wave 3 for Les Echos and Radio Classique) and 10319 (OpinionWay for CNews), both filed 15 October. Two new files supersede the files entered this morning without a notice number:

| File                          | Supersedes                        | Checked against | Entry               | R1 / R2 |
| ----------------------------- | --------------------------------- | --------------- | ------------------- | ------- |
| `opinionway-2026-10-14-10318` | `opinionway-2026-10-14-les-echos` | notice 10318    | primary             | 3 / 0   |
| `opinionway-2026-10-14-10319` | `opinionway-2026-10-14-cnews`     | notice 10319    | primary (was index) | 1 / 3   |

- Every share of every scenario was compared with the notices' "Publié" column. CNews: all 13 round 1 shares and the three runoffs match the earlier file, which had been taken from the JDD's reproduction of OpinionWay's chart. Présitrack: all shares match OpinionWay's report, except that the notice's scenario 1 table prints Le Pen 4% and Zemmour 35%, the reverse of the report and of the notice's scenarios 2 and 3; the report's values are kept.
- **Population changed to `likely`** for both: each notice states that the published intentions are computed on registered voters certain to vote (9–10 out of 10, 731 respondents), and the "Publié" column equals that column, not the all-registered one. This differs from the `registered` classification given to earlier OpinionWay polls in `research/sources/polls-2026-bootstrap-notes.md`; earlier OpinionWay notices were not re-read.
- Présitrack's notice says 978 registered drawn from 1,001 people on its methodology page and 1,043 in its weighting tables; 1,043 (also in OpinionWay's report) is kept.
- The CNews questionnaire lists François Hollande among round 1 options, but the results table has no Hollande row; nothing is entered for him.
- The poll count is unchanged (33 polls); 32 are now checked against a notice or the pollster's report.

**New polls.** None. The Commission's list (to notice 10320) has no other voting-intention notice after 10314; notices 10315, 10316 and 10320 are not voting intentions. The French Wikipedia table's newest polls are the two OpinionWay polls above.

**Field.** No change to `config/candidates.yaml`. There is no `data/raw/events/` directory yet, and the headlines in `data/clean/news.csv` up to 16 October 08:29 UTC report no confirmed declaration, withdrawal or ruling (Le Figaro, 13 October, on the Ecologists moving towards an autonomous candidacy, describes it as uncertain).

### Model 0.2.0: round 1 and round 2 errors fitted by the backtest (milestone 3)

`fr2027 backtest` (`moon run cli:backtest`, settings in `config/backtest.yaml`) replays the model on 2017 and 2022 at 182, 91, 30 and 7 days before round 1. Each replay sees only the polls published by its date whose fieldwork started in the year before round 1, as the live forecast does. The report is `research/backtest/report.md`, every number is in `results.json`, and the run is deterministic (seed 20170423, 20,000 simulations per replay and setting, shared random numbers across settings). Sources and checks of the data: `research/sources/README.md`.

**Parameter changes** (`config/model.yaml`):

| Parameter                   | Was                | Now   |
| --------------------------- | ------------------ | ----- |
| `round1_error.bloc_sd`      | 0.12 (provisional) | 0.18  |
| `round1_error.candidate_sd` | 0.10 (provisional) | 0.20  |
| `round2.error_sd`           | 0.08 (provisional) | 0.12  |
| `model_version`             | 0.1.0              | 0.2.0 |

`student_t_df` (5), the transfer table and every aggregation setting are unchanged.

**How the values were chosen.** Each setting on a grid is scored by the log score of the realised result, the log of the predictive density the model gave it: for round 1, the realised share of each contender (a runner whose fitted share is at least 10% before the election-day error), estimated from the simulations with a Gaussian kernel; for round 2, the realised runoff share given the actual matchup, which the simulation gives exactly. Three independent bodies of evidence are scored: the 2017 replays, the 2022 replays, and the final-week poll averages of 2002, 2007 and 2012 (`research/sources/round1-final-poll-errors-2002-2022.csv` and `round2-final-poll-errors-2002-2022.csv`, both checked first). The rule: adopt the grid point with the highest total among those that score at least as well as the current setting on all three; keep the current value if none does. Round 1 first, then round 2 with round 1 at its new values. Rerun with the new values, the rule keeps them (a fixed point).

- **Why not plain maximum likelihood on the total.** It runs to the edge of the grid (bloc_sd 0.22–0.4), driven by 2022 alone: Pécresse fell from 9.5% in the last polls to 4.8%, Mélenchon rose from 16–17% to 22% and Zemmour fell from 9.5% to 7.1%. 2017 on its own wants a bloc error of 0.04. With two elections, a value one election likes and the other dislikes is not justified, so the rule asks every source to agree.
- **Why not the binary scores.** In both elections the poll leaders qualified and Macron won, so qualification log-loss rewards narrower errors whatever their true size; it gets slightly worse with the new values (0.126 → 0.135). The realised shares say how far each result fell from the forecast, which the binary events cannot. The 10% contender threshold keeps the scored candidates to those who could qualify; with 5%, small candidates' large relative misses push the fit further out (bloc_sd 0.18, candidate_sd 0.4 in an exploratory run), because the model's error is constant on the log scale whatever a candidate's size.

**Scores, all eight replays** (provisional → fitted; latest-poll average in brackets):

|                                     | Provisional   | Fitted              | Latest-poll average |
| ----------------------------------- | ------------- | ------------------- | ------------------- |
| Qualification Brier / log-loss      | 0.038 / 0.126 | 0.041 / 0.135       | 0.043 / 0.301       |
| Win Brier / log-loss                | 0.023 / 0.080 | 0.024 / 0.083       | 0.043 / 0.301       |
| Round 1 share MAE                   | 2.47 pts      | 2.48 pts            | 2.76 pts            |
| 80% interval coverage               | 60%           | 66% (78% at 7 days) | –                   |
| Share log score (contenders)        | −28.8         | −20.8               | –                   |
| Runoff log score                    | 0.31          | 1.54                | –                   |
| Final-week polls 2002–2012, R1 / R2 | 6.7 / 1.51    | 6.8 / 1.98          | –                   |

- The model beats the latest-poll average on every score overall. The average gets 2017 wrong at 6 and 3 months (it puts Fillon or Le Pen with the wrong partner) and is exactly right at 30 and 7 days in both years; its log-loss only reflects the 0.001 floor.
- **Leave one election out**, same rule: without 2017 it picks 0.18 / 0.20 / 0.12, which improves the 2017 replays over the provisional values (round 1 1.2 vs 1.0, round 2 −1.25 vs −2.33). Without 2022 it picks 0.15 / 0.16 / 0.18, which improves 2022's round 1 (−25.2 vs −29.8) but not its runoff (2.42 vs 2.69).

**Overfitting risk, and what the backtest cannot say.** Two elections, both won by Macron by a wide margin against Le Pen, are a thin sample: the runoff evidence is two outcomes plus three older runoffs (2002 rests on one poll). The changes are small and in the direction every source agrees on; they are not precise estimates. Known gaps:

- **Long horizons are still overconfident.** Coverage of the 80% intervals is 59% at 6 months, 70% at 3 months and 57% at 1 month (78% at 1 week). The error is constant across horizons, and the poll random walk chosen by marginal likelihood sits at the top of its grid (0.03 a day) in 6 of the 8 replays. A horizon-dependent error, or a wider random-walk grid, is the next model change to test; the current 2027 forecast, six months out, should be read with this in mind.
- **Error size by candidate size.** Misses on the log scale are much larger for 5–10% candidates than for leaders; the model's constant log-scale error cannot fit both.
- **Leakage the replays do not remove:** the runoff transfer table was set from 2022 election-day surveys, and the replays condition on the field that actually ran (no field uncertainty), so replay probabilities are sharper than a live forecast's. `published_at` is approximated as the day after fieldwork.

**Effect on the 2027 forecast** (50,000 simulations). With the 33 polls and the decided PS slot (both entries below): Le Pen wins 83.8% → 81.6%, Philippe 5.5% → 6.7%, Bardella 5.9% → 5.8%, Glucksmann 1.9% → 2.6%. Measured first on the 28 polls of 9 October and the provisional slot: Le Pen wins 84.5% → 82.6%, Philippe 6.6% → 8.0%, Bardella 6.0% → 5.9%; Le Pen beats Philippe in their matchup 89% → 86%; Philippe qualifies 50.9% → 49.6%, Mélenchon 40.0% → 39.6%.

**Deviations from the spec.** Replay dates are the four asked for, not monthly then weekly (`horizons_days` in `config/backtest.yaml` takes any list). Markets, news and the other layers are not in the model, so there are no ablations yet.

### Data: 2017 and 2022 polls for the backtest

`data/backtest/2017/polls.csv` (334 polls, 972 scenarios) and `data/backtest/2022/polls.csv` (395 polls, 1,243 scenarios), extracted from pinned English Wikipedia revisions by `data/backtest/extract-wikipedia-polls.mts`, then checked against primary sources on a sample of 155 polls: 31 of 2,422 shares differed and are corrected; 134 polls are now `entry: primary`. The 2002–2022 final-week poll averages were checked too: the results match the Conseil constitutionnel, every final-week poll of 2002–2012 matches its primary source, and the 2007 average is corrected (it used an internal TNS poll that was not published). Details and what could not be checked: `research/sources/README.md`.

- Blocs of the 2017 and 2022 candidates (`data/backtest/<year>/candidates.yaml`) are judgement calls on the scale of `config/candidates.yaml`: Dupont-Aignan, Lassalle, Asselineau and Cheminade `other`; Macron and Bayrou `centre`; Zemmour `far_right`; Philippot `other`; Lagarde `centre`.
- The backtest thins overlapping polls of one firm (Ifop's and OpinionWay's daily rolling polls): up to 110 of 282 published polls in a replay.
- The verification found Wikipedia links pointing to the wrong poll (Elabe October 2021, Harris April 2017 and 2022). The live conseil-constitutionnel.fr refused this machine after the check probed about 240 decision URLs by number; the decisions were read from Wayback copies. Proposed rule for agents: find official documents through a search or the Wayback CDX API, never by enumerating URLs on a government site.

### Field: Glucksmann is the PS–Place publique–GRS nominee

Raphaël Glucksmann won the closed primary in its first round with about 66% (Faure about 14.5%; Royal, Maurel and Guedj behind), on a vote held 9–11 October after electronic voting was extended by a day following a cyberattack. The 16–17 October round 2 was therefore not held ([Public Sénat](https://www.publicsenat.fr/actualites/politique/primaire-de-la-gauche-raphael-glucksmann-plie-le-match-et-remporte-le-scrutin-des-le-premier-tour-avec-66), [French Wikipedia](https://fr.wikipedia.org/wiki/Primaire_pr%C3%A9sidentielle_socialiste_fran%C3%A7aise_de_2026)).

- `config/candidates.yaml`, slot `ps_primary`: Glucksmann 0.55 → 0.95, the same level as other designated nominees (Mélenchon 0.95); the remaining 0.05 covers a left-union deal or a withdrawal. Faure, Royal, Guedj and Maurel go to 0 and stay in the slot, because every candidate must sit in one. `as_of` moves to 2026-10-16.
- With this and the five new polls, a local run gives Le Pen 83.8% to win, Bardella 5.9%, Philippe 5.5%, Glucksmann 1.9% (15.4% to reach round 2, up from 5.9% with the provisional slot).

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
