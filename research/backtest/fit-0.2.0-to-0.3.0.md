# Backtest: 2002, 2007, 2012, 2017 and 2022 replays

Written by `fr2027 backtest` (moon task `cli:backtest`) from `config/backtest.yaml`, `config/model.yaml` (model 0.2.0) and `data/backtest/`; `results.json` beside this file has every number. Seed 20170423, 20000 simulations per replay and setting. How to read it and what was decided: `research/LOG.md`.

Each replay fits the model to the polls published by its date (fieldwork started at most 365 days before round 1; overlapping polls of one firm thinned), conditions on the field that actually ran, and simulates both rounds. Probabilities are clipped to [0.001, 0.999] for log-loss. Share log scores are summed over contenders (fitted share ≥ 10%) and replays; higher is better. Brier, log-loss and MAE: lower is better.

## Settings

Horizon errors are per square root of 30 days before the round; they add in variance to the election-day error.

| | Current (config/model.yaml) | Fitted |
|---|---|---|
| round1_error.bloc_sd | 0.18 | 0.18 |
| round1_error.candidate_sd | 0.2 | 0.2 |
| round1_error.horizon_bloc_sd | 0 | 0 |
| round1_error.horizon_candidate_sd | 0 | 0.2 |
| round1_error.student_t_df | 5 | 3 |
| round2.error_sd | 0.12 | 0.12 |
| round2.horizon_error_sd | 0 | 0 |
## Scores

All replays (every election, every horizon):

| | Qualify Brier | Qualify log-loss | Win Brier | Win log-loss | Share MAE (pts) | 80% coverage | Share log score | Round 2 log score | P(winner \| matchup) |
|---|---|---|---|---|---|---|---|---|---|
| Latest-poll average | 0.049 | 0.341 | 0.025 | 0.171 | 2.19 | – | – | – | 100.0% |
| Model, current | 0.048 | 0.173 | 0.021 | 0.073 | 2.09 | 66.9% | -8.1 | -8.06 | 86.6% |
| Model, fitted | 0.047 | 0.165 | 0.021 | 0.073 | 2.08 | 68.6% | -7.0 | -7.74 | 86.2% |

By election:

| | Qualify Brier | Qualify log-loss | Win Brier | Win log-loss | Share MAE (pts) | 80% coverage | Share log score | Round 2 log score | P(winner \| matchup) |
|---|---|---|---|---|---|---|---|---|---|
| 2002, baseline | 0.125 | 0.864 | 0.031 | 0.217 | 2.41 | – | – | – | – |
| 2007, baseline | 0.000 | 0.001 | 0.000 | 0.001 | 1.41 | – | – | – | 100.0% |
| 2012, baseline | 0.000 | 0.001 | 0.000 | 0.001 | 1.48 | – | – | – | 100.0% |
| 2017, baseline | 0.091 | 0.629 | 0.091 | 0.629 | 2.41 | – | – | – | 100.0% |
| 2022, baseline | 0.000 | 0.001 | 0.000 | 0.001 | 3.03 | – | – | – | 100.0% |
| 2002, current | 0.113 | 0.408 | 0.026 | 0.076 | 2.30 | 61.7% | -1.6 | -13.10 | 86.7% |
| 2007, current | 0.004 | 0.025 | 0.027 | 0.090 | 1.43 | 74.5% | 9.1 | 3.82 | 67.3% |
| 2012, current | 0.015 | 0.060 | 0.002 | 0.023 | 1.64 | 68.4% | 5.3 | -0.32 | 95.3% |
| 2017, current | 0.060 | 0.183 | 0.047 | 0.150 | 1.77 | 72.1% | 1.2 | -1.25 | 89.1% |
| 2022, current | 0.023 | 0.092 | 0.002 | 0.022 | 3.11 | 60.4% | -22.1 | 2.79 | 94.5% |
| 2002, fitted | 0.109 | 0.378 | 0.026 | 0.078 | 2.28 | 63.3% | -1.4 | -12.77 | 83.7% |
| 2007, fitted | 0.004 | 0.023 | 0.026 | 0.087 | 1.43 | 72.3% | 9.4 | 3.82 | 67.4% |
| 2012, fitted | 0.015 | 0.063 | 0.002 | 0.026 | 1.61 | 73.7% | 5.3 | -0.32 | 95.3% |
| 2017, fitted | 0.059 | 0.182 | 0.047 | 0.151 | 1.78 | 69.8% | 1.6 | -1.25 | 89.8% |
| 2022, fitted | 0.023 | 0.092 | 0.002 | 0.021 | 3.12 | 66.7% | -22.0 | 2.78 | 94.7% |

By horizon (days before round 1):

| | Qualify Brier | Qualify log-loss | Win Brier | Win log-loss | Share MAE (pts) | 80% coverage | Share log score | Round 2 log score | P(winner \| matchup) |
|---|---|---|---|---|---|---|---|---|---|
| 182, baseline | 0.066 | 0.454 | 0.033 | 0.227 | 3.21 | – | – | – | 100.0% |
| 91, baseline | 0.066 | 0.454 | 0.033 | 0.227 | 2.60 | – | – | – | 100.0% |
| 30, baseline | 0.033 | 0.227 | 0.033 | 0.227 | 1.93 | – | – | – | 100.0% |
| 7, baseline | 0.033 | 0.227 | 0.000 | 0.001 | 1.18 | – | – | – | 100.0% |
| 182, current | 0.058 | 0.203 | 0.033 | 0.108 | 2.75 | 63.6% | -2.3 | -4.11 | 76.7% |
| 91, current | 0.062 | 0.209 | 0.024 | 0.080 | 2.57 | 69.5% | -8.3 | -0.86 | 87.6% |
| 30, current | 0.039 | 0.164 | 0.017 | 0.056 | 1.91 | 60.7% | -5.4 | -0.49 | 91.6% |
| 7, current | 0.034 | 0.115 | 0.011 | 0.048 | 1.21 | 73.8% | 7.8 | -2.60 | 90.4% |
| 182, fitted | 0.056 | 0.192 | 0.034 | 0.111 | 2.72 | 72.7% | -2.4 | -3.78 | 77.5% |
| 91, fitted | 0.060 | 0.196 | 0.025 | 0.081 | 2.56 | 72.9% | -7.6 | -0.83 | 86.8% |
| 30, fitted | 0.037 | 0.155 | 0.016 | 0.054 | 1.91 | 62.3% | -4.9 | -0.54 | 90.3% |
| 7, fitted | 0.034 | 0.116 | 0.011 | 0.046 | 1.21 | 67.2% | 7.9 | -2.59 | 90.1% |

## Reliability

Qualification and win events pooled over every replay.

| Forecast | Events (current) | Mean forecast | Observed | Events (fitted) | Mean forecast | Observed |
|---|---|---|---|---|---|---|
| 0–10% | 391 | 0.5% | 1.0% | 391 | 0.6% | 1.3% |
| 10–20% | 16 | 13.6% | 12.5% | 16 | 12.6% | 6.2% |
| 20–30% | 5 | 24.2% | 0.0% | 8 | 25.8% | 0.0% |
| 30–40% | 9 | 33.2% | 22.2% | 7 | 34.3% | 28.6% |
| 40–50% | 10 | 43.3% | 30.0% | 9 | 43.9% | 33.3% |
| 50–60% | 9 | 57.3% | 66.7% | 9 | 56.4% | 66.7% |
| 60–70% | 4 | 65.6% | 100.0% | 4 | 65.8% | 100.0% |
| 70–80% | 7 | 73.8% | 85.7% | 7 | 75.2% | 85.7% |
| 80–90% | 10 | 85.8% | 100.0% | 13 | 86.7% | 84.6% |
| 90–100% | 27 | 95.9% | 85.2% | 24 | 95.7% | 91.7% |

## Fit

The rule, in each stage: among the grid points that score at least as well as the stage's starting setting on every source of evidence (each election's replays, and the final-week polls of other elections: none), take the one with the highest total log score; if none does, keep the starting setting. The first stage starts from config/model.yaml and each later one from the setting the previous one chose; the three stages repeat until a pass changes nothing (2 passes here, at most 4). Every stage keeps or improves every source, so the fitted setting scores at least as well as the current one on all of them. Round 1 stages score contenders' shares; the round 2 stage scores the realised runoff share given the actual matchup. Final-week polls are taken as election-eve polls, with no horizon error.


