# Poll data: method, checks and discrepancies

As of 2026-10-09. These notes cover `polls_r1.csv` (29 polls, 132 scenarios, 1,360 rows) and `polls_r2.csv` (14 polls, 52 head-to-heads). Fieldwork ends between 2026-04-30 and 2026-09-29. No poll in the index ends between 2026-04-01 and 2026-04-29.

## How the files were built

1. **Index.** On 2026-10-09, about 14:18 UTC, I downloaded the raw wikitext (`action=raw`) of the French page "Liste de sondages sur l'élection présidentielle française de 2027" and the English page "Opinion polling for the 2027 French presidential election". Copies are in `downloads/`. The French page was the primary index because it is more complete for April–June 2026. It also lists the Commission des sondages notice for almost every poll. `scripts/wikitable.py` parses the tables, resolving rowspan and colspan. `scripts/normalize.py` turns them into poll records. The French tables put substitute candidates inside cells, for example "4 / Faure (PS)" in the Glucksmann column, and put extra candidates in an "Autres" column. Both are attributed to the named person.
2. **Cross-check against the English index** (`scripts/compare_en_fr.py`, output in `downloads/compare_en_fr.txt`):
   - 101 French scenarios are identical in the English table.
   - 11 differ by 0.5–1 point.
   - 20 have no English equivalent with the same candidate set, mostly Le Pen scenarios from before July that the English page files separately or omits, plus the Bonnal and vote-blanc polls.
   - Every value difference was settled against the primary notice, and **in all 11 cases the French index was right** (details below).
3. **Primary sources.** For 28 of the 29 polls, the source is the pollster's own notice deposited with the Commission des sondages (`https://www.commission-des-sondages.fr/notices/files/notices/2026/...pdf`). This is the regulator-filed document, and it contains the published results tables. All 28 notices were downloaded to `downloads/primary/` and converted to text with `pdftotext -layout`. For the remaining poll (Odoxa, 20–21 May), the source is the results page on odoxa.fr.
   - `scripts/verify_notices.py` compares each Wikipedia scenario automatically with every column of numbers printed in the notice.
   - Where the layout defeated the parser, I compared by eye: two-column pages, candidate names wrapped across lines, bar charts, and Cluster17's one-decimal tables.
   - Results per scenario are in `downloads/poll_checks.json`.
4. **`primary_checked`.**
   - 1,353 of 1,360 first-round rows are `true`.
   - The 7 `false` rows are the Odoxa poll's candidates whose figures the odoxa.fr page does not quote. That page gives only Philippe 17, Mélenchon 16 and Bardella 32, and those three rows are `true`.
   - All 52 second-round rows are `true`.
5. **The task's minimum spot-check (the 5 most recent polls) was exceeded.** All 10 Ifop scenarios (25–29 Sep), all 5 Harris (22–24 Sep), all 6 YouGov (17–21 Sep), all 3 Cluster17 (15–16 Sep) and the Ifop vote-blanc poll (9–11 Sep) match their notices. Cluster17's notice prints one decimal, and its values are used (see below).

## Column conventions

- `sample_size` is the **total sample** printed in the notice. The registered-voter subsample is often smaller, for example Ifop 25–29 Sep: 1,527 total and 1,393 registered. The indexes mix the two, which explains the English/French sample differences (see the list below). The registered and total figures for every poll are in the `META` table of `scripts/build_polls.py`.
- `method` is `online` for every poll that has a notice; all of them state self-administered online interviews. It is empty for Odoxa, where the method was not established.
- `population` is the base the published vote intentions were computed on, per the notice:
  - `registered`: Ifop, OpinionWay ("Publié" column on all registered voters), Verian, and Harris waves 1–4 (April–August).
  - `likely`: Ipsos ("certains d'aller voter"), Elabe (registered and intending to vote), Harris waves 5–6 (September; the base changed to "certains d'aller voter"), YouGov ("votants décidés"), and Cluster17 (registered, weighted by declared probability of voting; my classification).
  - Empty for Odoxa.
