# Research sources

Reference data the model's parameters are set from. Every number was read from the page or PDF in its `source_url`; nothing here is entered by guesswork.

## `round2-transfers-2017-2022.csv`

How each first-round electorate voted (or said it would vote) in the 2017 and 2022 Macron–Le Pen runoffs: share to each finalist and share abstaining or voting blank or null. One row per first-round candidate per survey.

- `to_finalist_a` is Macron and `to_finalist_b` is Le Pen in both years.
- Election-day surveys (`survey` without "PRE-ELECTION") and the last pre-runoff polls are both included; the pre-runoff ones are marked in `survey`.
- **Read `notes` before pooling rows.** Each starts with `BASE=` and says what the sample is (all registered voters, round 1 voters, likely voters, round 2 voters only, or expressed votes only) and what the third column covers. Bases differ between firms, which explains much of their disagreement: in 2022 the share of Mélenchon voters who went to Macron is 38 (Elabe), 42 (Ipsos, Ifop) and 54 (OpinionWay, which records far less abstention).
- For Ipsos election-day rows, `abstain_blank_null` is the sum of two printed columns (blank/null and abstention); `notes` gives both.
- `_r1_abstention_blank_null` is the row for people who did not vote for a candidate in round 1, where published.

Known gaps:

- No 2022 election-day survey with abstention covers Roussel, Lassalle or Dupont-Aignan.
- 2017 election-day coverage of the finalists' own electorates is Ifop only.

## `results-2017-2022.csv`

Official round 1 and round 2 results for 2017 and 2022, as % of expressed votes, taken from the French Wikipedia results tables. Their vote counts match the Conseil constitutionnel decisions cited in `notes`. Special rows give turnout and abstention (% of registered voters) and blank and null ballots (% of voters). The 2017 null-ballot figures rest on Wikipedia alone: the Conseil constitutionnel decisions for 2017 do not list them.

## `round1-final-poll-errors-2002-2022.csv` and `round1-final-polls-2002-2012.csv`

Round 1 results and final-week poll averages, 2002–2022, per main candidate. The backtest uses 2002, 2007 and 2012 as extra evidence on the election-day error (2017 and 2022 are scored by the replays). Checked on 2026-10-16; `check_notes` says what was checked for each year.

- **Rule.** The average is, for each firm, its latest poll with fieldwork ending in the 7 days before the vote, averaged over firms. It reproduces every value of the original file for 2002, 2012, 2017 (with Le Terrain left out) and 2022 (with Ipsos = the 8 April Le Monde/Cevipof wave), from pinned English Wikipedia revisions: 2002 `oldid=1329460026`, 2007 `oldid=1341388824`, 2012 `oldid=1279812574`, 2017 `oldid=1213598918`, 2022 `oldid=1363121499`.
- **Correction, 2007.** The original average included TNS Sofres 20–21 April, an internal survey run during the publication ban and not published before the vote. It is replaced by TNS Sofres 18–19 April, published: Sarkozy 28.42 → 28.08, Royal 24.25 → 24.08, Bayrou 17.83 → 18.08, Le Pen 13.75 → 13.92, Besancenot 4.58 → 4.75.
- **Results** for 2002, 2007 and 2012 match the Conseil constitutionnel decisions 2002-109, 2007-139 and 2012-152 PDR to 0.01 for all 21 candidates (read from Wayback copies: the live site refused connections).
- **Polls.** `round1-final-polls-2002-2012.csv` has one row per final-week poll, with the English index value, the French Wikipedia comparison and the primary source read (`primary_url`, `primary_check`, `retrieved_at`). Every share of every poll matches its primary source. Sample sizes in the index are often the base of the shares (registered or certain-to-vote respondents) rather than the full sample; the rows say which.
- **Not checked:** whether Ifop-Fiducial 18–20 April 2012 (a non-rolling wave dated 21 April) was published before the ban; with Ifop's rolling poll instead the 2012 averages move by at most 0.1 point. The French Wikipedia revision for 2012 stops before the final week, so 2012 has no second index.

## `round2-final-poll-errors-2002-2022.csv` and `round2-final-polls-2002-2022.csv`

