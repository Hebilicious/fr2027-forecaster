# Backtest: 2017 and 2022 replays

Written by `fr2027 backtest` (moon task `cli:backtest`) from `config/backtest.yaml`, `config/model.yaml` (model 0.1.0) and `data/backtest/`; `results.json` beside this file has every number. Seed 20170423, 20000 simulations per replay and setting. How to read it and what was decided: `research/LOG.md`.

Each replay fits the model to the polls published by its date (fieldwork started at most 365 days before round 1; overlapping polls of one firm thinned), conditions on the field that actually ran, and simulates both rounds. Probabilities are clipped to [0.001, 0.999] for log-loss. Share log scores are summed over contenders (fitted share ≥ 10%) and replays; higher is better. Brier, log-loss and MAE: lower is better.

## Settings

| | round1_error.bloc_sd | round1_error.candidate_sd | round2.error_sd |
|---|---|---|---|
| Current (config/model.yaml) | 0.12 | 0.1 | 0.08 |
| Fitted | 0.18 | 0.2 | 0.12 |

## Scores

All replays (both elections, every horizon):

| | Qualify Brier | Qualify log-loss | Win Brier | Win log-loss | Share MAE (pts) | 80% coverage | Share log score | Round 2 log score | P(winner \| matchup) |
|---|---|---|---|---|---|---|---|---|---|
| Latest-poll average | 0.043 | 0.301 | 0.043 | 0.301 | 2.76 | – | – | – | 100.0% |
| Model, current | 0.038 | 0.126 | 0.023 | 0.080 | 2.47 | 60.4% | -28.8 | 0.31 | 93.3% |
| Model, fitted | 0.041 | 0.135 | 0.024 | 0.083 | 2.48 | 65.9% | -20.8 | 1.54 | 91.8% |

Final-week poll errors 2002–2012 (log score, round 1 / round 2): current 6.7 / 1.51, fitted 6.8 / 1.98.

By election:

| | Qualify Brier | Qualify log-loss | Win Brier | Win log-loss | Share MAE (pts) | 80% coverage | Share log score | Round 2 log score | P(winner \| matchup) |
|---|---|---|---|---|---|---|---|---|---|
| 2017, baseline | 0.091 | 0.629 | 0.091 | 0.629 | 2.41 | – | – | – | 100.0% |
| 2022, baseline | 0.000 | 0.001 | 0.000 | 0.001 | 3.03 | – | – | – | 100.0% |
| 2017, current | 0.058 | 0.178 | 0.047 | 0.150 | 1.77 | 65.1% | 1.0 | -2.39 | 89.7% |
| 2022, current | 0.020 | 0.079 | 0.001 | 0.016 | 3.09 | 56.2% | -29.8 | 2.70 | 96.9% |
| 2017, fitted | 0.060 | 0.183 | 0.047 | 0.150 | 1.77 | 72.1% | 1.2 | -1.25 | 89.1% |
| 2022, fitted | 0.023 | 0.092 | 0.002 | 0.022 | 3.11 | 60.4% | -22.1 | 2.79 | 94.5% |

By horizon (days before round 1):

| | Qualify Brier | Qualify log-loss | Win Brier | Win log-loss | Share MAE (pts) | 80% coverage | Share log score | Round 2 log score | P(winner \| matchup) |
|---|---|---|---|---|---|---|---|---|---|
| 182, baseline | 0.087 | 0.602 | 0.087 | 0.602 | 4.27 | – | – | – | 100.0% |
| 91, baseline | 0.087 | 0.602 | 0.087 | 0.602 | 3.32 | – | – | – | 100.0% |
| 30, baseline | 0.000 | 0.001 | 0.000 | 0.001 | 2.54 | – | – | – | 100.0% |
| 7, baseline | 0.000 | 0.001 | 0.000 | 0.001 | 1.20 | – | – | – | 100.0% |
| 182, current | 0.063 | 0.193 | 0.052 | 0.175 | 3.05 | 59.1% | -5.5 | -0.74 | 76.2% |
| 91, current | 0.069 | 0.201 | 0.037 | 0.108 | 3.07 | 69.6% | -9.8 | 1.33 | 98.7% |
| 30, current | 0.008 | 0.050 | 0.001 | 0.011 | 2.51 | 52.2% | -15.1 | 1.23 | 99.6% |
| 7, current | 0.013 | 0.061 | 0.004 | 0.025 | 1.26 | 60.9% | 1.7 | -1.51 | 98.6% |
| 182, fitted | 0.062 | 0.191 | 0.051 | 0.170 | 3.05 | 59.1% | -4.4 | -0.67 | 74.1% |
| 91, fitted | 0.069 | 0.202 | 0.037 | 0.111 | 3.08 | 69.6% | -8.7 | 1.28 | 97.5% |
| 30, fitted | 0.013 | 0.067 | 0.002 | 0.017 | 2.52 | 56.5% | -10.1 | 1.33 | 99.0% |
| 7, fitted | 0.018 | 0.080 | 0.006 | 0.035 | 1.28 | 78.3% | 2.4 | -0.40 | 96.7% |