### Pass 1: Round 1 horizon error and tails

Evidence: share log score of the contenders. Starts from horizon_bloc_sd 0, horizon_candidate_sd 0, student_t_df 5.

| Evidence | Start | Chosen | Best on this source alone |
|---|---|---|---|
| 2002 replays | -1.61 | -1.36 | -0.90 (horizon_bloc_sd 0, horizon_candidate_sd 0.2, student_t_df 5) |
| 2007 replays | 9.05 | 9.44 | 10.08 (horizon_bloc_sd 0, horizon_candidate_sd 0.1, student_t_df 2.5) |
| 2012 replays | 5.27 | 5.28 | 6.23 (horizon_bloc_sd 0, horizon_candidate_sd 0, student_t_df 2.5) |
| 2017 replays | 1.25 | 1.59 | 2.33 (horizon_bloc_sd 0.07, horizon_candidate_sd 0.4, student_t_df 2.5) |
| 2022 replays | -22.09 | -21.97 | -14.16 (horizon_bloc_sd 0.27, horizon_candidate_sd 0.2, student_t_df 10) |
| 182-day replays, every election (not used) | -2.26 | -2.39 | -2.03 (horizon_bloc_sd 0, horizon_candidate_sd 0.14, student_t_df 5) |
| 91-day replays, every election (not used) | -8.32 | -7.60 | -6.64 (horizon_bloc_sd 0.07, horizon_candidate_sd 0.33, student_t_df 3) |
| 30-day replays, every election (not used) | -5.38 | -4.89 | -3.68 (horizon_bloc_sd 0.2, horizon_candidate_sd 0.4, student_t_df 3) |
| 7-day replays, every election (not used) | 7.83 | 7.87 | 8.21 (horizon_bloc_sd 0, horizon_candidate_sd 0.4, student_t_df 2.5) |
| Total | -8.13 | -7.01 | -5.51 (horizon_bloc_sd 0.07, horizon_candidate_sd 0.4, student_t_df 2.5) |

Total log score by `horizon_bloc_sd` (rows) and `horizon_candidate_sd` (columns). `*` marks the points that score at least as well as the start on every source; bold is the chosen one.

student_t_df = 2.5:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -9.2 | -9.2 | -9.0 | -8.7 | -8.4 | -7.8 | -7.2 | -6.7 | -6.3 | -5.9 |
| 0.02 | -9.1 | -9.0 | -8.9 | -8.6 | -8.2 | -7.7 | -7.1 | -6.6 | -6.2 | -5.8 |
| 0.04 | -8.7 | -8.6 | -8.5 | -8.2 | -7.9 | -7.3 | -6.7 | -6.3 | -5.9 | -5.5 |
| 0.07 | -8.4 | -8.4 | -8.3 | -8.0 | -7.7 | -7.2 | -6.6 | -6.2 | -5.8 | -5.5 |
| 0.1 | -8.6 | -8.5 | -8.4 | -8.2 | -8.0 | -7.6 | -7.0 | -6.6 | -6.2 | -6.0 |
| 0.14 | -8.3 | -8.2 | -8.2 | -8.0 | -7.8 | -7.6 | -7.1 | -6.8 | -6.5 | -6.3 |
| 0.2 | -9.1 | -9.1 | -9.1 | -8.9 | -8.9 | -8.7 | -8.4 | -8.1 | -8.0 | -7.9 |
| 0.27 | -10.9 | -10.9 | -10.9 | -10.8 | -10.8 | -10.7 | -10.4 | -10.3 | -10.2 | -10.2 |

student_t_df = 3:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -9.1 | -9.0 | -8.9 | -8.5 | -8.0 | -7.7 | **-7.0*** | -6.6 | -6.5 | -6.6 |
| 0.02 | -8.9 | -8.9 | -8.7 | -8.3 | -7.9 | -7.6 | -6.9 | -6.5 | -6.4 | -6.5 |
| 0.04 | -8.5 | -8.4 | -8.3 | -7.9 | -7.4 | -7.1 | -6.5 | -6.1 | -6.0 | -6.1 |
| 0.07 | -7.8 | -7.8 | -7.6 | -7.3 | -6.9 | -6.7 | -6.0 | -5.7 | -5.6 | -5.7 |
| 0.1 | -8.1 | -8.1 | -8.0 | -7.7 | -7.4 | -7.2 | -6.6 | -6.3 | -6.2 | -6.4 |
| 0.14 | -8.4 | -8.4 | -8.3 | -8.1 | -7.8 | -7.6 | -7.2 | -7.0 | -7.0 | -7.3 |
| 0.2 | -9.7 | -9.7 | -9.6 | -9.5 | -9.4 | -9.3 | -9.1 | -9.0 | -9.1 | -9.4 |
| 0.27 | -12.8 | -12.8 | -12.8 | -12.7 | -12.7 | -12.6 | -12.5 | -12.6 | -12.8 | -13.3 |

student_t_df = 4:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -8.8 | -8.8 | -8.7 | -8.4 | -8.0 | -7.5* | -7.0 | -7.0 | -7.1 | -7.5 |
| 0.02 | -8.8 | -8.8 | -8.7 | -8.4 | -8.0 | -7.5 | -7.0 | -7.0 | -7.2 | -7.5 |
| 0.04 | -8.8 | -8.7 | -8.6 | -8.3 | -7.9 | -7.5 | -7.0 | -7.0 | -7.2 | -7.6 |
| 0.07 | -9.0 | -8.9 | -8.8 | -8.6 | -8.3 | -7.9 | -7.5 | -7.5 | -7.8 | -8.2 |
| 0.1 | -8.6 | -8.6 | -8.5 | -8.3 | -8.1 | -7.8 | -7.6 | -7.7 | -8.0 | -8.4 |
| 0.14 | -8.7 | -8.7 | -8.6 | -8.5 | -8.4 | -8.2 | -8.1 | -8.4 | -8.7 | -9.1 |
| 0.2 | -11.3 | -11.3 | -11.2 | -11.1 | -11.0 | -11.0 | -11.0 | -11.3 | -11.7 | -12.2 |
| 0.27 | -15.2 | -15.2 | -15.2 | -15.1 | -15.1 | -15.0 | -15.2 | -15.5 | -16.0 | -16.6 |

student_t_df = 5:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -8.1* | -8.1 | -7.9 | -7.7 | -7.4 | -7.0 | -6.5 | -6.5 | -6.9 | -7.5 |
| 0.02 | -8.0 | -8.0 | -7.8 | -7.6 | -7.3 | -6.9 | -6.5 | -6.4 | -6.8 | -7.5 |
| 0.04 | -7.9 | -7.9 | -7.8 | -7.5 | -7.3 | -6.9 | -6.5 | -6.5 | -6.9 | -7.5 |
| 0.07 | -7.8 | -7.8 | -7.7 | -7.5 | -7.3 | -7.0 | -6.7 | -6.7 | -7.1 | -7.8 |
| 0.1 | -8.2 | -8.2 | -8.1 | -7.9 | -7.6 | -7.4 | -7.2 | -7.2 | -7.7 | -8.4 |
| 0.14 | -9.1 | -9.1 | -9.0 | -8.9 | -8.7 | -8.6 | -8.5 | -8.6 | -9.1 | -9.8 |
| 0.2 | -11.6 | -11.6 | -11.6 | -11.5 | -11.4 | -11.3 | -11.4 | -11.8 | -12.3 | -13.1 |
| 0.27 | -16.0 | -16.0 | -16.0 | -15.9 | -15.9 | -16.0 | -16.2 | -16.8 | -17.3 | -18.1 |

student_t_df = 10:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -9.6 | -9.6 | -9.5 | -9.3 | -8.9 | -8.4 | -7.9 | -8.0 | -8.3 | -9.0 |
| 0.02 | -9.5 | -9.5 | -9.4 | -9.1 | -8.7 | -8.3 | -7.8 | -7.9 | -8.2 | -8.9 |
| 0.04 | -9.1 | -9.1 | -9.0 | -8.8 | -8.4 | -7.9 | -7.5 | -7.6 | -7.9 | -8.6 |
| 0.07 | -8.8 | -8.8 | -8.7 | -8.4 | -8.0 | -7.6 | -7.2 | -7.4 | -7.8 | -8.5 |
| 0.1 | -8.8 | -8.8 | -8.7 | -8.4 | -8.1 | -7.8 | -7.6 | -7.8 | -8.3 | -9.0 |
| 0.14 | -10.3 | -10.3 | -10.3 | -10.1 | -9.9 | -9.7 | -9.6 | -9.9 | -10.4 | -11.2 |
| 0.2 | -12.7 | -12.7 | -12.6 | -12.6 | -12.6 | -12.4 | -12.5 | -12.8 | -13.3 | -14.2 |
| 0.27 | -17.2 | -17.2 | -17.2 | -17.2 | -17.2 | -17.2 | -17.3 | -17.8 | -18.4 | -19.3 |