Runoff results and final-week poll averages, 2002–2022, as the winner's share of the two finalists, built on 2026-10-16 with the same rule (latest poll per firm, fieldwork in the 7 days before the publication ban, published in France). Results are the Conseil constitutionnel proclamations (2002-111, 2007-141, 2012-154 PDR; 2017 and 2022 from `results-2017-2022.csv`). The backtest uses 2002–2012.

- **2002:** only Ipsos (79) qualifies; CSA's 3 May poll (80) was released on 4 May for the Tribune de Genève, after the French ban, and Sofres published three scenarios rather than one figure. One poll is thin evidence; `notes` gives the alternative (79.5 with CSA).
- **2007:** 7 firms, 53.79. OpinionWay 2–3 May could not be checked against any source and repeats Ipsos's dates and shares; without it, 53.75.
- **2012:** 8 firms, 52.75. **2017:** 8 firms, 61.63, all checked. **2022:** 11 firms, 55.23 (AtlasIntel's own figure is 53.3, the index rounds it to 53).

## Backtest polls: `data/backtest/<year>/polls.csv`, `corrections.csv`, `primary-checks.csv`

Every national voting-intention poll for 2017 and 2022 listed in the English Wikipedia poll pages, at pinned revisions (2017 `oldid=1213598918`, 2022 `oldid=1363121499`, retrieved 2026-10-16T13:57Z), in the columns of `data/clean/polls.csv`. `data/backtest/extract-wikipedia-polls.mts` produces them (`node data/backtest/extract-wikipedia-polls.mts`); its header lists every extraction rule.

- **Counts.** 2017: 334 polls, 972 scenarios (245 polls with round 1 scenarios, 265 with head-to-heads), fieldwork from October 2012 to May 2017. 2022: 395 polls, 1,243 scenarios (326 and 332), from October 2017 to April 2022. The backtest uses only polls whose fieldwork started in the year before round 1.
- **Provenance.** Each row keeps the pollster or Commission des sondages link the table gives (`source_url`), the pinned revision (`index_url`), `retrieved_at` and the SHA-256 of the wikitext (`content_hash`). `published_at` is not in the tables and is set to the day after fieldwork ends.
- **Left out by the extraction:** the Dedicated Research poll of February 2017 (Belgian, marked by the page as not subject to French regulations); 24 rows of 2021 where one cell covers Arthaud and Poutou together; 6 rows of 2021 whose shares sum above 103; one poll without a sample size.
- **Index gap.** The 2017 page lists no OpinionWay poll between April 2015 and February 2017.
- **Primary-source check.** A sample of 155 polls was checked against the pollsters' publications or Commission des sondages notices (50 through Wayback captures): every poll with fieldwork in the final week before either round, the latest poll of the three busiest firms at each replay date, and 20 more per year drawn with a fixed seed. `primary-checks.csv` records, per poll, the document read, when, the result and every difference. 2,422 shares were compared; 31 differ, in 14 polls; one poll (OpinionWay April 2015) could not be found.
  - Wrong numbers in the index (the source sums to 100, the index does not): OpinionWay 23 September 2021, Elabe and Ifop 6 October 2021, Ifop 8 March 2022 (Lassalle), Ifop 8 April 2022 (Macron–Mélenchon 58/42, not 59/41), Cluster17 8 April 2022 (Pécresse 9.5).
  - Rounding or mixed bases: AtlasIntel 6 and 21 April 2022.
  - Six 2016–2017 Ifop barometers show Cheminade or Asselineau as "-" (not tested) where the index has 0.5.
  - Harris 28–31 December 2021: the index cites the earlier 10–13 December report; Harris's own report gives Macron–Zemmour 61/39.
  - Sample-size and fieldwork differences that leave the shares unchanged (Kantar 17 April 2017, Cluster17 4 and 8 April 2022, Ifop 22 April 2022, Odoxa and Ipsos runoff polls) are recorded but not corrected.
- **Corrections.** `corrections.csv` holds the 33 share corrections; the extractor applies them and refuses to run if a correction's index value is not the one it extracted. The 134 checked polls that now match their source are written `entry: primary`, with the document read as `source_url`; the rest stay `entry: index`.
- **Fields.** `data/backtest/<year>/candidates.yaml` lists everyone tested in a poll, with blocs on the scale of `config/candidates.yaml` (judgement calls), and gives the candidates who ran p = 1, everyone else p = 0.