- `published_at` is filled only where a notice or page states a publication or diffusion date:
  - Elabe 2026-07-11 and 2026-08-29 ("première publication").
  - Verian 2026-07-10.
  - Ipsos 2026-06-01, 2026-09-05 and 2026-09-13 ("date de diffusion prévue", i.e. planned diffusion).
  - Odoxa 2026-05-26 (odoxa.fr page).
  - Everything else is empty. The date in a notice filename is the date of deposit with the Commission, not necessarily the publication date.
- `scenario_label` is `S<n>` (the order in the French index) followed by the candidates that vary across that poll's scenarios. For example, "S4: Faure+Attal" means the PS slot is filled by Faure and the centre by Attal.
- `share` is the published share of expressed votes. "<1" and "<1%" are written as 0.5, following the brief. Rows for "Autres", blank votes or undecided are excluded.
- `poll_key` is `<field_end>_<firm>_<notice number>`.

## Discrepancies found and how they were resolved

**English index vs French index vs notice:**

| Poll                                     | Item                   | EN Wikipedia                                                 | FR Wikipedia       | Notice (primary)                                                                                                                                              |
| ---------------------------------------- | ---------------------- | ------------------------------------------------------------ | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ifop 25–29 Sep                           | sample                 | 1,393                                                        | 1,527              | 1,393 registered drawn from 1,527 (both right; CSV uses 1,527)                                                                                                |
| Cluster17 15–16 Sep, scenario 2 (Attal)  | Le Pen                 | 31.5                                                         | 31                 | 31.0                                                                                                                                                          |
| OpinionWay/Les Echos 8–9 Sep, S2 (Attal) | Le Pen                 | 33                                                           | 34                 | 34% ("Publié")                                                                                                                                                |
| Elabe 9–10 Jul, S2                       | Glucksmann             | 11.5                                                         | 11                 | 11                                                                                                                                                            |
| Elabe 9–10 Jul, S3                       | Roussel                | 2.5                                                          | 2                  | 2                                                                                                                                                             |
| Harris 7–8 Jul, S4                       | Dupont-Aignan          | 2                                                            | 1                  | 1                                                                                                                                                             |
| Harris 7–8 Jul                           | sample                 | 1,582                                                        | 1,592              | 1,592 registered drawn from 1,837                                                                                                                             |
| Ipsos 27–28 May, S1/S3/S8                | Roussel, Tondelier     | 4/3/3 and 4                                                  | 3/4/4 and 5        | matches FR                                                                                                                                                    |
| Ifop 26–28 May, S3/S7/S8                 | Villepin, Roussel, NDA | 2.5/3/2.5                                                    | 3/3.5/2            | matches FR                                                                                                                                                    |
| Ifop 26–28 May                           | sample                 | 1,368                                                        | 1,501              | 1,368 registered drawn from 1,501                                                                                                                             |
| Verian                                   | field dates            | 9–10 Jul                                                     | 8–10 Jul           | "Réalisée du 8 au 10 juillet 2026"                                                                                                                            |
| Harris "Vague 1"                         | date / n               | listed twice: 28–30 Apr (n=1,725) and "22 Apr 2026, n=2,000" | 28–30 Apr, n=1,725 | Toluna report: online 28–30 Apr, 1,725 registered drawn from 1,989. The English "22 Apr / 2,000" row is the same poll mis-dated, so it is not a separate poll |
| Elabe 26–28 Aug (round-2 table)          | sample                 | –                                                            | 1,503              | 1,501 (1,503 is the July Elabe sample)                                                                                                                        |

**Other issues:**