### Pass 1: Round 1 election-day error

Evidence: share log score of the contenders. Starts from bloc_sd 0.18, candidate_sd 0.2.

| Evidence | Start | Chosen | Best on this source alone |
|---|---|---|---|
| 2002 replays | -1.36 | -1.36 | -1.25 (bloc_sd 0.22, candidate_sd 0.13) |
| 2007 replays | 9.44 | 9.44 | 10.75 (bloc_sd 0.08, candidate_sd 0.03) |
| 2012 replays | 5.28 | 5.28 | 6.45 (bloc_sd 0.06, candidate_sd 0.07) |
| 2017 replays | 1.59 | 1.59 | 2.99 (bloc_sd 0.06, candidate_sd 0.4) |
| 2022 replays | -21.97 | -21.97 | -15.25 (bloc_sd 0.4, candidate_sd 0.4) |
| 182-day replays, every election (not used) | -2.39 | -2.39 | -2.17 (bloc_sd 0.15, candidate_sd 0.2) |
| 91-day replays, every election (not used) | -7.60 | -7.60 | -6.64 (bloc_sd 0.22, candidate_sd 0.4) |
| 30-day replays, every election (not used) | -4.89 | -4.89 | -3.67 (bloc_sd 0.27, candidate_sd 0.4) |
| 7-day replays, every election (not used) | 7.87 | 7.87 | 8.34 (bloc_sd 0.1, candidate_sd 0.3) |
| Total | -7.01 | -7.01 | -5.93 (bloc_sd 0.22, candidate_sd 0.3) |

Total log score by `bloc_sd` (rows) and `candidate_sd` (columns). `*` marks the points that score at least as well as the start on every source; bold is the chosen one.

| bloc_sd \ candidate_sd | 0.03 | 0.05 | 0.07 | 0.1 | 0.13 | 0.16 | 0.2 | 0.25 | 0.3 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0.04 | -29.6 | -29.1 | -28.2 | -26.8 | -25.8 | -25.1 | -23.9 | -22.5 | -21.7 | -20.4 |
| 0.06 | -25.3 | -24.9 | -24.3 | -23.5 | -23.1 | -22.6 | -21.7 | -20.4 | -19.6 | -18.4 |
| 0.08 | -17.9 | -17.6 | -17.2 | -16.5 | -15.9 | -15.5 | -14.8 | -13.8 | -13.1 | -12.2 |
| 0.1 | -12.2 | -12.0 | -11.7 | -11.0 | -10.5 | -10.0 | -9.3 | -8.5 | -7.9 | -7.2 |
| 0.12 | -9.8 | -9.6 | -9.3 | -8.8 | -8.4 | -8.1 | -7.6 | -7.1 | -6.6 | -6.1 |
| 0.15 | -9.4 | -9.3 | -9.1 | -8.7 | -8.4 | -8.1 | -7.8 | -7.3 | -7.0 | -6.6 |
| 0.18 | -8.4 | -8.3 | -8.1 | -7.8 | -7.5 | -7.3 | **-7.0*** | -6.7 | -6.5 | -6.3 |
| 0.22 | -7.3 | -7.2 | -7.1 | -6.9 | -6.7 | -6.6 | -6.3 | -6.0 | -5.9 | -6.0 |
| 0.27 | -8.0 | -8.0 | -7.9 | -7.8 | -7.8 | -7.7 | -7.5 | -7.4 | -7.4 | -7.6 |
| 0.33 | -10.0 | -10.0 | -10.0 | -9.9 | -9.9 | -9.8 | -9.8 | -9.8 | -9.9 | -10.3 |
| 0.4 | -13.4 | -13.4 | -13.4 | -13.4 | -13.4 | -13.4 | -13.5 | -13.6 | -13.7 | -14.2 |

### Pass 1: Round 2 errors

Evidence: log score of the realised runoff share. Starts from error_sd 0.12, horizon_error_sd 0.

| Evidence | Start | Chosen | Best on this source alone |
|---|---|---|---|
| 2002 replays | -12.77 | -12.77 | -8.72 (error_sd 0.4, horizon_error_sd 0.2) |
| 2007 replays | 3.82 | 3.82 | 5.21 (error_sd 0.02, horizon_error_sd 0) |
| 2012 replays | -0.32 | -0.32 | 0.70 (error_sd 0.08, horizon_error_sd 0.14) |
| 2017 replays | -1.25 | -1.25 | -0.50 (error_sd 0.22, horizon_error_sd 0.04) |
| 2022 replays | 2.78 | 2.78 | 2.78 (error_sd 0.1, horizon_error_sd 0) |
| 182-day replays, every election (not used) | -3.78 | -3.78 | -3.14 (error_sd 0.18, horizon_error_sd 0.1) |
| 91-day replays, every election (not used) | -0.83 | -0.83 | -0.80 (error_sd 0.12, horizon_error_sd 0.04) |
| 30-day replays, every election (not used) | -0.54 | -0.54 | -0.53 (error_sd 0.08, horizon_error_sd 0.07) |
| 7-day replays, every election (not used) | -2.59 | -2.59 | -2.07 (error_sd 0.18, horizon_error_sd 0.1) |
| Total | -7.74 | -7.74 | -7.02 (error_sd 0.15, horizon_error_sd 0.07) |

Total log score by `error_sd` (rows) and `horizon_error_sd` (columns). `*` marks the points that score at least as well as the start on every source; bold is the chosen one.

| error_sd \ horizon_error_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 |
|---|---|---|---|---|---|---|---|
| 0.02 | -15.0 | -14.1 | -12.1 | -9.5 | -8.1 | -7.7 | -8.5 |
| 0.03 | -14.1 | -13.3 | -11.6 | -9.2 | -8.0 | -7.7 | -8.5 |
| 0.05 | -12.1 | -11.5 | -10.3 | -8.6 | -7.7 | -7.6 | -8.6 |
| 0.08 | -9.6 | -9.3 | -8.6 | -7.7 | -7.4 | -7.5 | -8.6 |
| 0.1 | -8.5 | -8.3 | -7.9 | -7.3 | -7.2 | -7.5 | -8.7 |
| 0.12 | **-7.7*** | -7.6 | -7.4 | -7.1 | -7.1 | -7.6 | -8.8 |
| 0.15 | -7.2 | -7.2 | -7.1 | -7.0 | -7.2 | -7.7 | -9.0 |
| 0.18 | -7.1 | -7.1 | -7.1 | -7.2 | -7.4 | -8.0 | -9.2 |
| 0.22 | -7.4 | -7.4 | -7.4 | -7.5 | -7.8 | -8.4 | -9.6 |
| 0.27 | -8.0 | -8.0 | -8.0 | -8.2 | -8.5 | -9.0 | -10.1 |
| 0.33 | -8.8 | -8.9 | -8.9 | -9.1 | -9.4 | -9.9 | -10.8 |
| 0.4 | -10.0 | -10.0 | -10.0 | -10.2 | -10.4 | -10.9 | -11.7 |

### Pass 2: Round 1 horizon error and tails

Evidence: share log score of the contenders. Starts from horizon_bloc_sd 0, horizon_candidate_sd 0.2, student_t_df 3.

