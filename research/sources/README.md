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