- **Cluster17 rounding.** Cluster17's notices print one-decimal "Redressé" figures, while both indexes show them rounded to the nearest 0.5 (15–16 Sep) or to integers (22–24 Jul). `polls_r1.csv` and `polls_r2.csv` use the **notice values**, for example Le Pen 30.8, Mélenchon 19.1 and Philippe 18.8 in 15–16 Sep S1, and Mélenchon 39.2 vs Le Pen 60.8 in the July runoff. For the Le Point poll (31 Aug–1 Sep), the index's Villepin 3.5 in S1 corresponds to a notice value of 3.2. That is more than rounding, so the index value was probably taken from press coverage with different rounding.
- **OpinionWay/Les Echos (8–9 Sep), hypothesis 1.** The notice's own table labels the Le Pen row "4%" and the Zemmour row "34%", so the labels are swapped in the notice. The index and the CSV use Le Pen 34 and Zemmour 4.
- **YouGov 17–21 Sep.** Published integer shares sum to 102 (S1) and 98 (S4). That is what the notice prints (column "Pondérés"), so the values were kept.
- **OpinionWay/JDD 10–11 Jun.** The poll tested a generic "Le candidat du Rassemblement national, Marine Le Pen ou Jordan Bardella", not a named person. It appears in the CSV as `RN candidate (Marine Le Pen or Jordan Bardella, unnamed)`. The French index shows it as "Bardella / Le Pen", and the English index files it under Bardella.
- **Polls commissioned by interested parties** are included and flagged in `sponsor`:
  - Ifop for the Parti du vote blanc, 9–11 Sep. The CSV uses the standard hypothesis; the hypothesis that counts blank votes is excluded, as in the index.
  - OpinionWay for Sébastien Bonnal, a self-declared candidate, 2–4 Sep. 11 names were offered, including Bonnal but not Dupont-Aignan.
- **Verian 8–10 Jul** offered only 8 candidates (no Arthaud, Tondelier or Dupont-Aignan). Its notice shows the topline only as a bar chart; the values were matched by x-position, and L'Hémicycle's article quotes Le Pen 37, Philippe 17, Mélenchon 15, Glucksmann 11 and Attal 8.
- **Harris (Toluna) 7–8 Jul** started on 7 July after the Le Pen appeal verdict and her TF1 announcement (per the notice).
- **Pre-July Bardella scenarios.** Before the 7 July 2026 ruling, most scenarios used Bardella as the RN candidate, while some tested Le Pen. Both are kept with the named candidate.

## Polls considered and deliberately excluded

- **OpinionWay for Fondapol, fieldwork 1–8 Jun 2026** (notice 10218, n=3,057 registered): second-round preferences are given as % of all registered voters including blank and abstention, not % of expressed votes. The index does not list it.
- **Ifop for Marianne, 17–18 Jun 2026**, "Les seconds tours souhaités" (notice 10210): asks which runoffs people would _like_, not voting intention.
- **Ipsos BVA / Le Monde electoral survey, 2–9 Apr 2026** (notice 10168): no first-round voting-intention question was found in the notice.
- **Elabe "primaire à gauche", 29 Sep–2 Oct 2026** (notice 10289) and **Odoxa "primaire droite centre", 8–9 Sep 2026** (notice 10257): opinions on primaries, with no vote intention for the candidates.
- **Completeness.** The Commission's notice list fetched on 2026-10-09 shows no new presidential voting-intention notice after Ifop 25–29 Sep (notice 10284, 30 Sep). The 1–8 Oct notices cover popularity and stature barometers, lycée protests, the budget, youth and the left primary. Notices can lag publication by a few days, so a poll published on 7–9 Oct could still be missing.

## Known limitations

- I am confident the 29 polls match their notices. The risk lies in coverage: polls published but never added to either index or deposited with the Commission would be missed. The notice list was checked only from January to October 2026.
- Scenario descriptions are generated automatically. The pollsters' own hypothesis titles, for example Ifop's "hypothèse Bloc central G. Attal versus E. Philippe", are in the notices.
- The "Autres" column of the French index was attributed only when a name was printed in the cell. No unnamed "Autres" values were left in the window, apart from the vote-blanc hypothesis that was excluded.