| Evidence | Start | Chosen | Best on this source alone |
|---|---|---|---|
| 2002 replays | -1.36 | -1.36 | -0.90 (horizon_bloc_sd 0, horizon_candidate_sd 0.2, student_t_df 5) |
| 2007 replays | 9.44 | 9.44 | 10.08 (horizon_bloc_sd 0, horizon_candidate_sd 0.1, student_t_df 2.5) |
| 2012 replays | 5.28 | 5.28 | 6.23 (horizon_bloc_sd 0, horizon_candidate_sd 0, student_t_df 2.5) |
| 2017 replays | 1.59 | 1.59 | 2.33 (horizon_bloc_sd 0.07, horizon_candidate_sd 0.4, student_t_df 2.5) |
| 2022 replays | -21.97 | -21.97 | -14.16 (horizon_bloc_sd 0.27, horizon_candidate_sd 0.2, student_t_df 10) |
| 182-day replays, every election (not used) | -2.39 | -2.39 | -2.03 (horizon_bloc_sd 0, horizon_candidate_sd 0.14, student_t_df 5) |
| 91-day replays, every election (not used) | -7.60 | -7.60 | -6.64 (horizon_bloc_sd 0.07, horizon_candidate_sd 0.33, student_t_df 3) |
| 30-day replays, every election (not used) | -4.89 | -4.89 | -3.68 (horizon_bloc_sd 0.2, horizon_candidate_sd 0.4, student_t_df 3) |
| 7-day replays, every election (not used) | 7.87 | 7.87 | 8.21 (horizon_bloc_sd 0, horizon_candidate_sd 0.4, student_t_df 2.5) |
| Total | -7.01 | -7.01 | -5.51 (horizon_bloc_sd 0.07, horizon_candidate_sd 0.4, student_t_df 2.5) |

Total log score by `horizon_bloc_sd` (rows) and `horizon_candidate_sd` (columns). `*` marks the points that score at least as well as the start on every source; bold is the chosen one.

student_t_df = 2.5:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -9.2 | -9.2 | -9.0 | -8.7 | -8.4 | -7.8 | -7.2 | -6.7 | -6.3 | -5.9 |
| 0.02 | -9.1 | -9.0 | -8.9 | -8.6 | -8.2 | -7.7 | -7.1 | -6.6 | -6.2 | -5.8 |
| 0.04 | -8.7 | -8.6 | -8.5 | -8.2 | -7.9 | -7.3 | -6.7 | -6.3 | -5.9 | -5.5 |
| 0.07 | -8.4 | -8.4 | -8.3 | -8.0 | -7.7 | -7.2 | -6.6 | -6.2 | -5.8 | -5.5 |
| 0.1 | -8.6 | -8.5 | -8.4 | -8.2 | -8.0 | -7.6 | -7.0 | -6.6 | -6.2 | -6.0 |
| 0.14 | -8.3 | -8.2 | -8.2 | -8.0 | -7.8 | -7.6 | -7.1 | -6.8 | -6.5 | -6.3 |
| 0.2 | -9.1 | -9.1 | -9.1 | -8.9 | -8.9 | -8.7 | -8.4 | -8.1 | -8.0 | -7.9 |
| 0.27 | -10.9 | -10.9 | -10.9 | -10.8 | -10.8 | -10.7 | -10.4 | -10.3 | -10.2 | -10.2 |

student_t_df = 3:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -9.1 | -9.0 | -8.9 | -8.5 | -8.0 | -7.7 | **-7.0*** | -6.6 | -6.5 | -6.6 |
| 0.02 | -8.9 | -8.9 | -8.7 | -8.3 | -7.9 | -7.6 | -6.9 | -6.5 | -6.4 | -6.5 |
| 0.04 | -8.5 | -8.4 | -8.3 | -7.9 | -7.4 | -7.1 | -6.5 | -6.1 | -6.0 | -6.1 |
| 0.07 | -7.8 | -7.8 | -7.6 | -7.3 | -6.9 | -6.7 | -6.0 | -5.7 | -5.6 | -5.7 |
| 0.1 | -8.1 | -8.1 | -8.0 | -7.7 | -7.4 | -7.2 | -6.6 | -6.3 | -6.2 | -6.4 |
| 0.14 | -8.4 | -8.4 | -8.3 | -8.1 | -7.8 | -7.6 | -7.2 | -7.0 | -7.0 | -7.3 |
| 0.2 | -9.7 | -9.7 | -9.6 | -9.5 | -9.4 | -9.3 | -9.1 | -9.0 | -9.1 | -9.4 |
| 0.27 | -12.8 | -12.8 | -12.8 | -12.7 | -12.7 | -12.6 | -12.5 | -12.6 | -12.8 | -13.3 |

student_t_df = 4:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -8.8 | -8.8 | -8.7 | -8.4 | -8.0 | -7.5 | -7.0 | -7.0 | -7.1 | -7.5 |
| 0.02 | -8.8 | -8.8 | -8.7 | -8.4 | -8.0 | -7.5 | -7.0 | -7.0 | -7.2 | -7.5 |
| 0.04 | -8.8 | -8.7 | -8.6 | -8.3 | -7.9 | -7.5 | -7.0 | -7.0 | -7.2 | -7.6 |
| 0.07 | -9.0 | -8.9 | -8.8 | -8.6 | -8.3 | -7.9 | -7.5 | -7.5 | -7.8 | -8.2 |
| 0.1 | -8.6 | -8.6 | -8.5 | -8.3 | -8.1 | -7.8 | -7.6 | -7.7 | -8.0 | -8.4 |
| 0.14 | -8.7 | -8.7 | -8.6 | -8.5 | -8.4 | -8.2 | -8.1 | -8.4 | -8.7 | -9.1 |
| 0.2 | -11.3 | -11.3 | -11.2 | -11.1 | -11.0 | -11.0 | -11.0 | -11.3 | -11.7 | -12.2 |
| 0.27 | -15.2 | -15.2 | -15.2 | -15.1 | -15.1 | -15.0 | -15.2 | -15.5 | -16.0 | -16.6 |

student_t_df = 5:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -8.1 | -8.1 | -7.9 | -7.7 | -7.4 | -7.0 | -6.5 | -6.5 | -6.9 | -7.5 |
| 0.02 | -8.0 | -8.0 | -7.8 | -7.6 | -7.3 | -6.9 | -6.5 | -6.4 | -6.8 | -7.5 |
| 0.04 | -7.9 | -7.9 | -7.8 | -7.5 | -7.3 | -6.9 | -6.5 | -6.5 | -6.9 | -7.5 |
| 0.07 | -7.8 | -7.8 | -7.7 | -7.5 | -7.3 | -7.0 | -6.7 | -6.7 | -7.1 | -7.8 |
| 0.1 | -8.2 | -8.2 | -8.1 | -7.9 | -7.6 | -7.4 | -7.2 | -7.2 | -7.7 | -8.4 |
| 0.14 | -9.1 | -9.1 | -9.0 | -8.9 | -8.7 | -8.6 | -8.5 | -8.6 | -9.1 | -9.8 |
| 0.2 | -11.6 | -11.6 | -11.6 | -11.5 | -11.4 | -11.3 | -11.4 | -11.8 | -12.3 | -13.1 |
| 0.27 | -16.0 | -16.0 | -16.0 | -15.9 | -15.9 | -16.0 | -16.2 | -16.8 | -17.3 | -18.1 |

student_t_df = 10:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -9.6 | -9.6 | -9.5 | -9.3 | -8.9 | -8.4 | -7.9 | -8.0 | -8.3 | -9.0 |
| 0.02 | -9.5 | -9.5 | -9.4 | -9.1 | -8.7 | -8.3 | -7.8 | -7.9 | -8.2 | -8.9 |
| 0.04 | -9.1 | -9.1 | -9.0 | -8.8 | -8.4 | -7.9 | -7.5 | -7.6 | -7.9 | -8.6 |
| 0.07 | -8.8 | -8.8 | -8.7 | -8.4 | -8.0 | -7.6 | -7.2 | -7.4 | -7.8 | -8.5 |
| 0.1 | -8.8 | -8.8 | -8.7 | -8.4 | -8.1 | -7.8 | -7.6 | -7.8 | -8.3 | -9.0 |
| 0.14 | -10.3 | -10.3 | -10.3 | -10.1 | -9.9 | -9.7 | -9.6 | -9.9 | -10.4 | -11.2 |
| 0.2 | -12.7 | -12.7 | -12.6 | -12.6 | -12.6 | -12.4 | -12.5 | -12.8 | -13.3 | -14.2 |
| 0.27 | -17.2 | -17.2 | -17.2 | -17.2 | -17.2 | -17.2 | -17.3 | -17.8 | -18.4 | -19.3 |

### Pass 2: Round 1 election-day error

Evidence: share log score of the contenders. Starts from bloc_sd 0.18, candidate_sd 0.2.

