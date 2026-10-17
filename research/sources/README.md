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

## `results-2002-2022.csv`

Official round 1 and round 2 results for 2002–2022, as % of expressed votes (was `results-2017-2022.csv`).

- **2017 and 2022** are taken from the French Wikipedia results tables. Their vote counts match the Conseil constitutionnel decisions cited in `notes`. Special rows give turnout and abstention (% of registered voters) and blank and null ballots (% of voters). The 2017 null-ballot figures rest on Wikipedia alone: the Conseil constitutionnel decisions for 2017 do not list them.
- **2002, 2007 and 2012** (added 2026-10-17) are computed from the vote counts in the Conseil constitutionnel decisions themselves (2002-109, 2002-111, 2007-139, 2007-141, 2012-152 and 2012-154 PDR), read from Wayback captures recorded in each row's `notes`; the candidates' votes add up to the decision's expressed votes exactly. They agree to 0.01 with the values already checked in `round1-final-poll-errors-2002-2022.csv` and `round2-final-poll-errors-2002-2022.csv`. Blank and null ballots were counted together before 2014, so those years have one `_blank_null` row (% of voters) instead of `_blank` and `_null`.

## `round1-final-poll-errors-2002-2022.csv` and `round1-final-polls-2002-2012.csv`

Round 1 results and final-week poll averages, 2002–2022, per main candidate. The backtest used 2002, 2007 and 2012 as extra evidence on the election-day error until 2026-10-17; since then those years are replayed like 2017 and 2022, and the files are kept as checked reference data (`final_polls.years` is empty in `config/backtest.yaml`, see `research/LOG.md`). Checked on 2026-10-16; `check_notes` says what was checked for each year.

- **Rule.** The average is, for each firm, its latest poll with fieldwork ending in the 7 days before the vote, averaged over firms. It reproduces every value of the original file for 2002, 2012, 2017 (with Le Terrain left out) and 2022 (with Ipsos = the 8 April Le Monde/Cevipof wave), from pinned English Wikipedia revisions: 2002 `oldid=1329460026`, 2007 `oldid=1341388824`, 2012 `oldid=1279812574`, 2017 `oldid=1213598918`, 2022 `oldid=1363121499`.
- **Correction, 2007.** The original average included TNS Sofres 20–21 April, an internal survey run during the publication ban and not published before the vote. It is replaced by TNS Sofres 18–19 April, published: Sarkozy 28.42 → 28.08, Royal 24.25 → 24.08, Bayrou 17.83 → 18.08, Le Pen 13.75 → 13.92, Besancenot 4.58 → 4.75.
- **Results** for 2002, 2007 and 2012 match the Conseil constitutionnel decisions 2002-109, 2007-139 and 2012-152 PDR to 0.01 for all 21 candidates (read from Wayback copies: the live site refused connections).
- **Polls.** `round1-final-polls-2002-2012.csv` has one row per final-week poll, with the English index value, the French Wikipedia comparison and the primary source read (`primary_url`, `primary_check`, `retrieved_at`). Every share of every poll matches its primary source. Sample sizes in the index are often the base of the shares (registered or certain-to-vote respondents) rather than the full sample; the rows say which.
- **Not checked:** whether Ifop-Fiducial 18–20 April 2012 (a non-rolling wave dated 21 April) was published before the ban; with Ifop's rolling poll instead the 2012 averages move by at most 0.1 point. The French Wikipedia revision for 2012 stops before the final week, so 2012 has no second index.

## `round2-final-poll-errors-2002-2022.csv` and `round2-final-polls-2002-2022.csv`

Runoff results and final-week poll averages, 2002–2022, as the winner's share of the two finalists, built on 2026-10-16 with the same rule (latest poll per firm, fieldwork in the 7 days before the publication ban, published in France). Results are the Conseil constitutionnel proclamations (2002-111, 2007-141, 2012-154 PDR; 2017 and 2022 from `results-2002-2022.csv`). The backtest used 2002–2012 until 2026-10-17; they are now replayed.