## Reliability

Qualification and win events pooled over every replay.

| Forecast | Events (current) | Mean forecast | Observed | Events (fitted) | Mean forecast | Observed |
|---|---|---|---|---|---|---|
| 0–10% | 144 | 0.8% | 0.7% | 138 | 0.6% | 0.7% |
| 10–20% | 7 | 14.3% | 14.3% | 11 | 13.4% | 9.1% |
| 20–30% | 5 | 26.3% | 0.0% | 4 | 23.5% | 0.0% |
| 30–40% | 2 | 35.0% | 100.0% | 5 | 32.7% | 40.0% |
| 40–50% | 5 | 43.3% | 20.0% | 5 | 42.9% | 20.0% |
| 50–60% | 1 | 59.7% | 0.0% | 2 | 58.4% | 50.0% |
| 60–70% | 1 | 61.8% | 100.0% | 2 | 64.6% | 100.0% |
| 70–80% | 6 | 74.1% | 83.3% | 5 | 73.7% | 80.0% |
| 80–90% | 6 | 87.4% | 100.0% | 7 | 85.5% | 100.0% |
| 90–100% | 7 | 96.4% | 100.0% | 5 | 96.1% | 100.0% |

## Fit

The rule: among the grid points that score at least as well as the current setting on every source of evidence (each election's replays, and the final-week polls of 2002–2012), take the one with the highest total log score; if none does, keep the current value. Round 1 first, with round 2 at its current error; then round 2, with round 1 at its fitted values.

| Round 1 evidence (share log score) | Current | Fitted | Best on this source alone |
|---|---|---|---|
| 2017 replays | 1.0 | 1.2 | 2.9 (bloc_sd 0.04, candidate_sd 0.4) |
| 2022 replays | -29.8 | -22.1 | -14.4 (bloc_sd 0.4, candidate_sd 0.4) |
| 182-day replays, both elections (not used) | -5.5 | -4.4 | -3.7 (bloc_sd 0.33, candidate_sd 0.4) |
| 91-day replays, both elections (not used) | -9.8 | -8.7 | -6.9 (bloc_sd 0.4, candidate_sd 0.4) |
| 30-day replays, both elections (not used) | -15.1 | -10.1 | -6.2 (bloc_sd 0.4, candidate_sd 0.4) |
| 7-day replays, both elections (not used) | 1.7 | 2.4 | 4.4 (bloc_sd 0.04, candidate_sd 0.4) |
| Final-week polls 2002–2012 | 6.7 | 6.8 | 7.2 (bloc_sd 0.15, candidate_sd 0.13) |
| Total | -22.1 | -14.1 | -12.2 (bloc_sd 0.22, candidate_sd 0.4) |

Round 1 grid: total log score by `bloc_sd` (rows) and `candidate_sd` (columns). `*` marks the points that score at least as well as the current setting on every source; bold is the fitted one.

| bloc_sd \ candidate_sd | 0.03 | 0.05 | 0.07 | 0.1 | 0.13 | 0.16 | 0.2 | 0.25 | 0.3 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0.04 | -68.4 | -64.0 | -61.0 | -58.4 | -56.5 | -55.1 | -52.6 | -50.3 | -49.1 | -47.6 |
| 0.06 | -50.5 | -48.9 | -46.8 | -44.9 | -43.2 | -41.5 | -39.2 | -37.5 | -36.4 | -34.9 |
| 0.08 | -37.2 | -35.7 | -34.2 | -32.9 | -31.3 | -30.1 | -28.2 | -26.7 | -25.6 | -24.1 |
| 0.1 | -28.0 | -27.0 | -26.1 | -24.9 | -23.7 | -22.2 | -20.7 | -19.6 | -18.7 | -17.4 |
| 0.12 | -24.5 | -23.7 | -23.0 | -22.1* | -20.7* | -19.5* | -18.3 | -17.4 | -16.7 | -15.6 |
| 0.15 | -20.1 | -19.7 | -19.3 | -18.4* | -17.4* | -16.7* | -15.9* | -15.2* | -14.6 | -13.7 |
| 0.18 | -16.9 | -16.7 | -16.5 | -15.8 | -15.2* | -14.6* | **-14.1*** | -13.6 | -13.2 | -12.6 |
| 0.22 | -14.8 | -14.7 | -14.4 | -13.9 | -13.7 | -13.4 | -13.0 | -12.7 | -12.5 | -12.2 |
| 0.27 | -13.9 | -13.9 | -13.7 | -13.5 | -13.4 | -13.2 | -13.0 | -12.8 | -12.7 | -12.7 |
| 0.33 | -14.1 | -14.1 | -14.0 | -14.0 | -13.9 | -13.8 | -13.8 | -13.7 | -13.7 | -13.9 |
| 0.4 | -15.6 | -15.6 | -15.6 | -15.5 | -15.5 | -15.5 | -15.5 | -15.5 | -15.7 | -16.1 |

Round 2 grid, with round 1 at its fitted values: log score of the realised runoff share (the logit of the first finalist's share), given the actual matchup. Current error_sd 0.08: replays 2017 -2.33, 2022 2.69, final-week polls 1.51.

| error_sd | 2017 replays | 2022 replays | Final-week polls 2002–2012 | Total | P(winner \| matchup) |
|---|---|---|---|---|---|
| 0.02 | -4.93 | 1.65 | -5.71 | -8.99 | 93.8% |
| 0.03 | -4.54 | 1.85 | -2.54 | -5.23 | 93.8% |
| 0.05 | -3.61 | 2.28 | 0.13 | -1.19 | 93.6% |
| 0.08 * | -2.33 | 2.69 | 1.51 | 1.88 | 93.0% |
| 0.1 * | -1.70 | 2.79 | 1.85 | 2.94 | 92.5% |
| 0.12 (fitted) | -1.25 | 2.79 | 1.98 | 3.51 | 91.8% |
| 0.15 | -0.82 | 2.65 | 1.96 | 3.79 | 90.6% |
| 0.18 | -0.60 | 2.42 | 1.83 | 3.65 | 89.3% |
| 0.22 | -0.51 | 2.06 | 1.58 | 3.12 | 87.3% |
| 0.27 | -0.56 | 1.58 | 1.22 | 2.23 | 84.8% |
| 0.33 | -0.74 | 1.02 | 0.79 | 1.07 | 82.0% |
| 0.4 | -1.02 | 0.42 | 0.34 | -0.26 | 79.1% |

Final-week runoff polls used:

| Year | Winner | Final-week average | Result (share of the two) |
|---|---|---|---|
| 2002 | Jacques Chirac | 79.0% | 82.2% |
| 2007 | Nicolas Sarkozy | 53.8% | 53.1% |
| 2012 | François Hollande | 52.8% | 51.6% |

## Leave one election out

The same rule without one election's replays, then that election's replays scored at the setting it gives, and at the current one.

| Held out | bloc_sd | candidate_sd | round2 error_sd | Round 1 log score (chosen without / current) | Round 2 log score (chosen without / current) |
|---|---|---|---|---|---|
| 2017 | 0.18 | 0.2 | 0.12 | 1.2 / 1.0 | -1.25 / -2.33 |
| 2022 | 0.15 | 0.16 | 0.18 | -25.2 / -29.8 | 2.42 / 2.69 |

## Replays

| Election | Days out | As of | Polls used | Overlapping dropped | Random walk sd | Design effect | Contenders | Runners not yet polled |
|---|---|---|---|---|---|---|---|---|
| 2017 | 182 | 2016-10-23 | 16 | 0 | 0.012 | 2 | melenchon, macron, fillon, le_pen | hamon |
| 2017 | 91 | 2017-01-22 | 32 | 0 | 0.03 | 2 | melenchon, macron, fillon, le_pen | – |
| 2017 | 30 | 2017-03-24 | 87 | 51 | 0.03 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 7 | 2017-04-16 | 120 | 83 | 0.03 | 2 | melenchon, macron, fillon, le_pen | – |
| 2022 | 182 | 2021-10-10 | 30 | 0 | 0.02 | 2 | macron, pecresse, le_pen, zemmour | – |
| 2022 | 91 | 2022-01-09 | 74 | 4 | 0.03 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 30 | 2022-03-11 | 138 | 61 | 0.03 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 7 | 2022-04-03 | 172 | 110 | 0.03 | 2 | melenchon, macron, le_pen, zemmour | – |

### 2017, 182 days out (2016-10-23)

Actual matchup: current 11.5% (winner given matchup 59.1%), fitted 13.1% (winner given matchup 57.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 17.3% (12.4%–22.6%) | 12.8% / 15.7% ✓ | 7.5% / 9.0% ✓ |
| le_pen | 21.3% | 30.4% (23.2%–37.9%) | 95.5% / 92.9% ✓ | 27.3% / 26.9% |
| fillon | 20.0% | 21.0% (15.3%–27.0%) | 43.8% / 44.0% | 42.2% / 41.6% |
| melenchon | 19.6% | 21.3% (15.5%–27.5%) | 47.9% / 47.4% | 23.0% / 22.5% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 4.9% (3.1%–7.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.8% (0.3%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.6% (1.0%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.9% (0.4%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.4% (0.8%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 91 days out (2017-01-22)

Actual matchup: current 18.0% (winner given matchup 99.8%), fitted 18.3% (winner given matchup 99.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 20.0% (12.5%–28.5%) | 37.6% / 38.5% ✓ | 32.5% / 32.7% ✓ |
| le_pen | 21.3% | 26.0% (16.9%–35.9%) | 74.8% / 73.4% ✓ | 2.0% / 2.4% |
| fillon | 20.0% | 25.9% (16.8%–36.0%) | 74.5% / 73.0% | 59.7% / 58.2% |
| melenchon | 19.6% | 14.8% (8.6%–22.0%) | 12.7% / 14.3% | 5.6% / 6.3% |
| hamon | 6.4% | 8.0% (3.9%–12.9%) | 0.4% / 0.8% | 0.2% / 0.3% |
| dupont_aignan | 4.7% | 1.9% (0.9%–3.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.4% (0.1%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.0% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.5% (0.2%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.0% (0.5%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.4% (0.2%–0.7%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 30 days out (2017-03-24)

Actual matchup: current 78.3% (winner given matchup 99.9%), fitted 70.7% (winner given matchup 99.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 25.3% (18.8%–32.3%) | 89.4% / 85.4% ✓ | 89.0% / 84.5% ✓ |
| le_pen | 21.3% | 25.0% (18.6%–31.8%) | 88.7% / 84.8% ✓ | 0.5% / 1.3% |
| fillon | 20.0% | 17.9% (12.9%–23.3%) | 19.1% / 23.6% | 9.1% / 11.3% |
| melenchon | 19.6% | 12.6% (8.4%–17.2%) | 1.8% / 3.6% | 0.9% / 1.9% |
| hamon | 6.4% | 11.8% (7.8%–16.3%) | 1.0% / 2.6% | 0.4% / 1.1% |
| dupont_aignan | 4.7% | 3.8% (2.5%–5.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 1.0% (0.5%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.5%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 7 days out (2017-04-16)

Actual matchup: current 50.0% (winner given matchup 99.9%), fitted 42.2% (winner given matchup 99.8%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 22.9% (18.0%–28.1%) | 77.1% / 72.0% ✓ | 76.9% / 71.3% ✓ |
| le_pen | 21.3% | 22.2% (17.4%–27.1%) | 70.4% / 66.2% ✓ | 0.2% / 0.7% |
| fillon | 20.0% | 19.1% (14.9%–23.7%) | 26.7% / 31.4% | 10.3% / 11.9% |
| melenchon | 19.6% | 18.8% (14.5%–23.5%) | 25.8% / 30.2% | 12.6% / 16.0% |
| hamon | 6.4% | 8.4% (5.7%–11.3%) | 0.0% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 3.7% (2.6%–4.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 1.1% (0.7%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.6% (1.1%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.8% (0.5%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.5%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 182 days out (2021-10-10)

Actual matchup: current 53.6% (winner given matchup 93.3%), fitted 49.1% (winner given matchup 90.8%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 25.8% (17.0%–35.3%) | 91.2% / 89.4% ✓ | 85.6% / 82.0% ✓ |
| le_pen | 23.1% | 17.7% (10.8%–25.5%) | 61.8% / 58.7% ✓ | 8.1% / 9.7% |
| melenchon | 21.9% | 8.3% (4.6%–12.6%) | 2.7% / 3.6% | 0.3% / 0.5% |
| zemmour | 7.1% | 14.5% (8.4%–21.5%) | 29.0% / 30.7% | 3.4% / 4.2% |
| pecresse | 4.8% | 11.9% (7.3%–17.1%) | 13.5% / 14.8% | 2.2% / 2.9% |
| jadot | 4.6% | 7.4% (4.0%–11.5%) | 1.7% / 2.4% | 0.4% / 0.6% |
| lassalle | 3.1% | 1.2% (0.5%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.2% (1.1%–3.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 3.5% (1.9%–5.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 5.5% (2.9%–8.7%) | 0.2% / 0.4% | 0.1% / 0.1% |
| poutou | 0.8% | 1.1% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.5%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 91 days out (2022-01-09)

Actual matchup: current 33.0% (winner given matchup 97.6%), fitted 31.0% (winner given matchup 95.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 25.4% (16.4%–35.1%) | 88.7% / 86.6% ✓ | 82.8% / 78.9% ✓ |
| le_pen | 23.1% | 16.2% (9.5%–23.8%) | 41.6% / 41.0% ✓ | 4.9% / 5.9% |
| melenchon | 21.9% | 10.7% (6.0%–16.0%) | 8.7% / 9.7% | 0.9% / 1.3% |
| zemmour | 7.1% | 13.3% (7.4%–20.0%) | 19.3% / 20.9% | 2.3% / 2.8% |
| pecresse | 4.8% | 16.0% (9.8%–23.1%) | 41.0% / 40.5% | 8.9% / 10.7% |
| jadot | 4.6% | 6.9% (3.5%–10.8%) | 0.8% / 1.3% | 0.2% / 0.3% |
| lassalle | 3.1% | 1.2% (0.5%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.4% (1.1%–3.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.4% (1.3%–3.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 3.7% (1.8%–6.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.2% (0.6%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 30 days out (2022-03-11)

Actual matchup: current 70.5% (winner given matchup 99.4%), fitted 61.9% (winner given matchup 98.3%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 30.5% (23.0%–38.3%) | 99.7% / 98.9% ✓ | 99.2% / 97.7% ✓ |
| le_pen | 23.1% | 16.8% (11.6%–22.4%) | 70.9% / 62.9% ✓ | 0.7% / 1.6% |
| melenchon | 21.9% | 11.6% (7.9%–15.7%) | 10.0% / 12.2% | 0.0% / 0.1% |
| zemmour | 7.1% | 12.4% (8.0%–17.2%) | 9.7% / 14.1% | 0.1% / 0.3% |
| pecresse | 4.8% | 11.9% (8.4%–15.9%) | 9.8% / 11.8% | 0.1% / 0.2% |
| jadot | 4.6% | 5.4% (3.3%–7.8%) | 0.0% / 0.1% | 0.0% / 0.0% |
| lassalle | 3.1% | 1.9% (1.1%–2.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.6% (2.2%–5.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.0% (1.2%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.1% (1.2%–3.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.0% (0.6%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 7 days out (2022-04-03)

Actual matchup: current 92.2% (winner given matchup 97.2%), fitted 83.8% (winner given matchup 93.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 27.1% (21.5%–33.0%) | 99.6% / 98.1% ✓ | 97.0% / 92.8% ✓ |
| le_pen | 23.1% | 20.9% (16.0%–26.1%) | 92.7% / 85.6% ✓ | 2.8% / 6.2% |
| melenchon | 21.9% | 14.8% (11.1%–18.7%) | 7.6% / 14.2% | 0.2% / 0.8% |
| zemmour | 7.1% | 10.9% (7.5%–14.5%) | 0.1% / 1.7% | 0.0% / 0.1% |
| pecresse | 4.8% | 9.4% (7.0%–12.0%) | 0.1% / 0.4% | 0.0% / 0.0% |
| jadot | 4.6% | 5.1% (3.4%–7.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 3.1% | 2.5% (1.7%–3.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.2% (2.1%–4.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.1% (1.4%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.0% (1.3%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.2% (0.8%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