| Evidence | Start | Chosen | Best on this source alone |
|---|---|---|---|
| 2002 replays | -1.36 | -1.36 | -1.25 (bloc_sd 0.22, candidate_sd 0.13) |
| 2007 replays | 9.44 | 9.44 | 10.75 (bloc_sd 0.08, candidate_sd 0.03) |
| 2012 replays | 5.28 | 5.28 | 6.45 (bloc_sd 0.06, candidate_sd 0.07) |
| 2017 replays | 1.59 | 1.59 | 2.99 (bloc_sd 0.06, candidate_sd 0.4) |
| 2022 replays | -21.97 | -21.97 | -15.25 (bloc_sd 0.4, candidate_sd 0.4) |
| 182-day replays, every election (not used) | -2.39 | -2.39 | -2.17 (bloc_sd 0.15, candidate_sd 0.2) |
| 91-day replays, every election (not used) | -7.60 | -7.60 | -6.64 (bloc_sd 0.22, candidate_sd 0.4) |
| 30-day replays, every election (not used) | -4.89 | -4.89 | -3.67 (bloc_sd 0.27, candidate_sd 0.4) |
| 7-day replays, every election (not used) | 7.87 | 7.87 | 8.34 (bloc_sd 0.1, candidate_sd 0.3) |
| Total | -7.01 | -7.01 | -5.93 (bloc_sd 0.22, candidate_sd 0.3) |

Total log score by `bloc_sd` (rows) and `candidate_sd` (columns). `*` marks the points that score at least as well as the start on every source; bold is the chosen one.

| bloc_sd \ candidate_sd | 0.03 | 0.05 | 0.07 | 0.1 | 0.13 | 0.16 | 0.2 | 0.25 | 0.3 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0.04 | -29.6 | -29.1 | -28.2 | -26.8 | -25.8 | -25.1 | -23.9 | -22.5 | -21.7 | -20.4 |
| 0.06 | -25.3 | -24.9 | -24.3 | -23.5 | -23.1 | -22.6 | -21.7 | -20.4 | -19.6 | -18.4 |
| 0.08 | -17.9 | -17.6 | -17.2 | -16.5 | -15.9 | -15.5 | -14.8 | -13.8 | -13.1 | -12.2 |
| 0.1 | -12.2 | -12.0 | -11.7 | -11.0 | -10.5 | -10.0 | -9.3 | -8.5 | -7.9 | -7.2 |
| 0.12 | -9.8 | -9.6 | -9.3 | -8.8 | -8.4 | -8.1 | -7.6 | -7.1 | -6.6 | -6.1 |
| 0.15 | -9.4 | -9.3 | -9.1 | -8.7 | -8.4 | -8.1 | -7.8 | -7.3 | -7.0 | -6.6 |
| 0.18 | -8.4 | -8.3 | -8.1 | -7.8 | -7.5 | -7.3 | **-7.0*** | -6.7 | -6.5 | -6.3 |
| 0.22 | -7.3 | -7.2 | -7.1 | -6.9 | -6.7 | -6.6 | -6.3 | -6.0 | -5.9 | -6.0 |
| 0.27 | -8.0 | -8.0 | -7.9 | -7.8 | -7.8 | -7.7 | -7.5 | -7.4 | -7.4 | -7.6 |
| 0.33 | -10.0 | -10.0 | -10.0 | -9.9 | -9.9 | -9.8 | -9.8 | -9.8 | -9.9 | -10.3 |
| 0.4 | -13.4 | -13.4 | -13.4 | -13.4 | -13.4 | -13.4 | -13.5 | -13.6 | -13.7 | -14.2 |

### Pass 2: Round 2 errors

Evidence: log score of the realised runoff share. Starts from error_sd 0.12, horizon_error_sd 0.

| Evidence | Start | Chosen | Best on this source alone |
|---|---|---|---|
| 2002 replays | -12.77 | -12.77 | -8.72 (error_sd 0.4, horizon_error_sd 0.2) |
| 2007 replays | 3.82 | 3.82 | 5.21 (error_sd 0.02, horizon_error_sd 0) |
| 2012 replays | -0.32 | -0.32 | 0.70 (error_sd 0.08, horizon_error_sd 0.14) |
| 2017 replays | -1.25 | -1.25 | -0.50 (error_sd 0.22, horizon_error_sd 0.04) |
| 2022 replays | 2.78 | 2.78 | 2.78 (error_sd 0.1, horizon_error_sd 0) |
| 182-day replays, every election (not used) | -3.78 | -3.78 | -3.14 (error_sd 0.18, horizon_error_sd 0.1) |
| 91-day replays, every election (not used) | -0.83 | -0.83 | -0.80 (error_sd 0.12, horizon_error_sd 0.04) |
| 30-day replays, every election (not used) | -0.54 | -0.54 | -0.53 (error_sd 0.08, horizon_error_sd 0.07) |
| 7-day replays, every election (not used) | -2.59 | -2.59 | -2.07 (error_sd 0.18, horizon_error_sd 0.1) |
| Total | -7.74 | -7.74 | -7.02 (error_sd 0.15, horizon_error_sd 0.07) |

Total log score by `error_sd` (rows) and `horizon_error_sd` (columns). `*` marks the points that score at least as well as the start on every source; bold is the chosen one.

| error_sd \ horizon_error_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 |
|---|---|---|---|---|---|---|---|
| 0.02 | -15.0 | -14.1 | -12.1 | -9.5 | -8.1 | -7.7 | -8.5 |
| 0.03 | -14.1 | -13.3 | -11.6 | -9.2 | -8.0 | -7.7 | -8.5 |
| 0.05 | -12.1 | -11.5 | -10.3 | -8.6 | -7.7 | -7.6 | -8.6 |
| 0.08 | -9.6 | -9.3 | -8.6 | -7.7 | -7.4 | -7.5 | -8.6 |
| 0.1 | -8.5 | -8.3 | -7.9 | -7.3 | -7.2 | -7.5 | -8.7 |
| 0.12 | **-7.7*** | -7.6 | -7.4 | -7.1 | -7.1 | -7.6 | -8.8 |
| 0.15 | -7.2 | -7.2 | -7.1 | -7.0 | -7.2 | -7.7 | -9.0 |
| 0.18 | -7.1 | -7.1 | -7.1 | -7.2 | -7.4 | -8.0 | -9.2 |
| 0.22 | -7.4 | -7.4 | -7.4 | -7.5 | -7.8 | -8.4 | -9.6 |
| 0.27 | -8.0 | -8.0 | -8.0 | -8.2 | -8.5 | -9.0 | -10.1 |
| 0.33 | -8.8 | -8.9 | -8.9 | -9.1 | -9.4 | -9.9 | -10.8 |
| 0.4 | -10.0 | -10.0 | -10.0 | -10.2 | -10.4 | -10.9 | -11.7 |

## Leave one election out

The whole fit rerun without one election's replays, then that election's replays scored at the setting it gives, and at the current one.

| Held out | Passes | bloc_sd | candidate_sd | horizon_bloc_sd | horizon_candidate_sd | student_t_df | error_sd | horizon_error_sd | Round 1 log score (chosen without / current) | Round 2 log score (chosen without / current) |
|---|---|---|---|---|---|---|---|---|---|---|
| 2002 | 2 | 0.18 | 0.2 | 0.07 | 0.33 | 2.5 | 0.12 | 0 | -1.81 / -1.61 | -12.94 / -13.10 |
| 2007 | 2 | 0.18 | 0.2 | 0 | 0.2 | 3 | 0.12 | 0 | 9.44 / 9.05 | 3.82 / 3.82 |
| 2012 | 2 | 0.18 | 0.2 | 0.02 | 0.33 | 3 | 0.12 | 0 | 4.72 / 5.27 | -0.32 / -0.32 |
| 2017 | 2 | 0.18 | 0.2 | 0 | 0.2 | 3 | 0.12 | 0 | 1.59 / 1.25 | -1.25 / -1.25 |
| 2022 | 2 | 0.18 | 0.2 | 0 | 0.2 | 3 | 0.12 | 0 | -21.97 / -22.09 | 2.78 / 2.79 |

## Replays