- **2002:** only Ipsos (79) qualifies; CSA's 3 May poll (80) was released on 4 May for the Tribune de Genève, after the French ban, and Sofres published three scenarios rather than one figure. One poll is thin evidence; `notes` gives the alternative (79.5 with CSA).
- **2007:** 7 firms, 53.79. OpinionWay 2–3 May could not be checked against any source and repeats Ipsos's dates and shares; without it, 53.75. Update 2026-10-17: the backtest-poll check found OpinionWay's own report (`pdf/jdd.fr-vague_5.pdf`, linked from a May 2007 Wayback copy of opinion-way.com): 54/46, 2–3 May, 1,415 respondents, as indexed; the row above is left as it was checked on 2026-10-16.
- **2012:** 8 firms, 52.75. **2017:** 8 firms, 61.63, all checked. **2022:** 11 firms, 55.23 (AtlasIntel's own figure is 53.3, the index rounds it to 53).

## Backtest polls: `data/backtest/<year>/polls.csv`, `corrections.csv`, `primary-checks.csv`

Every national voting-intention poll for 2002, 2007, 2012, 2017 and 2022 listed in the English Wikipedia poll pages, at pinned revisions (2002 `oldid=1329460026`, 2007 `oldid=1341388824`, 2012 `oldid=1279812574`, retrieved 2026-10-17T00:43Z; 2017 `oldid=1213598918`, 2022 `oldid=1363121499`, retrieved 2026-10-16T13:57Z), in the columns of `data/clean/polls.csv`. The 2002–2012 revisions are the ones already used for the final-week averages; each covers polls from years before round 1 (from 1996, 2004 and 2007), so the one-year replay window is complete. `data/backtest/extract-wikipedia-polls.mts` produces them (`node data/backtest/extract-wikipedia-polls.mts`); its header lists every extraction rule. Adding 2002–2012 left the 2017 and 2022 files byte for byte unchanged.

- **Counts.** 2002: 210 polls, 333 scenarios (144 polls with round 1 scenarios, 172 with head-to-heads), from April 1996 to May 2002. 2007: 204 polls, 574 scenarios (170 and 193), from May 2004. 2012: 288 polls, 659 scenarios (242 and 243), from October 2007. 2017: 334 polls, 972 scenarios (245 and 265), from October 2012 to May 2017. 2022: 395 polls, 1,243 scenarios (326 and 332), from October 2017 to April 2022. The backtest uses only polls whose fieldwork started in the year before round 1 (2002: 128, 2007: 189, 2012: 241).
- **Provenance.** Each row keeps the pollster or Commission des sondages link the table gives (`source_url`), the pinned revision (`index_url`), `retrieved_at` and the SHA-256 of the wikitext (`content_hash`). `published_at` is not in the tables and is set to the day after fieldwork ends.
- **Left out by the extraction:** the Dedicated Research poll of February 2017 (Belgian, marked by the page as not subject to French regulations); the TNS Sofres poll of 20–21 April 2007 (an internal survey, not published before the vote, as the page says); 24 rows of 2021 and 2 of 2002 where one cell covers two candidates; 6 rows of 2021 and 3 of January–February 2012 (BVA and CSA, with many "<0.5" cells) whose shares sum above 103; one poll without a sample size; and the sub-national tables of 2002–2012 ("By department", "By commune").
- **Firms** are named as today: Sofres and TNS Sofres are `kantar`; Louis Harris (2002) and its successor LH2 are `lh2`, not Harris Interactive. In 2012, the "(nominee)" column of the NPA, in polls from before the party chose its candidate, is read as Poutou.
- **Index gap.** The 2017 page lists no OpinionWay poll between April 2015 and February 2017. The 2007 page has a single OpinionWay poll (the runoff of 2–3 May).
- **Primary-source check, 2017 and 2022.** A sample of 155 polls was checked against the pollsters' publications or Commission des sondages notices (50 through Wayback captures): every poll with fieldwork in the final week before either round, the latest poll of the three busiest firms at each replay date, and 20 more per year drawn with a fixed seed. `primary-checks.csv` records, per poll, the document read, when, the result and every difference. 2,422 shares were compared; 31 differ, in 14 polls; one poll (OpinionWay April 2015) could not be found.
  - Wrong numbers in the index (the source sums to 100, the index does not): OpinionWay 23 September 2021, Elabe and Ifop 6 October 2021, Ifop 8 March 2022 (Lassalle), Ifop 8 April 2022 (Macron–Mélenchon 58/42, not 59/41), Cluster17 8 April 2022 (Pécresse 9.5).
  - Rounding or mixed bases: AtlasIntel 6 and 21 April 2022.
  - Six 2016–2017 Ifop barometers show Cheminade or Asselineau as "-" (not tested) where the index has 0.5.
  - Harris 28–31 December 2021: the index cites the earlier 10–13 December report; Harris's own report gives Macron–Zemmour 61/39.
  - Sample-size and fieldwork differences that leave the shares unchanged (Kantar 17 April 2017, Cluster17 4 and 8 April 2022, Ifop 22 April 2022, Odoxa and Ipsos runoff polls) are recorded but not corrected.
- **Primary-source check, 2002–2012** (2026-10-17). 149 polls, by the same rule: every poll with fieldwork in the final week before either round (`sample_group` a and b; the documents found for `round1-final-polls-2002-2012.csv` and `round2-final-polls-2002-2022.csv` were reused, and every other scenario of those polls was compared too), the latest poll of the three busiest firms at each of the four replay dates (c), and 15 more per year from the replay window, drawn with Python's `random.Random(<year>)` from the polls not already chosen (d). Nearly all documents were read through Wayback captures (csa-fr.com, tns-sofres.com, bva.fr, ipsos.fr, ifop2007.fr, lh2.fr, parismatch.com for Ifop's 2012 rolling poll); a few from pollsters' live sites or a press article. 1,863 shares were compared; 3 differ, in 2 polls; none was unreachable.
  - TNS Sofres 18–19 April 2007: Bayrou–Sarkozy is 54/46 in TNS Sofres's page, 53/47 in the index.
  - Ifop 21–23 June 2011: Dupont-Aignan is "-" in the Hulot hypothesis, whose column sums to 100 without him (the index has 0.5), as in the 2016–2017 Ifop barometers. Elsewhere, where a report defines "-", "*" or "<0,5" as below 0.5% (Ifop's 2011 reports, Louis Harris, LH2, CSA, Ipsos), it is read as the index's 0.5.
  - Fieldwork differences, not corrected: BVA 26–27 September 2001 (index 27–29), BVA 3–5 January 2002 (index 2–5), Ipsos 13, 14 and 16 April 2007 (index 14–16).
  - Partly checked: BVA May 2001 (its history table has no Pasqua column) and Ipsos 27 February–1 March 2007 (no copy of Ipsos's report; every share is reproduced from Le Monde's next-day article, which gives each change, but the sample and dates are not).
  - Louis Harris 4–5 January 2002 and Ifop 26 February 2007 were checked against the recall columns of the firms' later reports, which do not give the sample size.
- **Corrections.** `corrections.csv` holds the 36 share corrections (33 for 2017–2022, 3 for 2002–2012); the extractor applies them and refuses to run if a correction's index value is not the one it extracted. The 278 checked polls that now match their source (134 of 2017–2022, 144 of 2002–2012) are written `entry: primary`, with the document read as `source_url`; the rest stay `entry: index`.
- **Fields.** `data/backtest/<year>/candidates.yaml` lists everyone tested in a poll, with blocs on the scale of `config/candidates.yaml` (judgement calls, `research/LOG.md`), and gives the candidates who ran p = 1, everyone else p = 0.