| Election | Days out | As of | Polls used | Overlapping dropped | Random walk sd | Design effect | Contenders | Runners not yet polled |
|---|---|---|---|---|---|---|---|---|
| 2002 | 182 | 2001-10-21 | 18 | 0 | 0.02 | 1.5 | jospin, chirac | gluckstein, taubira, saint_josse |
| 2002 | 91 | 2002-01-20 | 37 | 0 | 0.02 | 1 | jospin, chevenement, chirac | saint_josse |
| 2002 | 30 | 2002-03-22 | 66 | 2 | 0.02 | 1 | jospin, chirac | – |
| 2002 | 7 | 2002-04-14 | 86 | 4 | 0.02 | 1 | jospin, chirac, jm_le_pen | – |
| 2007 | 182 | 2006-10-22 | 17 | 0 | 0.004 | 2 | royal, sarkozy, jm_le_pen | schivardi |
| 2007 | 91 | 2007-01-21 | 39 | 0 | 0.012 | 1.5 | royal, sarkozy, jm_le_pen | – |
| 2007 | 30 | 2007-03-23 | 91 | 14 | 0.02 | 1.5 | royal, bayrou, sarkozy, jm_le_pen | – |
| 2007 | 7 | 2007-04-15 | 115 | 28 | 0.02 | 1.5 | royal, bayrou, sarkozy, jm_le_pen | – |
| 2012 | 182 | 2011-10-23 | 31 | 7 | 0.02 | 1.5 | hollande, sarkozy, le_pen | cheminade |
| 2012 | 91 | 2012-01-22 | 60 | 12 | 0.02 | 1.5 | hollande, bayrou, sarkozy, le_pen | cheminade |
| 2012 | 30 | 2012-03-23 | 102 | 48 | 0.02 | 1.5 | melenchon, hollande, bayrou, sarkozy, le_pen | – |
| 2012 | 7 | 2012-04-15 | 126 | 61 | 0.03 | 1 | melenchon, hollande, bayrou, sarkozy, le_pen | – |
| 2017 | 182 | 2016-10-23 | 16 | 0 | 0.012 | 2 | melenchon, macron, fillon, le_pen | hamon |
| 2017 | 91 | 2017-01-22 | 32 | 0 | 0.03 | 2 | melenchon, macron, fillon, le_pen | – |
| 2017 | 30 | 2017-03-24 | 87 | 51 | 0.03 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 7 | 2017-04-16 | 120 | 83 | 0.03 | 2 | melenchon, macron, fillon, le_pen | – |
| 2022 | 182 | 2021-10-10 | 30 | 0 | 0.02 | 2 | macron, pecresse, le_pen, zemmour | – |
| 2022 | 91 | 2022-01-09 | 74 | 4 | 0.03 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 30 | 2022-03-11 | 138 | 61 | 0.03 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 7 | 2022-04-03 | 172 | 110 | 0.03 | 2 | melenchon, macron, le_pen, zemmour | – |

### 2002, 182 days out (2001-10-21)

Actual matchup: current 1.3% (winner given matchup 90.0%), fitted 1.3% (winner given matchup 90.3%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 24.6% (15.5%–34.5%) | 94.6% / 91.0% ✓ | 59.6% / 55.1% ✓ |
| jm_le_pen | 16.9% | 7.6% (4.3%–11.5%) | 1.8% / 1.9% ✓ | 0.2% / 0.2% |
| jospin | 16.2% | 22.8% (12.6%–33.6%) | 94.8% / 84.3% | 37.4% / 34.7% |
| bayrou | 6.8% | 5.1% (2.8%–7.8%) | 0.2% / 0.3% | 0.1% / 0.1% |
| laguiller | 5.7% | 6.4% (3.6%–9.5%) | 0.6% / 0.7% | 0.1% / 0.1% |
| chevenement | 5.3% | 10.0% (4.2%–16.8%) | 6.2% / 12.2% | 2.1% / 5.7% |
| mamere | 5.2% | 7.0% (2.7%–12.3%) | 0.9% / 4.2% | 0.3% / 2.1% |
| besancenot | 4.2% | 1.0% (0.3%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| madelin | 3.9% | 5.5% (1.9%–10.2%) | 0.5% / 2.7% | 0.1% / 0.9% |
| hue | 3.4% | 6.0% (2.3%–10.6%) | 0.4% / 2.6% | 0.2% / 1.2% |
| megret | 2.3% | 1.6% (0.5%–3.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 1.1% (0.3%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 1.2% (0.3%–2.3%) | 0.0% / 0.1% | 0.0% / 0.1% |
| gluckstein | 0.5% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 91 days out (2002-01-20)

Actual matchup: current 2.0% (winner given matchup 90.0%), fitted 2.0% (winner given matchup 86.3%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 25.4% (18.1%–33.3%) | 97.3% / 95.9% ✓ | 65.3% / 64.8% ✓ |
| jm_le_pen | 16.9% | 9.2% (6.1%–12.7%) | 2.3% / 2.4% ✓ | 0.2% / 0.3% |
| jospin | 16.2% | 21.5% (13.8%–29.6%) | 94.7% / 87.7% | 33.9% / 32.4% |
| bayrou | 6.8% | 4.2% (2.7%–6.0%) | 0.0% / 0.1% | 0.0% / 0.0% |
| laguiller | 5.7% | 6.0% (3.8%–8.6%) | 0.1% / 0.3% | 0.0% / 0.1% |
| chevenement | 5.3% | 10.8% (5.9%–16.3%) | 5.0% / 9.8% | 0.3% / 0.6% |
| mamere | 5.2% | 7.3% (3.7%–11.3%) | 0.4% / 2.1% | 0.2% / 1.1% |
| besancenot | 4.2% | 0.6% (0.2%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| madelin | 3.9% | 4.7% (2.1%–7.8%) | 0.1% / 0.8% | 0.0% / 0.3% |
| hue | 3.4% | 5.7% (2.9%–8.9%) | 0.1% / 0.9% | 0.1% / 0.4% |
| megret | 2.3% | 1.6% (0.6%–2.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.6% (0.2%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 0.9% (0.4%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 0.8% (0.3%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.7% (0.2%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 30 days out (2002-03-22)

Actual matchup: current 1.5% (winner given matchup 86.2%), fitted 1.4% (winner given matchup 79.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 23.4% (18.2%–28.9%) | 99.0% / 98.4% ✓ | 41.0% / 41.6% ✓ |
| jm_le_pen | 16.9% | 9.6% (7.1%–12.3%) | 1.7% / 1.7% ✓ | 0.2% / 0.3% |
| jospin | 16.2% | 20.3% (14.9%–25.8%) | 96.8% / 94.9% | 58.2% / 57.2% |
| bayrou | 6.8% | 3.8% (2.7%–4.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 5.7% | 8.8% (6.4%–11.4%) | 1.0% / 1.1% | 0.3% / 0.3% |
| chevenement | 5.3% | 9.1% (5.9%–12.5%) | 1.3% / 2.9% | 0.1% / 0.2% |
| mamere | 5.2% | 5.5% (3.5%–7.7%) | 0.1% / 0.2% | 0.0% / 0.1% |
| besancenot | 4.2% | 0.9% (0.5%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | 5.5% (3.2%–8.1%) | 0.2% / 0.3% | 0.1% / 0.1% |
| madelin | 3.9% | 3.5% (2.0%–5.2%) | 0.0% / 0.1% | 0.0% / 0.0% |
| hue | 3.4% | 5.3% (3.3%–7.4%) | 0.0% / 0.3% | 0.0% / 0.1% |
| megret | 2.3% | 1.4% (0.8%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 0.9% (0.5%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 0.9% (0.5%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 7 days out (2002-04-14)

Actual matchup: current 8.6% (winner given matchup 80.5%), fitted 7.1% (winner given matchup 78.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 20.7% (16.7%–24.8%) | 97.4% / 97.5% ✓ | 53.7% / 53.1% ✓ |
| jm_le_pen | 16.9% | 12.2% (9.6%–14.9%) | 10.8% / 9.0% ✓ | 1.8% / 1.6% |
| jospin | 16.2% | 18.1% (14.3%–22.2%) | 90.3% / 91.3% | 44.0% / 44.6% |
| bayrou | 6.8% | 5.5% (4.2%–6.9%) | 0.0% / 0.1% | 0.0% / 0.1% |
| laguiller | 5.7% | 8.9% (6.9%–11.0%) | 0.9% / 0.8% | 0.3% / 0.3% |
| chevenement | 5.3% | 7.5% (5.4%–9.7%) | 0.4% / 0.8% | 0.0% / 0.0% |
| mamere | 5.2% | 6.2% (4.5%–8.1%) | 0.1% / 0.2% | 0.0% / 0.1% |
| besancenot | 4.2% | 1.7% (1.1%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | 3.9% (3.0%–4.9%) | 0.0% / 0.1% | 0.0% / 0.0% |
| madelin | 3.9% | 3.9% (2.6%–5.3%) | 0.0% / 0.1% | 0.0% / 0.0% |
| hue | 3.4% | 5.4% (3.8%–7.0%) | 0.1% / 0.1% | 0.0% / 0.0% |
| megret | 2.3% | 2.1% (1.4%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.9% (0.6%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 1.4% (0.9%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.6% (0.4%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 182 days out (2006-10-22)

Actual matchup: current 99.6% (winner given matchup 47.2%), fitted 98.4% (winner given matchup 47.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 31.2% (26.1%–36.3%) | 99.9% / 99.7% ✓ | 47.3% / 47.6% ✓ |
| royal | 25.9% | 27.7% (22.0%–33.1%) | 99.7% / 98.6% ✓ | 52.7% / 51.9% |
| bayrou | 18.6% | 7.7% (6.0%–9.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| jm_le_pen | 10.4% | 11.5% (9.1%–13.9%) | 0.4% / 0.8% | 0.0% / 0.1% |
| besancenot | 4.1% | 4.9% (2.8%–7.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 3.8% (2.6%–5.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 3.0% (1.2%–5.1%) | 0.0% / 0.4% | 0.0% / 0.1% |
| voynet | 1.6% | 2.5% (1.0%–4.2%) | 0.0% / 0.4% | 0.0% / 0.1% |
| laguiller | 1.3% | 3.4% (1.7%–5.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 3.2% (1.6%–5.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 1.3% (0.5%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 91 days out (2007-01-21)

Actual matchup: current 99.2% (winner given matchup 57.6%), fitted 98.5% (winner given matchup 57.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 30.4% (24.5%–36.6%) | 99.7% / 99.6% ✓ | 57.7% / 57.6% ✓ |
| royal | 25.9% | 28.8% (22.8%–34.9%) | 99.5% / 98.9% ✓ | 42.3% / 42.3% |
| bayrou | 18.6% | 9.1% (6.8%–11.6%) | 0.1% / 0.2% | 0.0% / 0.0% |
| jm_le_pen | 10.4% | 12.3% (9.3%–15.4%) | 0.7% / 1.0% | 0.0% / 0.0% |
| besancenot | 4.1% | 4.1% (2.4%–5.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 2.5% (1.7%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 3.2% (1.5%–5.2%) | 0.0% / 0.2% | 0.0% / 0.1% |
| voynet | 1.6% | 2.2% (1.1%–3.5%) | 0.0% / 0.1% | 0.0% / 0.0% |
| laguiller | 1.3% | 3.2% (1.8%–4.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 2.7% (1.4%–4.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.6% (0.3%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 30 days out (2007-03-23)

Actual matchup: current 67.7% (winner given matchup 80.1%), fitted 69.8% (winner given matchup 80.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 28.3% (22.8%–34.0%) | 91.1% / 92.2% ✓ | 57.7% / 59.6% ✓ |
| royal | 25.9% | 24.7% (19.7%–29.9%) | 76.6% / 77.5% ✓ | 15.0% / 15.1% |
| bayrou | 18.6% | 20.6% (16.1%–25.4%) | 31.4% / 29.3% | 27.3% / 25.3% |
| jm_le_pen | 10.4% | 12.8% (9.8%–15.9%) | 0.9% / 0.9% | 0.0% / 0.0% |
| besancenot | 4.1% | 3.2% (2.1%–4.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 1.6% (1.1%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 2.5% (1.4%–3.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| voynet | 1.6% | 1.1% (0.6%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 1.3% | 2.0% (1.2%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 2.0% (1.2%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 0.7% (0.4%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 7 days out (2007-04-15)

Actual matchup: current 81.9% (winner given matchup 84.2%), fitted 85.3% (winner given matchup 84.2%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 28.2% (23.4%–33.0%) | 95.9% / 96.6% ✓ | 71.6% / 74.1% ✓ |
| royal | 25.9% | 24.5% (20.2%–28.8%) | 86.0% / 88.6% ✓ | 13.7% / 14.1% |
| bayrou | 18.6% | 18.5% (15.0%–22.2%) | 16.8% / 13.5% | 14.7% / 11.7% |
| jm_le_pen | 10.4% | 13.6% (10.9%–16.4%) | 1.3% / 1.1% | 0.0% / 0.0% |
| besancenot | 4.1% | 3.9% (2.9%–5.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 1.5% (1.1%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 2.7% (1.8%–3.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| voynet | 1.6% | 1.2% (0.8%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 1.3% | 2.0% (1.4%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 1.9% (1.3%–2.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 1.3% (0.9%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 182 days out (2011-10-23)

Actual matchup: current 65.0% (winner given matchup 98.1%), fitted 62.3% (winner given matchup 98.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 35.8% (24.1%–47.7%) | 97.1% / 94.5% ✓ | 93.2% / 90.0% ✓ |
| sarkozy | 27.2% | 21.9% (14.0%–30.7%) | 67.8% / 67.3% ✓ | 4.0% / 5.4% |
| le_pen | 17.9% | 17.7% (11.1%–25.2%) | 32.7% / 32.1% | 2.5% / 2.4% |
| melenchon | 11.1% | 7.3% (2.6%–13.0%) | 0.9% / 3.2% | 0.1% / 1.2% |
| bayrou | 9.1% | 9.0% (5.3%–13.3%) | 1.1% / 1.2% | 0.2% / 0.4% |
| joly | 2.3% | 5.8% (2.0%–10.5%) | 0.3% / 1.8% | 0.0% / 0.7% |
| dupont_aignan | 1.8% | 0.8% (0.4%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.2%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.0% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 91 days out (2012-01-22)

Actual matchup: current 51.6% (winner given matchup 95.4%), fitted 50.6% (winner given matchup 95.2%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 30.1% (21.6%–38.8%) | 92.4% / 90.4% ✓ | 84.5% / 82.6% ✓ |
| sarkozy | 27.2% | 21.9% (15.5%–28.8%) | 58.9% / 59.4% ✓ | 9.2% / 10.5% |
| le_pen | 17.9% | 20.3% (14.4%–26.7%) | 44.2% / 44.3% | 4.5% / 4.3% |
| melenchon | 11.1% | 8.3% (4.0%–13.1%) | 0.5% / 2.0% | 0.1% / 0.8% |
| bayrou | 9.1% | 12.9% (8.6%–17.6%) | 4.0% / 3.7% | 1.8% / 1.7% |
| joly | 2.3% | 4.1% (1.9%–6.7%) | 0.0% / 0.2% | 0.0% / 0.1% |
| dupont_aignan | 1.8% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 30 days out (2012-03-23)

Actual matchup: current 93.0% (winner given matchup 93.7%), fitted 92.5% (winner given matchup 93.8%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 28.1% (21.7%–34.6%) | 96.8% / 95.7% ✓ | 90.3% / 89.3% ✓ |
| sarkozy | 27.2% | 27.2% (21.5%–33.2%) | 96.2% / 96.7% ✓ | 8.7% / 9.2% |
| le_pen | 17.9% | 15.1% (11.4%–18.9%) | 4.6% / 4.1% | 0.3% / 0.2% |
| melenchon | 11.1% | 11.7% (7.6%–16.1%) | 1.4% / 2.5% | 0.3% / 0.8% |
| bayrou | 9.1% | 12.4% (9.3%–15.7%) | 1.1% / 1.0% | 0.4% / 0.4% |
| joly | 2.3% | 2.8% (1.7%–4.0%) | 0.0% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 1.8% | 0.9% (0.6%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.4%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.4%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 7 days out (2012-04-15)

Actual matchup: current 91.1% (winner given matchup 94.0%), fitted 91.9% (winner given matchup 94.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 26.9% (21.3%–32.6%) | 95.0% / 95.2% ✓ | 88.8% / 89.1% ✓ |
| sarkozy | 27.2% | 27.5% (22.1%–33.0%) | 96.0% / 96.7% ✓ | 9.8% / 9.4% |
| le_pen | 17.9% | 15.4% (12.0%–19.1%) | 5.2% / 4.2% | 0.3% / 0.2% |
| melenchon | 11.1% | 14.0% (10.1%–18.2%) | 3.5% / 3.5% | 1.0% / 1.1% |
| bayrou | 9.1% | 10.5% (8.0%–13.1%) | 0.2% / 0.3% | 0.1% / 0.2% |
| joly | 2.3% | 2.5% (1.7%–3.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 1.8% | 1.0% (0.7%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.5%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.5%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 182 days out (2016-10-23)

Actual matchup: current 13.1% (winner given matchup 57.5%), fitted 12.0% (winner given matchup 60.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 17.3% (12.7%–22.2%) | 15.7% / 14.0% ✓ | 9.0% / 8.3% ✓ |
| le_pen | 21.3% | 30.4% (23.6%–37.4%) | 92.9% / 93.9% ✓ | 26.9% / 27.0% |
| fillon | 20.0% | 21.0% (15.6%–26.6%) | 44.0% / 44.2% | 41.6% / 42.1% |
| melenchon | 19.6% | 21.3% (15.8%–27.1%) | 47.4% / 47.8% | 22.5% / 22.6% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 4.8% (2.8%–6.9%) | 0.0% / 0.1% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.8% (0.2%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.6% (0.8%–2.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 1.0% (0.3%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.4% (0.7%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 91 days out (2017-01-22)

Actual matchup: current 18.3% (winner given matchup 99.5%), fitted 17.9% (winner given matchup 99.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 20.0% (12.7%–28.2%) | 38.5% / 37.7% ✓ | 32.7% / 32.0% ✓ |
| le_pen | 21.3% | 26.0% (17.1%–35.7%) | 73.4% / 73.6% ✓ | 2.4% / 2.4% |
| fillon | 20.0% | 25.9% (16.9%–35.6%) | 73.0% / 73.2% | 58.2% / 58.7% |
| melenchon | 19.6% | 14.7% (8.0%–22.1%) | 14.3% / 13.8% | 6.3% / 6.2% |
| hamon | 6.4% | 8.1% (3.5%–13.5%) | 0.8% / 1.6% | 0.3% / 0.7% |
| dupont_aignan | 4.7% | 1.9% (0.9%–3.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.5% (0.1%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.0% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.6% (0.1%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.0% (0.4%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.4% (0.1%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 30 days out (2017-03-24)

Actual matchup: current 70.7% (winner given matchup 99.7%), fitted 72.7% (winner given matchup 99.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 25.3% (19.2%–31.8%) | 85.4% / 86.8% ✓ | 84.5% / 85.9% ✓ |
| le_pen | 21.3% | 25.0% (18.9%–31.4%) | 84.8% / 85.4% ✓ | 1.3% / 1.3% |
| fillon | 20.0% | 17.8% (13.1%–22.9%) | 23.6% / 21.4% | 11.3% / 9.9% |
| melenchon | 19.6% | 12.6% (8.3%–17.3%) | 3.6% / 3.7% | 1.9% / 1.7% |
| hamon | 6.4% | 11.8% (7.6%–16.5%) | 2.6% / 2.6% | 1.1% / 1.2% |
| dupont_aignan | 4.7% | 3.8% (2.5%–5.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 1.0% (0.5%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.5%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 7 days out (2017-04-16)

Actual matchup: current 42.2% (winner given matchup 99.8%), fitted 45.5% (winner given matchup 99.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 22.9% (18.5%–27.5%) | 72.0% / 74.6% ✓ | 71.3% / 73.9% ✓ |
| le_pen | 21.3% | 22.1% (17.8%–26.6%) | 66.2% / 67.5% ✓ | 0.7% / 0.8% |
| fillon | 20.0% | 19.1% (15.1%–23.2%) | 31.4% / 29.6% | 11.9% / 10.7% |
| melenchon | 19.6% | 18.8% (14.7%–23.1%) | 30.2% / 28.0% | 16.0% / 14.5% |
| hamon | 6.4% | 8.4% (5.8%–11.3%) | 0.1% / 0.2% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 3.6% (2.6%–4.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 1.1% (0.7%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.6% (1.1%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.9% (0.5%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.5%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 182 days out (2021-10-10)

Actual matchup: current 49.1% (winner given matchup 90.8%), fitted 47.0% (winner given matchup 91.2%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 25.8% (17.2%–35.0%) | 89.4% / 90.3% ✓ | 82.0% / 83.3% ✓ |
| le_pen | 23.1% | 17.7% (9.2%–26.9%) | 58.7% / 55.0% ✓ | 9.7% / 8.3% |
| melenchon | 21.9% | 8.2% (3.7%–13.4%) | 3.6% / 4.5% | 0.5% / 0.7% |
| zemmour | 7.1% | 14.6% (6.8%–23.5%) | 30.7% / 33.8% | 4.2% / 3.9% |
| pecresse | 4.8% | 11.8% (7.4%–16.9%) | 14.8% / 11.5% | 2.9% / 2.3% |
| jadot | 4.6% | 7.4% (3.2%–12.4%) | 2.4% / 3.3% | 0.6% / 1.1% |
| lassalle | 3.1% | 1.3% (0.4%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.3% (0.8%–4.0%) | 0.0% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 3.4% (1.8%–5.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 5.6% (2.2%–9.7%) | 0.4% / 1.4% | 0.1% / 0.4% |
| poutou | 0.8% | 1.1% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.4%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 91 days out (2022-01-09)

Actual matchup: current 31.0% (winner given matchup 95.4%), fitted 31.6% (winner given matchup 95.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 25.4% (16.6%–35.0%) | 86.6% / 86.7% ✓ | 78.9% / 79.5% ✓ |
| le_pen | 23.1% | 16.2% (8.8%–24.4%) | 41.0% / 40.8% ✓ | 5.9% / 5.5% |
| melenchon | 21.9% | 10.5% (5.5%–16.3%) | 9.7% / 9.6% | 1.3% / 1.3% |
| zemmour | 7.1% | 13.3% (6.6%–21.0%) | 20.9% / 22.9% | 2.8% / 2.8% |
| pecresse | 4.8% | 16.0% (10.0%–22.7%) | 40.5% / 37.6% | 10.7% / 10.3% |
| jadot | 4.6% | 6.9% (3.2%–11.3%) | 1.3% / 2.0% | 0.3% / 0.6% |
| lassalle | 3.1% | 1.2% (0.5%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.5% (1.0%–4.2%) | 0.0% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.3% (1.2%–3.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 3.7% (1.5%–6.5%) | 0.0% / 0.2% | 0.0% / 0.1% |
| poutou | 0.8% | 1.2% (0.6%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.3%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 30 days out (2022-03-11)

Actual matchup: current 61.9% (winner given matchup 98.3%), fitted 62.6% (winner given matchup 98.2%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 30.5% (23.4%–37.8%) | 98.9% / 98.9% ✓ | 97.7% / 97.7% ✓ |
| le_pen | 23.1% | 16.8% (11.5%–22.5%) | 62.9% / 63.5% ✓ | 1.6% / 1.6% |
| melenchon | 21.9% | 11.5% (7.7%–15.7%) | 12.2% / 11.2% | 0.1% / 0.1% |
| zemmour | 7.1% | 12.4% (7.8%–17.5%) | 14.1% / 15.9% | 0.3% / 0.3% |
| pecresse | 4.8% | 11.9% (8.5%–15.5%) | 11.8% / 10.1% | 0.2% / 0.2% |
| jadot | 4.6% | 5.4% (3.1%–7.9%) | 0.1% / 0.2% | 0.0% / 0.0% |
| lassalle | 3.1% | 1.9% (1.1%–2.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.7% (2.1%–5.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.0% (1.2%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.2% (1.2%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.0% (0.6%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 7 days out (2022-04-03)

Actual matchup: current 83.8% (winner given matchup 93.7%), fitted 85.6% (winner given matchup 93.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 27.1% (22.0%–32.4%) | 98.1% / 98.3% ✓ | 92.8% / 92.9% ✓ |
| le_pen | 23.1% | 21.0% (16.3%–25.7%) | 85.6% / 87.2% ✓ | 6.2% / 6.2% |
| melenchon | 21.9% | 14.8% (11.2%–18.5%) | 14.2% / 12.0% | 0.8% / 0.7% |
| zemmour | 7.1% | 10.9% (7.6%–14.4%) | 1.7% / 2.1% | 0.1% / 0.1% |
| pecresse | 4.8% | 9.4% (7.2%–11.6%) | 0.4% / 0.3% | 0.0% / 0.0% |
| jadot | 4.6% | 5.1% (3.4%–6.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 3.1% | 2.5% (1.7%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.2% (2.1%–4.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.1% (1.5%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.1% (1.3%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.2% (0.8%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
