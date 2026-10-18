# Backtest: 2002, 2007, 2012, 2017 and 2022 replays

Written by `fr2027 backtest` (moon task `cli:backtest`) from `config/backtest.yaml`, `config/model.yaml` (model 0.5.0) and `data/backtest/`; `results.json` beside this file has every number. Seed 20170423, 20000 simulations per replay and setting. How to read it and what was decided: `research/LOG.md`.

Each replay fits the model to the polls published by its date (fieldwork started at most 365 days before round 1; overlapping polls of one firm thinned), conditions on the field that actually ran, and simulates both rounds. Probabilities are clipped to [0.001, 0.999] for log-loss. Share log scores are summed over contenders (fitted share ≥ 10%) and replays; higher is better. Brier, log-loss and MAE: lower is better.

## Settings

Horizon errors are per square root of 30 days before the round; they add in variance to the election-day error.

| | Current (config/model.yaml) | Fitted |
|---|---|---|
| round1_error.bloc_sd | 0.18 | 0.18 |
| round1_error.candidate_sd | 0.2 | 0.2 |
| round1_error.horizon_bloc_sd | 0 | 0 |
| round1_error.horizon_candidate_sd | 0.2 | 0.2 |
| round1_error.student_t_df | 3 | 3 |
| round2.error_sd | 0.12 | 0.12 |
| round2.horizon_error_sd | 0 | 0 |
## Scores

All replays (every election, every horizon):

| | Qualify Brier | Qualify log-loss | Win Brier | Win log-loss | Share MAE (pts) | 80% coverage | Share log score | Round 2 log score | P(winner \| matchup) |
|---|---|---|---|---|---|---|---|---|---|
| Latest-poll average | 0.048 | 0.335 | 0.034 | 0.238 | 2.47 | – | – | – | 91.9% |
| Model, current | 0.049 | 0.179 | 0.023 | 0.080 | 2.31 | 67.0% | -113.9 | -45.17 | 84.1% |
| Model, fitted | 0.049 | 0.179 | 0.023 | 0.080 | 2.31 | 67.0% | -113.9 | -45.17 | 84.1% |

By election:

| | Qualify Brier | Qualify log-loss | Win Brier | Win log-loss | Share MAE (pts) | 80% coverage | Share log score | Round 2 log score | P(winner \| matchup) |
|---|---|---|---|---|---|---|---|---|---|
| 2002, baseline | 0.125 | 0.864 | 0.036 | 0.248 | 2.65 | – | – | – | – |
| 2007, baseline | 0.000 | 0.001 | 0.048 | 0.330 | 1.63 | – | – | – | 71.4% |
| 2012, baseline | 0.000 | 0.001 | 0.000 | 0.001 | 1.57 | – | – | – | 100.0% |
| 2017, baseline | 0.087 | 0.599 | 0.087 | 0.599 | 3.06 | – | – | – | 100.0% |
| 2022, baseline | 0.000 | 0.001 | 0.000 | 0.001 | 3.30 | – | – | – | 100.0% |
| 2002, current | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2007, current | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2012, current | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2017, current | 0.061 | 0.188 | 0.056 | 0.180 | 2.17 | 66.2% | -21.5 | -10.85 | 84.6% |
| 2022, current | 0.025 | 0.097 | 0.004 | 0.027 | 3.34 | 60.2% | -131.8 | 14.13 | 92.8% |
| 2002, fitted | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2007, fitted | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2012, fitted | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2017, fitted | 0.061 | 0.188 | 0.056 | 0.180 | 2.17 | 66.2% | -21.5 | -10.85 | 84.6% |
| 2022, fitted | 0.025 | 0.097 | 0.004 | 0.027 | 3.34 | 60.2% | -131.8 | 14.13 | 92.8% |

By horizon (days before round 1):

| | Qualify Brier | Qualify log-loss | Win Brier | Win log-loss | Share MAE (pts) | 80% coverage | Share log score | Round 2 log score | P(winner \| matchup) |
|---|---|---|---|---|---|---|---|---|---|
| 334, baseline | 0.066 | 0.454 | 0.066 | 0.454 | 2.95 | – | – | – | 66.7% |
| 304, baseline | 0.066 | 0.454 | 0.033 | 0.227 | 3.20 | – | – | – | 100.0% |
| 273, baseline | 0.066 | 0.454 | 0.066 | 0.454 | 3.79 | – | – | – | 66.7% |
| 243, baseline | 0.066 | 0.454 | 0.066 | 0.454 | 3.12 | – | – | – | 66.7% |
| 212, baseline | 0.066 | 0.454 | 0.066 | 0.454 | 3.14 | – | – | – | 66.7% |
| 182, baseline | 0.066 | 0.454 | 0.033 | 0.227 | 3.21 | – | – | – | 100.0% |
| 152, baseline | 0.066 | 0.454 | 0.066 | 0.454 | 3.06 | – | – | – | 66.7% |
| 122, baseline | 0.066 | 0.454 | 0.066 | 0.454 | 3.00 | – | – | – | 66.7% |
| 91, baseline | 0.066 | 0.454 | 0.033 | 0.227 | 2.60 | – | – | – | 100.0% |
| 84, baseline | 0.066 | 0.454 | 0.033 | 0.227 | 2.57 | – | – | – | 100.0% |
| 77, baseline | 0.033 | 0.227 | 0.000 | 0.001 | 2.50 | – | – | – | 100.0% |
| 70, baseline | 0.033 | 0.227 | 0.000 | 0.001 | 2.45 | – | – | – | 100.0% |
| 63, baseline | 0.033 | 0.227 | 0.000 | 0.001 | 2.31 | – | – | – | 100.0% |
| 56, baseline | 0.033 | 0.227 | 0.000 | 0.001 | 2.26 | – | – | – | 100.0% |
| 49, baseline | 0.033 | 0.227 | 0.033 | 0.227 | 2.18 | – | – | – | 100.0% |
| 42, baseline | 0.033 | 0.227 | 0.033 | 0.227 | 2.24 | – | – | – | 100.0% |
| 35, baseline | 0.033 | 0.227 | 0.033 | 0.227 | 2.03 | – | – | – | 100.0% |
| 28, baseline | 0.033 | 0.227 | 0.033 | 0.227 | 1.91 | – | – | – | 100.0% |
| 21, baseline | 0.033 | 0.227 | 0.033 | 0.227 | 1.69 | – | – | – | 100.0% |
| 14, baseline | 0.033 | 0.227 | 0.033 | 0.227 | 1.48 | – | – | – | 100.0% |
| 7, baseline | 0.033 | 0.227 | 0.000 | 0.001 | 1.18 | – | – | – | 100.0% |
| 334, current | 0.041 | 0.186 | 0.027 | 0.091 | 2.80 | 68.8% | -3.2 | -2.66 | 73.3% |
| 304, current | 0.057 | 0.224 | 0.042 | 0.134 | 2.65 | 71.2% | -0.6 | -3.77 | 69.0% |
| 273, current | 0.053 | 0.215 | 0.037 | 0.116 | 2.68 | 68.5% | -0.6 | -3.15 | 72.4% |
| 243, current | 0.051 | 0.213 | 0.033 | 0.106 | 2.59 | 68.5% | -0.3 | -2.93 | 73.2% |
| 212, current | 0.052 | 0.201 | 0.035 | 0.118 | 2.64 | 69.1% | -5.0 | -3.77 | 73.2% |
| 182, current | 0.058 | 0.200 | 0.034 | 0.115 | 2.74 | 72.7% | -2.2 | -3.79 | 77.5% |
| 152, current | 0.059 | 0.198 | 0.035 | 0.122 | 2.81 | 67.3% | -4.4 | -4.06 | 73.6% |
| 122, current | 0.069 | 0.220 | 0.039 | 0.118 | 2.85 | 68.4% | -6.5 | -2.21 | 83.8% |
| 91, current | 0.057 | 0.187 | 0.023 | 0.076 | 2.52 | 71.2% | -7.8 | -0.82 | 86.8% |
| 84, current | 0.062 | 0.200 | 0.022 | 0.075 | 2.57 | 69.5% | -9.3 | -0.74 | 89.0% |
| 77, current | 0.053 | 0.184 | 0.020 | 0.071 | 2.49 | 67.8% | -13.2 | -1.04 | 90.1% |
| 70, current | 0.051 | 0.178 | 0.017 | 0.063 | 2.45 | 68.3% | -12.6 | -1.39 | 90.4% |
| 63, current | 0.047 | 0.171 | 0.014 | 0.055 | 2.31 | 67.2% | -12.1 | -1.34 | 92.1% |
| 56, current | 0.042 | 0.163 | 0.010 | 0.046 | 2.19 | 68.9% | -10.5 | -2.12 | 90.1% |
| 49, current | 0.043 | 0.164 | 0.012 | 0.046 | 2.28 | 62.3% | -11.3 | -2.53 | 90.7% |
| 42, current | 0.042 | 0.167 | 0.015 | 0.053 | 2.04 | 63.9% | -6.9 | -1.92 | 90.4% |
| 35, current | 0.039 | 0.159 | 0.017 | 0.058 | 1.98 | 59.0% | -5.5 | -1.36 | 90.6% |
| 28, current | 0.036 | 0.152 | 0.016 | 0.053 | 1.92 | 59.0% | -7.7 | -0.61 | 89.6% |
| 21, current | 0.034 | 0.134 | 0.015 | 0.053 | 1.72 | 65.6% | -3.4 | -1.11 | 89.6% |
| 14, current | 0.035 | 0.125 | 0.014 | 0.053 | 1.46 | 65.6% | 1.3 | -1.23 | 90.2% |
| 7, current | 0.039 | 0.128 | 0.013 | 0.052 | 1.22 | 67.2% | 8.0 | -2.60 | 90.0% |
| 334, fitted | 0.041 | 0.186 | 0.027 | 0.091 | 2.80 | 68.8% | -3.2 | -2.66 | 73.3% |
| 304, fitted | 0.057 | 0.224 | 0.042 | 0.134 | 2.65 | 71.2% | -0.6 | -3.77 | 69.0% |
| 273, fitted | 0.053 | 0.215 | 0.037 | 0.116 | 2.68 | 68.5% | -0.6 | -3.15 | 72.4% |
| 243, fitted | 0.051 | 0.213 | 0.033 | 0.106 | 2.59 | 68.5% | -0.3 | -2.93 | 73.2% |
| 212, fitted | 0.052 | 0.201 | 0.035 | 0.118 | 2.64 | 69.1% | -5.0 | -3.77 | 73.2% |
| 182, fitted | 0.058 | 0.200 | 0.034 | 0.115 | 2.74 | 72.7% | -2.2 | -3.79 | 77.5% |
| 152, fitted | 0.059 | 0.198 | 0.035 | 0.122 | 2.81 | 67.3% | -4.4 | -4.06 | 73.6% |
| 122, fitted | 0.069 | 0.220 | 0.039 | 0.118 | 2.85 | 68.4% | -6.5 | -2.21 | 83.8% |
| 91, fitted | 0.057 | 0.187 | 0.023 | 0.076 | 2.52 | 71.2% | -7.8 | -0.82 | 86.8% |
| 84, fitted | 0.062 | 0.200 | 0.022 | 0.075 | 2.57 | 69.5% | -9.3 | -0.74 | 89.0% |
| 77, fitted | 0.053 | 0.184 | 0.020 | 0.071 | 2.49 | 67.8% | -13.2 | -1.04 | 90.1% |
| 70, fitted | 0.051 | 0.178 | 0.017 | 0.063 | 2.45 | 68.3% | -12.6 | -1.39 | 90.4% |
| 63, fitted | 0.047 | 0.171 | 0.014 | 0.055 | 2.31 | 67.2% | -12.1 | -1.34 | 92.1% |
| 56, fitted | 0.042 | 0.163 | 0.010 | 0.046 | 2.19 | 68.9% | -10.5 | -2.12 | 90.1% |
| 49, fitted | 0.043 | 0.164 | 0.012 | 0.046 | 2.28 | 62.3% | -11.3 | -2.53 | 90.7% |
| 42, fitted | 0.042 | 0.167 | 0.015 | 0.053 | 2.04 | 63.9% | -6.9 | -1.92 | 90.4% |
| 35, fitted | 0.039 | 0.159 | 0.017 | 0.058 | 1.98 | 59.0% | -5.5 | -1.36 | 90.6% |
| 28, fitted | 0.036 | 0.152 | 0.016 | 0.053 | 1.92 | 59.0% | -7.7 | -0.61 | 89.6% |
| 21, fitted | 0.034 | 0.134 | 0.015 | 0.053 | 1.72 | 65.6% | -3.4 | -1.11 | 89.6% |
| 14, fitted | 0.035 | 0.125 | 0.014 | 0.053 | 1.46 | 65.6% | 1.3 | -1.23 | 90.2% |
| 7, fitted | 0.039 | 0.128 | 0.013 | 0.052 | 1.22 | 67.2% | 8.0 | -2.60 | 90.0% |

## Reliability

Qualification and win events pooled over every replay.

| Forecast | Events (current) | Mean forecast | Observed | Events (fitted) | Mean forecast | Observed |
|---|---|---|---|---|---|---|
| 0–10% | 2045 | 0.6% | 1.3% | 2045 | 0.6% | 1.3% |
| 10–20% | 95 | 14.0% | 8.4% | 95 | 14.0% | 8.4% |
| 20–30% | 37 | 24.4% | 5.4% | 37 | 24.4% | 5.4% |
| 30–40% | 44 | 34.2% | 29.5% | 44 | 34.2% | 29.5% |
| 40–50% | 47 | 45.1% | 42.6% | 47 | 45.1% | 42.6% |
| 50–60% | 38 | 54.9% | 47.4% | 38 | 54.9% | 47.4% |
| 60–70% | 25 | 64.9% | 80.0% | 25 | 64.9% | 80.0% |
| 70–80% | 34 | 75.5% | 94.1% | 34 | 75.5% | 94.1% |
| 80–90% | 69 | 85.5% | 88.4% | 69 | 85.5% | 88.4% |
| 90–100% | 128 | 95.4% | 89.8% | 128 | 95.4% | 89.8% |

## Fit

The rule, in each stage: among the grid points that score at least as well as the stage's starting setting on every source of evidence (each election's replays, and the final-week polls of other elections: none), take the one with the highest total log score; if none does, keep the starting setting. The first stage starts from config/model.yaml and each later one from the setting the previous one chose; the three stages repeat until a pass changes nothing (1 passes here, at most 4). Every stage keeps or improves every source, so the fitted setting scores at least as well as the current one on all of them. Round 1 stages score contenders' shares; the round 2 stage scores the realised runoff share given the actual matchup. Final-week polls are taken as election-eve polls, with no horizon error.


### Pass 1: Round 1 horizon error and tails

Evidence: share log score of the contenders. Starts from horizon_bloc_sd 0, horizon_candidate_sd 0.2, student_t_df 3.

| Evidence | Start | Chosen | Best on this source alone |
|---|---|---|---|
| 2002 replays | -24.07 | -24.07 | -16.96 (horizon_bloc_sd 0.1, horizon_candidate_sd 0.2, student_t_df 10) |
| 2007 replays | 35.47 | 35.47 | 37.98 (horizon_bloc_sd 0, horizon_candidate_sd 0.02, student_t_df 2.5) |
| 2012 replays | 28.03 | 28.03 | 33.64 (horizon_bloc_sd 0, horizon_candidate_sd 0, student_t_df 2.5) |
| 2017 replays | -21.48 | -21.48 | -12.35 (horizon_bloc_sd 0.07, horizon_candidate_sd 0.4, student_t_df 10) |
| 2022 replays | -131.81 | -131.81 | -88.71 (horizon_bloc_sd 0.27, horizon_candidate_sd 0.2, student_t_df 10) |
| 334-day replays, every election (not used) | -3.22 | -3.22 | -2.34 (horizon_bloc_sd 0, horizon_candidate_sd 0.2, student_t_df 10) |
| 304-day replays, every election (not used) | -0.58 | -0.58 | 0.08 (horizon_bloc_sd 0, horizon_candidate_sd 0.2, student_t_df 10) |
| 273-day replays, every election (not used) | -0.57 | -0.57 | -0.19 (horizon_bloc_sd 0.02, horizon_candidate_sd 0.14, student_t_df 10) |
| 243-day replays, every election (not used) | -0.29 | -0.29 | 0.00 (horizon_bloc_sd 0, horizon_candidate_sd 0.2, student_t_df 10) |
| 212-day replays, every election (not used) | -4.97 | -4.97 | -3.89 (horizon_bloc_sd 0.07, horizon_candidate_sd 0.2, student_t_df 10) |
| 182-day replays, every election (not used) | -2.20 | -2.20 | -1.87 (horizon_bloc_sd 0, horizon_candidate_sd 0.14, student_t_df 10) |
| 152-day replays, every election (not used) | -4.44 | -4.44 | -3.94 (horizon_bloc_sd 0.04, horizon_candidate_sd 0.2, student_t_df 10) |
| 122-day replays, every election (not used) | -6.54 | -6.54 | -5.83 (horizon_bloc_sd 0.1, horizon_candidate_sd 0.33, student_t_df 2.5) |
| 91-day replays, every election (not used) | -7.83 | -7.83 | -6.84 (horizon_bloc_sd 0.07, horizon_candidate_sd 0.4, student_t_df 2.5) |
| 84-day replays, every election (not used) | -9.33 | -9.33 | -8.41 (horizon_bloc_sd 0.2, horizon_candidate_sd 0.4, student_t_df 2.5) |
| 77-day replays, every election (not used) | -13.20 | -13.20 | -10.64 (horizon_bloc_sd 0.2, horizon_candidate_sd 0.4, student_t_df 3) |
| 70-day replays, every election (not used) | -12.56 | -12.56 | -10.52 (horizon_bloc_sd 0.2, horizon_candidate_sd 0.33, student_t_df 4) |
| 63-day replays, every election (not used) | -12.12 | -12.12 | -9.53 (horizon_bloc_sd 0.14, horizon_candidate_sd 0.4, student_t_df 3) |
| 56-day replays, every election (not used) | -10.51 | -10.51 | -8.20 (horizon_bloc_sd 0.07, horizon_candidate_sd 0.4, student_t_df 3) |
| 49-day replays, every election (not used) | -11.27 | -11.27 | -8.54 (horizon_bloc_sd 0.14, horizon_candidate_sd 0.4, student_t_df 10) |
| 42-day replays, every election (not used) | -6.90 | -6.90 | -5.07 (horizon_bloc_sd 0.1, horizon_candidate_sd 0.4, student_t_df 4) |
| 35-day replays, every election (not used) | -5.51 | -5.51 | -4.25 (horizon_bloc_sd 0.02, horizon_candidate_sd 0.4, student_t_df 3) |
| 28-day replays, every election (not used) | -7.67 | -7.67 | -5.45 (horizon_bloc_sd 0.2, horizon_candidate_sd 0.33, student_t_df 5) |
| 21-day replays, every election (not used) | -3.44 | -3.44 | -2.04 (horizon_bloc_sd 0.2, horizon_candidate_sd 0.27, student_t_df 5) |
| 14-day replays, every election (not used) | 1.34 | 1.34 | 1.57 (horizon_bloc_sd 0.1, horizon_candidate_sd 0.4, student_t_df 2.5) |
| 7-day replays, every election (not used) | 7.98 | 7.98 | 8.20 (horizon_bloc_sd 0, horizon_candidate_sd 0.4, student_t_df 2.5) |
| Total | -113.85 | -113.85 | -102.01 (horizon_bloc_sd 0.07, horizon_candidate_sd 0.4, student_t_df 2.5) |

Total log score by `horizon_bloc_sd` (rows) and `horizon_candidate_sd` (columns). `*` marks the points that score at least as well as the start on every source; bold is the chosen one.

student_t_df = 2.5:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -156.0 | -154.9 | -153.3 | -148.5 | -142.4 | -135.1 | -125.0 | -115.6 | -110.5 | -106.6 |
| 0.02 | -154.1 | -153.2 | -151.4 | -146.9 | -141.2 | -133.9 | -124.0 | -114.9 | -109.8 | -106.0 |
| 0.04 | -150.5 | -150.0 | -148.3 | -144.2 | -138.8 | -132.0 | -122.7 | -114.0 | -109.1 | -105.5 |
| 0.07 | -141.4 | -140.9 | -139.6 | -136.3 | -131.9 | -126.2 | -117.6 | -109.9 | -105.4 | -102.0 |
| 0.1 | -136.3 | -136.0 | -135.0 | -132.4 | -128.7 | -123.8 | -115.9 | -109.3 | -105.3 | -102.3 |
| 0.14 | -134.7 | -134.4 | -133.7 | -131.6 | -129.1 | -125.2 | -119.0 | -113.7 | -110.5 | -108.0 |
| 0.2 | -134.8 | -134.6 | -134.2 | -132.9 | -131.1 | -128.0 | -123.6 | -119.8 | -117.7 | -116.5 |
| 0.27 | -142.4 | -142.4 | -142.1 | -141.3 | -140.3 | -138.4 | -135.8 | -133.5 | -132.5 | -132.3 |

student_t_df = 3:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -144.3 | -143.4 | -141.3 | -136.4 | -130.6 | -122.8 | **-113.9*** | -106.7 | -103.8 | -102.2 |
| 0.02 | -143.4 | -142.5 | -140.3 | -135.7 | -129.9 | -122.4 | -113.6 | -106.6 | -103.7 | -102.2 |
| 0.04 | -140.9 | -140.1 | -138.1 | -133.7 | -128.1 | -121.0 | -113.0 | -106.3 | -103.6 | -102.1 |
| 0.07 | -135.1 | -134.6 | -133.1 | -129.6 | -124.6 | -118.8 | -111.7 | -105.7 | -103.4 | -102.2 |
| 0.1 | -132.6 | -132.2 | -131.0 | -128.0 | -124.2 | -119.3 | -112.8 | -107.9 | -105.9 | -105.0 |
| 0.14 | -131.2 | -130.9 | -130.1 | -128.0 | -125.2 | -121.4 | -116.8 | -113.0 | -111.4 | -111.1 |
| 0.2 | -137.2 | -137.0 | -136.6 | -135.3 | -133.5 | -131.2 | -128.6 | -126.3 | -125.6 | -126.1 |
| 0.27 | -152.8 | -152.7 | -152.4 | -151.7 | -150.6 | -149.5 | -148.2 | -147.3 | -147.5 | -148.8 |

student_t_df = 4:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -139.1 | -138.2 | -136.4 | -131.3 | -126.1 | -119.1 | -110.9 | -105.5 | -104.2 | -105.3 |
| 0.02 | -138.5 | -137.7 | -136.1 | -131.0 | -126.0 | -119.2 | -111.0 | -105.7 | -104.5 | -105.5 |
| 0.04 | -136.3 | -135.8 | -134.2 | -130.0 | -125.5 | -119.2 | -111.4 | -106.4 | -105.2 | -106.3 |
| 0.07 | -133.6 | -133.2 | -131.8 | -128.5 | -124.8 | -119.1 | -112.2 | -108.1 | -107.4 | -108.6 |
| 0.1 | -132.2 | -131.8 | -130.7 | -128.2 | -124.9 | -120.1 | -114.7 | -111.5 | -111.1 | -112.5 |
| 0.14 | -132.7 | -132.4 | -131.7 | -129.9 | -127.7 | -124.4 | -120.5 | -118.5 | -118.7 | -120.4 |
| 0.2 | -142.2 | -142.1 | -141.6 | -140.6 | -139.2 | -137.5 | -135.4 | -134.7 | -135.9 | -138.5 |
| 0.27 | -165.2 | -165.1 | -164.9 | -164.4 | -163.9 | -163.0 | -162.1 | -162.6 | -164.6 | -167.6 |

student_t_df = 5:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -140.6 | -139.8 | -137.2 | -131.9 | -125.4 | -118.4 | -110.6 | -107.1 | -107.0 | -109.4 |
| 0.02 | -139.2 | -138.4 | -136.1 | -131.0 | -124.8 | -117.9 | -110.3 | -106.9 | -106.8 | -109.2 |
| 0.04 | -137.5 | -136.7 | -134.5 | -129.8 | -124.4 | -118.0 | -110.9 | -107.7 | -107.8 | -110.3 |
| 0.07 | -131.4 | -130.8 | -129.2 | -126.0 | -121.8 | -116.2 | -110.4 | -107.7 | -108.0 | -110.7 |
| 0.1 | -129.0 | -128.6 | -127.4 | -124.8 | -121.4 | -117.1 | -112.4 | -110.3 | -110.9 | -114.0 |
| 0.14 | -131.9 | -131.6 | -130.8 | -128.9 | -126.6 | -123.4 | -120.2 | -119.1 | -120.3 | -123.6 |
| 0.2 | -143.8 | -143.6 | -143.2 | -142.2 | -140.8 | -139.2 | -138.0 | -138.5 | -140.5 | -144.1 |
| 0.27 | -169.0 | -168.9 | -168.7 | -168.1 | -167.4 | -166.8 | -167.2 | -169.0 | -171.8 | -175.8 |

student_t_df = 10:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -153.1 | -152.1 | -149.5 | -143.8 | -137.1 | -129.6 | -122.1 | -119.2 | -119.7 | -122.7 |
| 0.02 | -149.7 | -148.9 | -146.6 | -141.0 | -134.5 | -127.3 | -119.9 | -117.1 | -117.6 | -120.6 |
| 0.04 | -143.1 | -142.5 | -140.5 | -135.3 | -129.6 | -122.9 | -116.1 | -113.4 | -114.0 | -117.1 |
| 0.07 | -134.6 | -134.1 | -132.5 | -128.6 | -124.3 | -118.8 | -112.9 | -110.7 | -111.5 | -114.8 |
| 0.1 | -133.3 | -133.0 | -131.9 | -129.1 | -125.7 | -121.2 | -116.3 | -115.0 | -116.1 | -119.6 |
| 0.14 | -135.2 | -134.9 | -134.3 | -132.4 | -130.0 | -126.9 | -123.9 | -123.8 | -125.3 | -128.9 |
| 0.2 | -149.9 | -149.8 | -149.4 | -148.5 | -147.3 | -145.6 | -144.4 | -145.1 | -147.3 | -151.4 |
| 0.27 | -176.3 | -176.2 | -176.0 | -175.5 | -175.0 | -174.3 | -174.4 | -176.4 | -179.4 | -183.7 |

### Pass 1: Round 1 election-day error

Evidence: share log score of the contenders. Starts from bloc_sd 0.18, candidate_sd 0.2.

| Evidence | Start | Chosen | Best on this source alone |
|---|---|---|---|
| 2002 replays | -24.07 | -24.07 | -19.68 (bloc_sd 0.33, candidate_sd 0.4) |
| 2007 replays | 35.47 | 35.47 | 38.46 (bloc_sd 0.1, candidate_sd 0.03) |
| 2012 replays | 28.03 | 28.03 | 33.43 (bloc_sd 0.08, candidate_sd 0.03) |
| 2017 replays | -21.48 | -21.48 | -17.82 (bloc_sd 0.22, candidate_sd 0.4) |
| 2022 replays | -131.81 | -131.81 | -90.75 (bloc_sd 0.4, candidate_sd 0.4) |
| 334-day replays, every election (not used) | -3.22 | -3.22 | -3.01 (bloc_sd 0.22, candidate_sd 0.4) |
| 304-day replays, every election (not used) | -0.58 | -0.58 | -0.53 (bloc_sd 0.22, candidate_sd 0.4) |
| 273-day replays, every election (not used) | -0.57 | -0.57 | -0.57 (bloc_sd 0.18, candidate_sd 0.13) |
| 243-day replays, every election (not used) | -0.29 | -0.29 | -0.28 (bloc_sd 0.18, candidate_sd 0.25) |
| 212-day replays, every election (not used) | -4.97 | -4.97 | -4.13 (bloc_sd 0.27, candidate_sd 0.4) |
| 182-day replays, every election (not used) | -2.20 | -2.20 | -2.20 (bloc_sd 0.18, candidate_sd 0.2) |
| 152-day replays, every election (not used) | -4.44 | -4.44 | -4.31 (bloc_sd 0.22, candidate_sd 0.4) |
| 122-day replays, every election (not used) | -6.54 | -6.54 | -6.10 (bloc_sd 0.27, candidate_sd 0.3) |
| 91-day replays, every election (not used) | -7.83 | -7.83 | -6.84 (bloc_sd 0.27, candidate_sd 0.4) |
| 84-day replays, every election (not used) | -9.33 | -9.33 | -8.53 (bloc_sd 0.27, candidate_sd 0.4) |
| 77-day replays, every election (not used) | -13.20 | -13.20 | -10.88 (bloc_sd 0.4, candidate_sd 0.4) |
| 70-day replays, every election (not used) | -12.56 | -12.56 | -10.60 (bloc_sd 0.4, candidate_sd 0.4) |
| 63-day replays, every election (not used) | -12.12 | -12.12 | -9.90 (bloc_sd 0.27, candidate_sd 0.4) |
| 56-day replays, every election (not used) | -10.51 | -10.51 | -9.08 (bloc_sd 0.27, candidate_sd 0.4) |
| 49-day replays, every election (not used) | -11.27 | -11.27 | -9.10 (bloc_sd 0.27, candidate_sd 0.4) |
| 42-day replays, every election (not used) | -6.90 | -6.90 | -5.53 (bloc_sd 0.18, candidate_sd 0.4) |
| 35-day replays, every election (not used) | -5.51 | -5.51 | -4.30 (bloc_sd 0.22, candidate_sd 0.4) |
| 28-day replays, every election (not used) | -7.67 | -7.67 | -5.51 (bloc_sd 0.33, candidate_sd 0.4) |
| 21-day replays, every election (not used) | -3.44 | -3.44 | -2.14 (bloc_sd 0.27, candidate_sd 0.3) |
| 14-day replays, every election (not used) | 1.34 | 1.34 | 1.57 (bloc_sd 0.22, candidate_sd 0.2) |
| 7-day replays, every election (not used) | 7.98 | 7.98 | 8.28 (bloc_sd 0.1, candidate_sd 0.3) |
| Total | -113.85 | -113.85 | -99.17 (bloc_sd 0.27, candidate_sd 0.4) |

Total log score by `bloc_sd` (rows) and `candidate_sd` (columns). `*` marks the points that score at least as well as the start on every source; bold is the chosen one.

| bloc_sd \ candidate_sd | 0.03 | 0.05 | 0.07 | 0.1 | 0.13 | 0.16 | 0.2 | 0.25 | 0.3 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0.04 | -659.2 | -657.9 | -655.6 | -651.7 | -647.3 | -642.9 | -636.8 | -629.9 | -623.8 | -615.1 |
| 0.06 | -572.2 | -570.9 | -569.2 | -566.0 | -562.1 | -558.2 | -553.1 | -546.9 | -541.3 | -532.5 |
| 0.08 | -387.8 | -386.7 | -385.2 | -382.6 | -379.4 | -376.1 | -371.6 | -366.0 | -361.0 | -353.8 |
| 0.1 | -176.9 | -175.9 | -174.6 | -172.3 | -169.5 | -166.3 | -161.7 | -156.2 | -151.5 | -144.4 |
| 0.12 | -165.6 | -164.8 | -163.7 | -161.5 | -159.1 | -156.3 | -152.2 | -147.4 | -143.4 | -137.0 |
| 0.15 | -139.2 | -138.5 | -137.5 | -135.7 | -133.6 | -131.3 | -127.7 | -123.6 | -120.0 | -114.1 |
| 0.18 | -123.0 | -122.5 | -121.7 | -120.2 | -118.6 | -116.7 | **-113.9*** | -110.5 | -107.6 | -102.7 |
| 0.22 | -116.8 | -116.4 | -115.8 | -114.7 | -113.4 | -111.9 | -109.5 | -106.7 | -104.2 | -100.3 |
| 0.27 | -110.9 | -110.6 | -110.2 | -109.4 | -108.4 | -107.4 | -105.7 | -103.5 | -101.8 | -99.2 |
| 0.33 | -113.7 | -113.6 | -113.3 | -112.8 | -112.2 | -111.5 | -110.4 | -109.0 | -108.0 | -106.7 |
| 0.4 | -120.9 | -120.8 | -120.7 | -120.4 | -120.0 | -119.6 | -119.1 | -118.5 | -118.1 | -117.8 |

### Pass 1: Round 2 errors

Evidence: log score of the realised runoff share. Starts from error_sd 0.12, horizon_error_sd 0.

| Evidence | Start | Chosen | Best on this source alone |
|---|---|---|---|
| 2002 replays | -65.44 | -65.44 | -43.92 (error_sd 0.4, horizon_error_sd 0.2) |
| 2007 replays | 18.76 | 18.76 | 25.09 (error_sd 0.02, horizon_error_sd 0) |
| 2012 replays | -1.76 | -1.76 | 2.18 (error_sd 0.12, horizon_error_sd 0.1) |
| 2017 replays | -10.85 | -10.85 | -6.15 (error_sd 0.15, horizon_error_sd 0.14) |
| 2022 replays | 14.13 | 14.13 | 14.61 (error_sd 0.08, horizon_error_sd 0) |
| 334-day replays, every election (not used) | -2.66 | -2.66 | -2.61 (error_sd 0.18, horizon_error_sd 0.02) |
| 304-day replays, every election (not used) | -3.77 | -3.77 | -3.34 (error_sd 0.27, horizon_error_sd 0.04) |
| 273-day replays, every election (not used) | -3.15 | -3.15 | -2.99 (error_sd 0.08, horizon_error_sd 0.07) |
| 243-day replays, every election (not used) | -2.93 | -2.93 | -2.84 (error_sd 0.02, horizon_error_sd 0.07) |
| 212-day replays, every election (not used) | -3.77 | -3.77 | -3.44 (error_sd 0.05, horizon_error_sd 0.1) |
| 182-day replays, every election (not used) | -3.79 | -3.79 | -3.14 (error_sd 0.18, horizon_error_sd 0.1) |
| 152-day replays, every election (not used) | -4.06 | -4.06 | -3.47 (error_sd 0.27, horizon_error_sd 0.07) |
| 122-day replays, every election (not used) | -2.21 | -2.21 | -1.77 (error_sd 0.22, horizon_error_sd 0.02) |
| 91-day replays, every election (not used) | -0.82 | -0.82 | -0.79 (error_sd 0.12, horizon_error_sd 0.04) |
| 84-day replays, every election (not used) | -0.74 | -0.74 | -0.73 (error_sd 0.05, horizon_error_sd 0.07) |
| 77-day replays, every election (not used) | -1.04 | -1.04 | -1.02 (error_sd 0.12, horizon_error_sd 0.04) |
| 70-day replays, every election (not used) | -1.39 | -1.39 | -1.33 (error_sd 0.15, horizon_error_sd 0.02) |
| 63-day replays, every election (not used) | -1.34 | -1.34 | -1.24 (error_sd 0.02, horizon_error_sd 0.1) |
| 56-day replays, every election (not used) | -2.12 | -2.12 | -1.75 (error_sd 0.12, horizon_error_sd 0.1) |
| 49-day replays, every election (not used) | -2.53 | -2.53 | -2.02 (error_sd 0.08, horizon_error_sd 0.14) |
| 42-day replays, every election (not used) | -1.92 | -1.92 | -1.67 (error_sd 0.12, horizon_error_sd 0.1) |
| 35-day replays, every election (not used) | -1.36 | -1.36 | -1.28 (error_sd 0.15, horizon_error_sd 0.02) |
| 28-day replays, every election (not used) | -0.61 | -0.61 | -0.60 (error_sd 0.1, horizon_error_sd 0.04) |
| 21-day replays, every election (not used) | -1.11 | -1.11 | -1.10 (error_sd 0.12, horizon_error_sd 0.04) |
| 14-day replays, every election (not used) | -1.23 | -1.23 | -1.21 (error_sd 0.02, horizon_error_sd 0.14) |
| 7-day replays, every election (not used) | -2.60 | -2.60 | -2.07 (error_sd 0.18, horizon_error_sd 0.1) |
| Total | -45.17 | -45.17 | -41.69 (error_sd 0.12, horizon_error_sd 0.07) |

Total log score by `error_sd` (rows) and `horizon_error_sd` (columns). `*` marks the points that score at least as well as the start on every source; bold is the chosen one.

| error_sd \ horizon_error_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 |
|---|---|---|---|---|---|---|---|
| 0.02 | -70.8 | -66.5 | -57.7 | -47.2 | -43.2 | -44.2 | -51.4 |
| 0.03 | -68.1 | -64.3 | -56.3 | -46.6 | -43.0 | -44.3 | -51.5 |
| 0.05 | -61.6 | -58.7 | -52.7 | -45.2 | -42.6 | -44.3 | -51.7 |
| 0.08 | -52.5 | -50.9 | -47.4 | -43.2 | -42.1 | -44.6 | -52.1 |
| 0.1 | -48.2 | -47.1 | -44.8 | -42.2 | -42.0 | -44.9 | -52.5 |
| 0.12 | **-45.2*** | -44.5 | -43.1 | -41.7 | -42.2 | -45.4 | -53.0 |
| 0.15 | -42.8 | -42.5 | -41.9 | -41.7 | -42.9 | -46.4 | -54.0 |
| 0.18 | -42.2 | -42.1 | -42.0 | -42.5 | -44.0 | -47.7 | -55.1 |
| 0.22 | -43.1 | -43.2 | -43.4 | -44.3 | -46.1 | -49.7 | -56.9 |
| 0.27 | -45.7 | -45.8 | -46.2 | -47.3 | -49.1 | -52.6 | -59.4 |
| 0.33 | -49.8 | -49.9 | -50.3 | -51.5 | -53.2 | -56.5 | -62.7 |
| 0.4 | -55.1 | -55.2 | -55.6 | -56.6 | -58.3 | -61.2 | -66.8 |

## Leave one election out

The whole fit rerun without one election's replays, then that election's replays scored at the setting it gives, and at the current one.

| Held out | Passes | bloc_sd | candidate_sd | horizon_bloc_sd | horizon_candidate_sd | student_t_df | error_sd | horizon_error_sd | Round 1 log score (chosen without / current) | Round 2 log score (chosen without / current) |
|---|---|---|---|---|---|---|---|---|---|---|
| 2002 | 2 | 0.18 | 0.2 | 0.02 | 0.33 | 2.5 | 0.12 | 0 | -24.47 / -24.07 | -66.72 / -65.44 |
| 2007 | 1 | 0.18 | 0.2 | 0 | 0.2 | 3 | 0.12 | 0 | 35.47 / 35.47 | 18.76 / 18.76 |
| 2012 | 1 | 0.18 | 0.2 | 0 | 0.2 | 3 | 0.12 | 0 | 28.03 / 28.03 | -1.76 / -1.76 |
| 2017 | 1 | 0.18 | 0.2 | 0 | 0.2 | 3 | 0.12 | 0 | -21.48 / -21.48 | -10.85 / -10.85 |
| 2022 | 1 | 0.18 | 0.2 | 0 | 0.2 | 3 | 0.12 | 0 | -131.81 / -131.81 | 14.13 / 14.13 |

## Replays

| Election | Days out | As of | Polls used | Overlapping dropped | Random walk sd | Design effect | Contenders | Runners not yet polled |
|---|---|---|---|---|---|---|---|---|
| 2002 | 334 | 2001-05-22 | 3 | 0 | 0.002 | 1.5 | jospin, chirac | besancenot, gluckstein, taubira, lepage, boutin, saint_josse |
| 2002 | 304 | 2001-06-21 | 6 | 0 | 0.002 | 2 | jospin, chirac | besancenot, gluckstein, taubira, lepage, saint_josse |
| 2002 | 273 | 2001-07-22 | 10 | 0 | 0.002 | 1.5 | jospin, chirac | gluckstein, taubira, saint_josse |
| 2002 | 243 | 2001-08-21 | 10 | 0 | 0.002 | 1.5 | jospin, chirac | gluckstein, taubira, saint_josse |
| 2002 | 212 | 2001-09-21 | 13 | 0 | 0.012 | 1.5 | jospin, chirac | gluckstein, taubira, saint_josse |
| 2002 | 182 | 2001-10-21 | 18 | 0 | 0.02 | 1.5 | jospin, chirac | gluckstein, taubira, saint_josse |
| 2002 | 152 | 2001-11-20 | 21 | 0 | 0.02 | 1.5 | jospin, chevenement, chirac | gluckstein, taubira, saint_josse |
| 2002 | 122 | 2001-12-20 | 27 | 0 | 0.02 | 1 | jospin, chevenement, chirac | gluckstein, saint_josse |
| 2002 | 91 | 2002-01-20 | 37 | 0 | 0.02 | 1 | jospin, chevenement, chirac | saint_josse |
| 2002 | 84 | 2002-01-27 | 38 | 0 | 0.02 | 1 | jospin, chevenement, chirac | saint_josse |
| 2002 | 77 | 2002-02-03 | 44 | 0 | 0.02 | 1 | jospin, chevenement, chirac | saint_josse |
| 2002 | 70 | 2002-02-10 | 46 | 0 | 0.02 | 1 | jospin, chevenement, chirac | – |
| 2002 | 63 | 2002-02-17 | 51 | 0 | 0.02 | 1 | jospin, chevenement, chirac | – |
| 2002 | 56 | 2002-02-24 | 54 | 1 | 0.02 | 1 | jospin, chevenement, chirac | – |
| 2002 | 49 | 2002-03-03 | 58 | 1 | 0.02 | 1 | jospin, chevenement, chirac | – |
| 2002 | 42 | 2002-03-10 | 61 | 1 | 0.02 | 1 | jospin, chirac | – |
| 2002 | 35 | 2002-03-17 | 66 | 2 | 0.02 | 1 | jospin, chirac | – |
| 2002 | 28 | 2002-03-24 | 70 | 2 | 0.02 | 1 | jospin, chirac, jm_le_pen | – |
| 2002 | 21 | 2002-03-31 | 77 | 2 | 0.02 | 1 | jospin, chirac, jm_le_pen | – |
| 2002 | 14 | 2002-04-07 | 82 | 3 | 0.02 | 1 | jospin, chirac, jm_le_pen | – |
| 2002 | 7 | 2002-04-14 | 86 | 4 | 0.02 | 1 | jospin, chirac, jm_le_pen | – |
| 2007 | 334 | 2006-05-23 | 3 | 0 | 0.03 | 2 | royal, sarkozy, jm_le_pen | schivardi, nihous |
| 2007 | 304 | 2006-06-22 | 6 | 0 | 0.002 | 2 | royal, sarkozy, jm_le_pen | schivardi, nihous |
| 2007 | 273 | 2006-07-23 | 8 | 0 | 0.002 | 1.5 | royal, sarkozy, jm_le_pen | schivardi, nihous |
| 2007 | 243 | 2006-08-22 | 9 | 0 | 0.004 | 2 | royal, sarkozy, jm_le_pen | schivardi, nihous |
| 2007 | 212 | 2006-09-22 | 12 | 0 | 0.012 | 1.5 | royal, sarkozy, jm_le_pen | schivardi |
| 2007 | 182 | 2006-10-22 | 17 | 0 | 0.004 | 2 | royal, sarkozy, jm_le_pen | schivardi |
| 2007 | 152 | 2006-11-21 | 21 | 0 | 0.007 | 2 | royal, sarkozy, jm_le_pen | schivardi |
| 2007 | 122 | 2006-12-21 | 29 | 0 | 0.007 | 2 | royal, sarkozy, jm_le_pen | schivardi |
| 2007 | 91 | 2007-01-21 | 39 | 0 | 0.012 | 1.5 | royal, sarkozy, jm_le_pen | – |
| 2007 | 84 | 2007-01-28 | 43 | 0 | 0.012 | 1.5 | royal, sarkozy, jm_le_pen | – |
| 2007 | 77 | 2007-02-04 | 47 | 0 | 0.012 | 1.5 | royal, bayrou, sarkozy, jm_le_pen | – |
| 2007 | 70 | 2007-02-11 | 51 | 0 | 0.012 | 1.5 | royal, bayrou, sarkozy, jm_le_pen | – |
| 2007 | 63 | 2007-02-18 | 60 | 0 | 0.012 | 1.5 | royal, bayrou, sarkozy, jm_le_pen | – |
| 2007 | 56 | 2007-02-25 | 65 | 0 | 0.02 | 1.5 | royal, bayrou, sarkozy, jm_le_pen | – |
| 2007 | 49 | 2007-03-04 | 72 | 3 | 0.02 | 1.5 | royal, bayrou, sarkozy, jm_le_pen | – |
| 2007 | 42 | 2007-03-11 | 79 | 7 | 0.02 | 1.5 | royal, bayrou, sarkozy, jm_le_pen | – |
| 2007 | 35 | 2007-03-18 | 86 | 11 | 0.02 | 1.5 | royal, bayrou, sarkozy, jm_le_pen | – |
| 2007 | 28 | 2007-03-25 | 94 | 15 | 0.02 | 1.5 | royal, bayrou, sarkozy, jm_le_pen | – |
| 2007 | 21 | 2007-04-01 | 101 | 19 | 0.02 | 1.5 | royal, bayrou, sarkozy, jm_le_pen | – |
| 2007 | 14 | 2007-04-08 | 109 | 23 | 0.02 | 1.5 | royal, bayrou, sarkozy, jm_le_pen | – |
| 2007 | 7 | 2007-04-15 | 115 | 28 | 0.02 | 1.5 | royal, bayrou, sarkozy, jm_le_pen | – |
| 2012 | 334 | 2011-05-24 | 9 | 7 | 0.002 | 3 | hollande, bayrou, sarkozy, le_pen | cheminade |
| 2012 | 304 | 2011-06-23 | 13 | 7 | 0.007 | 3 | hollande, bayrou, sarkozy, le_pen | cheminade |
| 2012 | 273 | 2011-07-24 | 19 | 7 | 0.012 | 3 | hollande, bayrou, sarkozy, le_pen | cheminade |
| 2012 | 243 | 2011-08-23 | 19 | 7 | 0.012 | 3 | hollande, bayrou, sarkozy, le_pen | cheminade |
| 2012 | 212 | 2011-09-23 | 25 | 7 | 0.012 | 3 | hollande, bayrou, sarkozy, le_pen | cheminade |
| 2012 | 182 | 2011-10-23 | 31 | 7 | 0.02 | 1.5 | hollande, sarkozy, le_pen | cheminade |
| 2012 | 152 | 2011-11-22 | 38 | 7 | 0.02 | 1.5 | hollande, sarkozy, le_pen | cheminade |
| 2012 | 122 | 2011-12-22 | 49 | 7 | 0.02 | 1.5 | hollande, bayrou, sarkozy, le_pen | cheminade |
| 2012 | 91 | 2012-01-22 | 60 | 12 | 0.02 | 1.5 | hollande, bayrou, sarkozy, le_pen | cheminade |
| 2012 | 84 | 2012-01-29 | 63 | 16 | 0.02 | 1.5 | hollande, bayrou, sarkozy, le_pen | cheminade |
| 2012 | 77 | 2012-02-05 | 68 | 20 | 0.02 | 1.5 | hollande, bayrou, sarkozy, le_pen | cheminade |
| 2012 | 70 | 2012-02-12 | 71 | 24 | 0.02 | 1.5 | hollande, bayrou, sarkozy, le_pen | cheminade |
| 2012 | 63 | 2012-02-19 | 78 | 28 | 0.02 | 1.5 | hollande, bayrou, sarkozy, le_pen | – |
| 2012 | 56 | 2012-02-26 | 81 | 32 | 0.02 | 1.5 | hollande, bayrou, sarkozy, le_pen | – |
| 2012 | 49 | 2012-03-04 | 87 | 36 | 0.02 | 1.5 | hollande, bayrou, sarkozy, le_pen | – |
| 2012 | 42 | 2012-03-11 | 91 | 40 | 0.02 | 1.5 | hollande, bayrou, sarkozy, le_pen | – |
| 2012 | 35 | 2012-03-18 | 98 | 45 | 0.02 | 1.5 | melenchon, hollande, bayrou, sarkozy, le_pen | – |
| 2012 | 28 | 2012-03-25 | 103 | 49 | 0.02 | 1.5 | melenchon, hollande, bayrou, sarkozy, le_pen | – |
| 2012 | 21 | 2012-04-01 | 112 | 53 | 0.03 | 1 | melenchon, hollande, bayrou, sarkozy, le_pen | – |
| 2012 | 14 | 2012-04-08 | 119 | 58 | 0.03 | 1 | melenchon, hollande, bayrou, sarkozy, le_pen | – |
| 2012 | 7 | 2012-04-15 | 126 | 61 | 0.03 | 1 | melenchon, hollande, bayrou, sarkozy, le_pen | – |
| 2017 | 334 | 2016-05-24 | 4 | 0 | 0.002 | 1.5 | melenchon, macron, fillon, le_pen | hamon, lassalle, asselineau |
| 2017 | 304 | 2016-06-23 | 7 | 0 | 0.002 | 2 | melenchon, macron, fillon, le_pen | hamon |
| 2017 | 273 | 2016-07-24 | 8 | 0 | 0.002 | 2 | melenchon, macron, fillon, le_pen | hamon |
| 2017 | 243 | 2016-08-23 | 8 | 0 | 0.002 | 2 | melenchon, macron, fillon, le_pen | hamon |
| 2017 | 212 | 2016-09-23 | 13 | 0 | 0.002 | 2 | melenchon, macron, fillon, le_pen | hamon |
| 2017 | 182 | 2016-10-23 | 16 | 0 | 0.002 | 2 | melenchon, macron, fillon, le_pen | hamon |
| 2017 | 152 | 2016-11-22 | 19 | 0 | 0.007 | 2 | melenchon, macron, fillon, le_pen | hamon |
| 2017 | 122 | 2016-12-22 | 27 | 0 | 0.02 | 2 | melenchon, macron, fillon, le_pen | – |
| 2017 | 91 | 2017-01-22 | 32 | 0 | 0.02 | 2 | melenchon, macron, fillon, le_pen | – |
| 2017 | 84 | 2017-01-29 | 33 | 0 | 0.02 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 77 | 2017-02-05 | 36 | 3 | 0.02 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 70 | 2017-02-12 | 43 | 9 | 0.03 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 63 | 2017-02-19 | 47 | 16 | 0.03 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 56 | 2017-02-26 | 56 | 22 | 0.03 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 49 | 2017-03-05 | 64 | 30 | 0.02 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 42 | 2017-03-12 | 72 | 37 | 0.02 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 35 | 2017-03-19 | 81 | 45 | 0.02 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 28 | 2017-03-26 | 89 | 51 | 0.02 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 21 | 2017-04-02 | 97 | 62 | 0.02 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 14 | 2017-04-09 | 108 | 69 | 0.02 | 2 | melenchon, macron, fillon, le_pen | – |
| 2017 | 7 | 2017-04-16 | 120 | 83 | 0.03 | 2 | melenchon, macron, fillon, le_pen | – |
| 2022 | 334 | 2021-05-11 | 2 | 0 | 0.002 | 2 | melenchon, macron, pecresse, le_pen | zemmour |
| 2022 | 304 | 2021-06-10 | 8 | 0 | 0.012 | 1.5 | melenchon, macron, le_pen | – |
| 2022 | 273 | 2021-07-11 | 14 | 0 | 0.012 | 1.5 | macron, pecresse, le_pen | – |
| 2022 | 243 | 2021-08-10 | 14 | 0 | 0.012 | 1.5 | macron, pecresse, le_pen | – |
| 2022 | 212 | 2021-09-10 | 18 | 0 | 0.007 | 1.5 | melenchon, macron, pecresse, le_pen | – |
| 2022 | 182 | 2021-10-10 | 30 | 0 | 0.02 | 2 | macron, pecresse, le_pen, zemmour | – |
| 2022 | 152 | 2021-11-09 | 44 | 0 | 0.02 | 2 | macron, pecresse, le_pen, zemmour | – |
| 2022 | 122 | 2021-12-09 | 57 | 0 | 0.02 | 2 | macron, pecresse, le_pen, zemmour | – |
| 2022 | 91 | 2022-01-09 | 74 | 4 | 0.02 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 84 | 2022-01-16 | 80 | 11 | 0.02 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 77 | 2022-01-23 | 86 | 18 | 0.02 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 70 | 2022-01-30 | 93 | 25 | 0.02 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 63 | 2022-02-06 | 101 | 26 | 0.02 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 56 | 2022-02-13 | 107 | 41 | 0.02 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 49 | 2022-02-20 | 115 | 41 | 0.02 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 42 | 2022-02-27 | 121 | 55 | 0.02 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 35 | 2022-03-06 | 131 | 55 | 0.02 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 28 | 2022-03-13 | 140 | 69 | 0.02 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 21 | 2022-03-20 | 149 | 83 | 0.02 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 14 | 2022-03-27 | 159 | 96 | 0.02 | 2 | melenchon, macron, pecresse, le_pen, zemmour | – |
| 2022 | 7 | 2022-04-03 | 172 | 110 | 0.02 | 2 | melenchon, macron, le_pen, zemmour | – |

### 2002, 334 days out (2001-05-22)

Actual matchup: current 0.3% (winner given matchup 78.0%), fitted 0.3% (winner given matchup 78.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 27.0% (20.3%–33.6%) | 96.8% / 96.8% ✓ | 49.3% / 49.3% ✓ |
| jm_le_pen | 16.9% | 7.9% (5.3%–10.6%) | 0.4% / 0.4% ✓ | 0.1% / 0.1% |
| jospin | 16.2% | 25.3% (15.4%–34.6%) | 90.2% / 90.2% | 46.0% / 46.0% |
| bayrou | 6.8% | 6.7% (5.1%–8.5%) | 0.1% / 0.1% | 0.1% / 0.1% |
| laguiller | 5.7% | 6.6% (5.0%–8.3%) | 0.1% / 0.1% | 0.0% / 0.0% |
| chevenement | 5.3% | 5.3% (1.8%–9.3%) | 2.1% / 2.1% | 0.7% / 0.7% |
| mamere | 5.2% | 7.0% (2.5%–12.2%) | 4.0% / 4.0% | 1.6% / 1.6% |
| besancenot | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| madelin | 3.9% | 5.4% (1.7%–10.1%) | 2.7% / 2.7% | 0.9% / 0.9% |
| hue | 3.4% | 6.6% (2.3%–11.6%) | 3.5% / 3.5% | 1.3% / 1.3% |
| megret | 2.3% | 2.2% (0.7%–4.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 304 days out (2001-06-21)

Actual matchup: current 0.2% (winner given matchup 76.7%), fitted 0.2% (winner given matchup 76.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 24.7% (17.6%–31.5%) | 95.7% / 95.7% ✓ | 52.3% / 52.3% ✓ |
| jm_le_pen | 16.9% | 7.6% (5.2%–10.1%) | 0.3% / 0.3% ✓ | 0.1% / 0.1% |
| jospin | 16.2% | 25.5% (15.9%–34.5%) | 91.3% / 91.3% | 43.1% / 43.1% |
| bayrou | 6.8% | 6.1% (4.7%–7.6%) | 0.1% / 0.1% | 0.1% / 0.1% |
| laguiller | 5.7% | 6.7% (5.1%–8.3%) | 0.2% / 0.2% | 0.0% / 0.0% |
| chevenement | 5.3% | 5.2% (1.9%–9.1%) | 1.9% / 1.9% | 0.7% / 0.7% |
| mamere | 5.2% | 6.6% (2.5%–11.3%) | 3.3% / 3.3% | 1.2% / 1.2% |
| besancenot | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| madelin | 3.9% | 4.8% (1.6%–8.7%) | 1.9% / 1.9% | 0.7% / 0.7% |
| hue | 3.4% | 7.1% (2.7%–12.3%) | 4.1% / 4.1% | 1.4% / 1.4% |
| megret | 2.3% | 2.1% (0.7%–3.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 3.5% (1.1%–6.5%) | 1.2% / 1.2% | 0.5% / 0.5% |
| gluckstein | 0.5% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 273 days out (2001-07-22)

Actual matchup: current 0.2% (winner given matchup 81.1%), fitted 0.2% (winner given matchup 81.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 24.8% (18.0%–31.3%) | 96.3% / 96.3% ✓ | 53.1% / 53.1% ✓ |
| jm_le_pen | 16.9% | 7.3% (5.1%–9.6%) | 0.3% / 0.3% ✓ | 0.0% / 0.0% |
| jospin | 16.2% | 24.9% (15.9%–33.6%) | 91.8% / 91.8% | 42.8% / 42.8% |
| bayrou | 6.8% | 4.5% (2.6%–6.5%) | 0.1% / 0.1% | 0.0% / 0.0% |
| laguiller | 5.7% | 6.2% (4.1%–8.5%) | 0.2% / 0.2% | 0.0% / 0.0% |
| chevenement | 5.3% | 5.6% (2.2%–9.4%) | 2.0% / 2.0% | 0.7% / 0.7% |
| mamere | 5.2% | 6.4% (2.6%–10.8%) | 3.0% / 3.0% | 1.1% / 1.1% |
| besancenot | 4.2% | 1.3% (0.3%–2.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| madelin | 3.9% | 5.4% (2.0%–9.4%) | 2.3% / 2.3% | 0.8% / 0.8% |
| hue | 3.4% | 6.9% (2.8%–11.6%) | 3.6% / 3.6% | 1.3% / 1.3% |
| megret | 2.3% | 2.0% (0.7%–3.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 2.1% (0.7%–3.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 2.7% (0.9%–4.8%) | 0.6% / 0.6% | 0.2% / 0.2% |
| gluckstein | 0.5% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 243 days out (2001-08-21)

Actual matchup: current 0.2% (winner given matchup 76.5%), fitted 0.2% (winner given matchup 76.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 24.9% (18.3%–31.2%) | 96.8% / 96.8% ✓ | 53.3% / 53.3% ✓ |
| jm_le_pen | 16.9% | 7.3% (5.1%–9.5%) | 0.2% / 0.2% ✓ | 0.0% / 0.0% |
| jospin | 16.2% | 25.0% (16.3%–33.4%) | 92.8% / 92.8% | 43.1% / 43.1% |
| bayrou | 6.8% | 4.5% (2.6%–6.5%) | 0.1% / 0.1% | 0.0% / 0.0% |
| laguiller | 5.7% | 6.3% (4.2%–8.5%) | 0.2% / 0.2% | 0.1% / 0.1% |
| chevenement | 5.3% | 5.5% (2.3%–9.2%) | 1.7% / 1.7% | 0.6% / 0.6% |
| mamere | 5.2% | 6.4% (2.7%–10.6%) | 2.6% / 2.6% | 0.9% / 0.9% |
| besancenot | 4.2% | 1.2% (0.3%–2.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| madelin | 3.9% | 5.3% (2.1%–9.2%) | 1.9% / 1.9% | 0.6% / 0.6% |
| hue | 3.4% | 6.8% (2.9%–11.4%) | 3.1% / 3.1% | 1.1% / 1.1% |
| megret | 2.3% | 2.0% (0.8%–3.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 2.1% (0.8%–3.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 2.6% (0.9%–4.7%) | 0.4% / 0.4% | 0.2% / 0.2% |
| gluckstein | 0.5% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 212 days out (2001-09-21)

Actual matchup: current 0.4% (winner given matchup 89.6%), fitted 0.4% (winner given matchup 89.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 24.4% (16.6%–32.6%) | 95.0% / 95.0% ✓ | 52.9% / 52.9% ✓ |
| jm_le_pen | 16.9% | 7.1% (4.4%–10.0%) | 0.6% / 0.6% ✓ | 0.0% / 0.0% |
| jospin | 16.2% | 23.9% (14.5%–33.7%) | 90.4% / 90.4% | 42.0% / 42.0% |
| bayrou | 6.8% | 4.5% (2.3%–6.9%) | 0.1% / 0.1% | 0.1% / 0.1% |
| laguiller | 5.7% | 6.5% (4.1%–9.2%) | 0.4% / 0.4% | 0.1% / 0.1% |
| chevenement | 5.3% | 7.6% (3.2%–12.8%) | 4.4% / 4.4% | 1.7% / 1.7% |
| mamere | 5.2% | 6.8% (2.7%–11.6%) | 3.5% / 3.5% | 1.3% / 1.3% |
| besancenot | 4.2% | 1.2% (0.4%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| madelin | 3.9% | 5.6% (2.1%–9.9%) | 2.5% / 2.5% | 0.8% / 0.8% |
| hue | 3.4% | 6.4% (2.6%–10.9%) | 2.9% / 2.9% | 1.0% / 1.0% |
| megret | 2.3% | 1.8% (0.6%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 2.1% (0.7%–3.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 2.0% (0.7%–3.7%) | 0.3% / 0.3% | 0.2% / 0.2% |
| gluckstein | 0.5% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 182 days out (2001-10-21)

Actual matchup: current 1.3% (winner given matchup 90.3%), fitted 1.3% (winner given matchup 90.3%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 24.6% (15.5%–34.5%) | 91.0% / 91.0% ✓ | 57.6% / 57.6% ✓ |
| jm_le_pen | 16.9% | 7.6% (4.3%–11.5%) | 1.9% / 1.9% ✓ | 0.2% / 0.2% |
| jospin | 16.2% | 22.8% (12.6%–33.6%) | 84.3% / 84.3% | 34.7% / 34.7% |
| bayrou | 6.8% | 5.1% (2.8%–7.8%) | 0.3% / 0.3% | 0.1% / 0.1% |
| laguiller | 5.7% | 6.4% (3.6%–9.5%) | 0.7% / 0.7% | 0.1% / 0.1% |
| chevenement | 5.3% | 10.0% (4.2%–16.8%) | 12.2% / 12.2% | 4.1% / 4.1% |
| mamere | 5.2% | 7.0% (2.7%–12.3%) | 4.2% / 4.2% | 1.4% / 1.4% |
| besancenot | 4.2% | 1.0% (0.3%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| madelin | 3.9% | 5.5% (1.9%–10.2%) | 2.7% / 2.7% | 0.9% / 0.9% |
| hue | 3.4% | 6.0% (2.3%–10.6%) | 2.6% / 2.6% | 0.8% / 0.8% |
| megret | 2.3% | 1.6% (0.5%–3.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 1.1% (0.3%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 1.2% (0.3%–2.3%) | 0.1% / 0.1% | 0.1% / 0.1% |
| gluckstein | 0.5% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 152 days out (2001-11-20)

Actual matchup: current 1.8% (winner given matchup 86.7%), fitted 1.8% (winner given matchup 86.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 24.3% (15.8%–33.6%) | 90.7% / 90.7% ✓ | 61.7% / 61.7% ✓ |
| jm_le_pen | 16.9% | 8.4% (5.0%–12.4%) | 2.6% / 2.6% ✓ | 0.3% / 0.3% |
| jospin | 16.2% | 22.4% (12.8%–32.7%) | 83.5% / 83.5% | 30.5% / 30.5% |
| bayrou | 6.8% | 4.6% (2.6%–6.9%) | 0.1% / 0.1% | 0.1% / 0.1% |
| laguiller | 5.7% | 6.2% (3.6%–9.2%) | 0.5% / 0.5% | 0.1% / 0.1% |
| chevenement | 5.3% | 10.9% (5.0%–17.7%) | 14.1% / 14.1% | 4.6% / 4.6% |
| mamere | 5.2% | 7.6% (3.2%–12.8%) | 4.5% / 4.5% | 1.4% / 1.4% |
| besancenot | 4.2% | 0.9% (0.3%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| madelin | 3.9% | 5.0% (1.8%–9.0%) | 1.7% / 1.7% | 0.6% / 0.6% |
| hue | 3.4% | 6.0% (2.5%–10.2%) | 2.1% / 2.1% | 0.7% / 0.7% |
| megret | 2.3% | 1.6% (0.5%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 1.0% (0.3%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 1.1% (0.3%–1.9%) | 0.1% / 0.1% | 0.1% / 0.1% |
| gluckstein | 0.5% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 122 days out (2001-12-20)

Actual matchup: current 1.5% (winner given matchup 88.1%), fitted 1.5% (winner given matchup 88.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 24.4% (16.4%–33.1%) | 92.9% / 92.9% ✓ | 58.5% / 58.5% ✓ |
| jm_le_pen | 16.9% | 8.3% (5.2%–11.9%) | 1.9% / 1.9% ✓ | 0.2% / 0.2% |
| jospin | 16.2% | 22.3% (13.4%–31.7%) | 86.4% / 86.4% | 38.2% / 38.2% |
| bayrou | 6.8% | 4.2% (2.5%–6.1%) | 0.1% / 0.1% | 0.0% / 0.0% |
| laguiller | 5.7% | 6.6% (4.1%–9.4%) | 0.5% / 0.5% | 0.1% / 0.1% |
| chevenement | 5.3% | 10.5% (5.2%–16.6%) | 10.8% / 10.8% | 0.8% / 0.8% |
| mamere | 5.2% | 7.9% (3.7%–12.8%) | 4.0% / 4.0% | 1.2% / 1.2% |
| besancenot | 4.2% | 0.8% (0.2%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| madelin | 3.9% | 4.9% (1.9%–8.5%) | 1.4% / 1.4% | 0.4% / 0.4% |
| hue | 3.4% | 6.3% (2.9%–10.3%) | 1.9% / 1.9% | 0.5% / 0.5% |
| megret | 2.3% | 1.4% (0.5%–2.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.7% (0.2%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 1.0% (0.4%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 0.8% (0.3%–1.4%) | 0.1% / 0.1% | 0.0% / 0.0% |
| gluckstein | 0.5% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 91 days out (2002-01-20)

Actual matchup: current 2.0% (winner given matchup 86.3%), fitted 2.0% (winner given matchup 86.3%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 25.4% (18.1%–33.3%) | 95.9% / 95.9% ✓ | 65.3% / 65.3% ✓ |
| jm_le_pen | 16.9% | 9.2% (6.1%–12.7%) | 2.4% / 2.4% ✓ | 0.3% / 0.3% |
| jospin | 16.2% | 21.5% (13.8%–29.6%) | 87.7% / 87.7% | 32.5% / 32.5% |
| bayrou | 6.8% | 4.2% (2.7%–6.0%) | 0.1% / 0.1% | 0.0% / 0.0% |
| laguiller | 5.7% | 6.0% (3.8%–8.6%) | 0.3% / 0.3% | 0.1% / 0.1% |
| chevenement | 5.3% | 10.8% (5.9%–16.3%) | 9.8% / 9.8% | 0.6% / 0.6% |
| mamere | 5.2% | 7.3% (3.7%–11.3%) | 2.1% / 2.1% | 0.7% / 0.7% |
| besancenot | 4.2% | 0.6% (0.2%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| madelin | 3.9% | 4.7% (2.1%–7.8%) | 0.8% / 0.8% | 0.3% / 0.3% |
| hue | 3.4% | 5.7% (2.9%–8.9%) | 0.9% / 0.9% | 0.2% / 0.2% |
| megret | 2.3% | 1.6% (0.6%–2.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.6% (0.2%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 0.9% (0.4%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 0.8% (0.3%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.7% (0.2%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 84 days out (2002-01-27)

Actual matchup: current 2.0% (winner given matchup 85.5%), fitted 2.0% (winner given matchup 85.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 25.0% (17.9%–32.7%) | 95.5% / 95.5% ✓ | 66.1% / 66.1% ✓ |
| jm_le_pen | 16.9% | 9.4% (6.3%–12.9%) | 2.5% / 2.5% ✓ | 0.3% / 0.3% |
| jospin | 16.2% | 21.5% (14.0%–29.4%) | 87.7% / 87.7% | 31.8% / 31.8% |
| bayrou | 6.8% | 4.3% (2.7%–6.0%) | 0.1% / 0.1% | 0.0% / 0.0% |
| laguiller | 5.7% | 6.3% (3.9%–8.9%) | 0.3% / 0.3% | 0.1% / 0.1% |
| chevenement | 5.3% | 11.3% (6.3%–16.8%) | 11.0% / 11.0% | 0.7% / 0.7% |
| mamere | 5.2% | 6.7% (3.5%–10.3%) | 1.5% / 1.5% | 0.5% / 0.5% |
| besancenot | 4.2% | 0.6% (0.2%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| madelin | 3.9% | 4.8% (2.2%–7.9%) | 0.8% / 0.8% | 0.3% / 0.3% |
| hue | 3.4% | 5.5% (2.8%–8.6%) | 0.7% / 0.7% | 0.2% / 0.2% |
| megret | 2.3% | 1.6% (0.7%–2.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.5% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 0.9% (0.4%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 0.8% (0.3%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.7% (0.2%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 77 days out (2002-02-03)

Actual matchup: current 1.5% (winner given matchup 83.1%), fitted 1.5% (winner given matchup 83.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 22.9% (16.4%–29.9%) | 92.5% / 92.5% ✓ | 54.3% / 54.3% ✓ |
| jm_le_pen | 16.9% | 9.6% (6.5%–12.9%) | 2.3% / 2.3% ✓ | 0.3% / 0.3% |
| jospin | 16.2% | 22.4% (14.9%–30.2%) | 88.9% / 88.9% | 43.4% / 43.4% |
| bayrou | 6.8% | 4.1% (2.7%–5.7%) | 0.1% / 0.1% | 0.0% / 0.0% |
| laguiller | 5.7% | 6.3% (4.1%–8.8%) | 0.3% / 0.3% | 0.1% / 0.1% |
| chevenement | 5.3% | 12.1% (7.0%–17.6%) | 13.5% / 13.5% | 1.0% / 1.0% |
| mamere | 5.2% | 6.6% (3.5%–10.0%) | 1.2% / 1.2% | 0.4% / 0.4% |
| besancenot | 4.2% | 0.7% (0.3%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| madelin | 3.9% | 5.0% (2.4%–8.0%) | 0.7% / 0.7% | 0.2% / 0.2% |
| hue | 3.4% | 5.6% (3.0%–8.7%) | 0.7% / 0.7% | 0.2% / 0.2% |
| megret | 2.3% | 1.6% (0.7%–2.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.7% (0.3%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 1.0% (0.4%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.6% (0.2%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 70 days out (2002-02-10)

Actual matchup: current 1.6% (winner given matchup 79.4%), fitted 1.6% (winner given matchup 79.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 21.7% (15.6%–28.1%) | 91.7% / 91.7% ✓ | 57.1% / 57.1% ✓ |
| jm_le_pen | 16.9% | 9.1% (6.2%–12.2%) | 2.2% / 2.2% ✓ | 0.4% / 0.4% |
| jospin | 16.2% | 20.7% (13.7%–27.9%) | 86.5% / 86.5% | 40.2% / 40.2% |
| bayrou | 6.8% | 3.7% (2.4%–5.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 5.7% | 6.5% (4.3%–8.9%) | 0.4% / 0.4% | 0.1% / 0.1% |
| chevenement | 5.3% | 12.2% (7.2%–17.6%) | 16.6% / 16.6% | 1.3% / 1.3% |
| mamere | 5.2% | 6.5% (3.6%–9.8%) | 1.2% / 1.2% | 0.4% / 0.4% |
| besancenot | 4.2% | 0.8% (0.3%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | 4.2% (2.3%–6.4%) | 0.1% / 0.1% | 0.1% / 0.1% |
| madelin | 3.9% | 4.5% (2.2%–7.2%) | 0.5% / 0.5% | 0.2% / 0.2% |
| hue | 3.4% | 5.8% (3.2%–8.8%) | 0.8% / 0.8% | 0.2% / 0.2% |
| megret | 2.3% | 1.5% (0.7%–2.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 0.9% (0.4%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.6% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 63 days out (2002-02-17)

Actual matchup: current 1.8% (winner given matchup 82.8%), fitted 1.8% (winner given matchup 82.8%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 23.7% (17.5%–30.2%) | 96.6% / 96.6% ✓ | 65.2% / 65.2% ✓ |
| jm_le_pen | 16.9% | 9.3% (6.5%–12.4%) | 2.2% / 2.2% ✓ | 0.3% / 0.3% |
| jospin | 16.2% | 20.6% (14.0%–27.4%) | 89.9% / 89.9% | 33.1% / 33.1% |
| bayrou | 6.8% | 3.4% (2.3%–4.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 5.7% | 6.6% (4.4%–9.0%) | 0.3% / 0.3% | 0.1% / 0.1% |
| chevenement | 5.3% | 10.8% (6.5%–15.6%) | 9.0% / 9.0% | 0.6% / 0.6% |
| mamere | 5.2% | 6.0% (3.3%–8.9%) | 0.7% / 0.7% | 0.2% / 0.2% |
| besancenot | 4.2% | 0.8% (0.3%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | 5.1% (2.9%–7.6%) | 0.2% / 0.2% | 0.1% / 0.1% |
| madelin | 3.9% | 4.0% (2.0%–6.3%) | 0.3% / 0.3% | 0.1% / 0.1% |
| hue | 3.4% | 5.6% (3.1%–8.4%) | 0.6% / 0.6% | 0.1% / 0.1% |
| megret | 2.3% | 1.4% (0.7%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 0.9% (0.4%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.6% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 56 days out (2002-02-24)

Actual matchup: current 1.2% (winner given matchup 80.0%), fitted 1.2% (winner given matchup 80.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 23.3% (17.4%–29.6%) | 96.3% / 96.3% ✓ | 56.7% / 56.7% ✓ |
| jm_le_pen | 16.9% | 9.2% (6.5%–12.1%) | 1.6% / 1.6% ✓ | 0.3% / 0.3% |
| jospin | 16.2% | 21.2% (14.7%–27.9%) | 91.7% / 91.7% | 41.7% / 41.7% |
| bayrou | 6.8% | 3.3% (2.2%–4.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 5.7% | 7.0% (4.8%–9.4%) | 0.4% / 0.4% | 0.1% / 0.1% |
| chevenement | 5.3% | 11.0% (6.7%–15.6%) | 8.3% / 8.3% | 0.5% / 0.5% |
| mamere | 5.2% | 5.9% (3.4%–8.8%) | 0.6% / 0.6% | 0.3% / 0.3% |
| besancenot | 4.2% | 0.7% (0.3%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | 4.8% (2.7%–7.2%) | 0.1% / 0.1% | 0.1% / 0.1% |
| madelin | 3.9% | 4.0% (2.0%–6.2%) | 0.3% / 0.3% | 0.1% / 0.1% |
| hue | 3.4% | 5.7% (3.3%–8.4%) | 0.5% / 0.5% | 0.1% / 0.1% |
| megret | 2.3% | 1.3% (0.6%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 0.8% (0.4%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 0.8% (0.4%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.6% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 49 days out (2002-03-03)

Actual matchup: current 1.4% (winner given matchup 78.8%), fitted 1.4% (winner given matchup 78.8%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 22.7% (17.1%–28.6%) | 97.0% / 97.0% ✓ | 45.6% / 45.6% ✓ |
| jm_le_pen | 16.9% | 9.6% (6.9%–12.6%) | 1.9% / 1.9% ✓ | 0.3% / 0.3% |
| jospin | 16.2% | 21.2% (15.0%–27.6%) | 93.4% / 93.4% | 52.9% / 52.9% |
| bayrou | 6.8% | 3.4% (2.4%–4.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 5.7% | 8.0% (5.6%–10.6%) | 0.8% / 0.8% | 0.3% / 0.3% |
| chevenement | 5.3% | 10.0% (6.2%–14.2%) | 5.4% / 5.4% | 0.4% / 0.4% |
| mamere | 5.2% | 6.0% (3.5%–8.7%) | 0.5% / 0.5% | 0.2% / 0.2% |
| besancenot | 4.2% | 0.9% (0.4%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | 4.9% (2.8%–7.3%) | 0.1% / 0.1% | 0.1% / 0.1% |
| madelin | 3.9% | 3.7% (1.9%–5.7%) | 0.3% / 0.3% | 0.1% / 0.1% |
| hue | 3.4% | 5.5% (3.2%–8.1%) | 0.4% / 0.4% | 0.1% / 0.1% |
| megret | 2.3% | 1.3% (0.7%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 0.8% (0.4%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 42 days out (2002-03-10)

Actual matchup: current 1.4% (winner given matchup 77.5%), fitted 1.4% (winner given matchup 77.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 22.7% (17.3%–28.4%) | 97.5% / 97.5% ✓ | 40.3% / 40.3% ✓ |
| jm_le_pen | 16.9% | 9.7% (7.0%–12.6%) | 1.9% / 1.9% ✓ | 0.4% / 0.4% |
| jospin | 16.2% | 21.1% (15.2%–27.2%) | 94.3% / 94.3% | 58.4% / 58.4% |
| bayrou | 6.8% | 3.5% (2.4%–4.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 5.7% | 8.5% (6.0%–11.1%) | 1.0% / 1.0% | 0.3% / 0.3% |
| chevenement | 5.3% | 9.7% (6.1%–13.6%) | 4.3% / 4.3% | 0.3% / 0.3% |
| mamere | 5.2% | 5.6% (3.4%–8.0%) | 0.3% / 0.3% | 0.1% / 0.1% |
| besancenot | 4.2% | 0.9% (0.4%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | 5.1% (2.9%–7.6%) | 0.1% / 0.1% | 0.1% / 0.1% |
| madelin | 3.9% | 3.6% (1.9%–5.4%) | 0.2% / 0.2% | 0.0% / 0.0% |
| hue | 3.4% | 5.5% (3.3%–7.9%) | 0.4% / 0.4% | 0.1% / 0.1% |
| megret | 2.3% | 1.5% (0.8%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 0.8% (0.4%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 0.9% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.6% (0.3%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 35 days out (2002-03-17)

Actual matchup: current 1.4% (winner given matchup 80.0%), fitted 1.4% (winner given matchup 80.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 23.4% (18.1%–28.9%) | 98.4% / 98.4% ✓ | 39.9% / 39.9% ✓ |
| jm_le_pen | 16.9% | 9.6% (7.1%–12.3%) | 1.7% / 1.7% ✓ | 0.3% / 0.3% |
| jospin | 16.2% | 20.3% (14.8%–25.9%) | 94.6% / 94.6% | 58.9% / 58.9% |
| bayrou | 6.8% | 3.8% (2.7%–4.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 5.7% | 8.8% (6.4%–11.4%) | 1.1% / 1.1% | 0.3% / 0.3% |
| chevenement | 5.3% | 9.1% (5.9%–12.5%) | 3.1% / 3.1% | 0.2% / 0.2% |
| mamere | 5.2% | 5.5% (3.4%–7.8%) | 0.3% / 0.3% | 0.1% / 0.1% |
| besancenot | 4.2% | 0.9% (0.5%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | 5.5% (3.2%–8.1%) | 0.3% / 0.3% | 0.1% / 0.1% |
| madelin | 3.9% | 3.5% (2.0%–5.2%) | 0.2% / 0.2% | 0.0% / 0.0% |
| hue | 3.4% | 5.3% (3.3%–7.5%) | 0.3% / 0.3% | 0.1% / 0.1% |
| megret | 2.3% | 1.4% (0.8%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 0.9% (0.5%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 0.9% (0.5%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 28 days out (2002-03-24)

Actual matchup: current 1.4% (winner given matchup 77.6%), fitted 1.4% (winner given matchup 77.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 23.1% (18.0%–28.3%) | 98.4% / 98.4% ✓ | 40.9% / 40.9% ✓ |
| jm_le_pen | 16.9% | 10.1% (7.6%–12.9%) | 1.9% / 1.9% ✓ | 0.4% / 0.4% |
| jospin | 16.2% | 20.9% (15.7%–26.3%) | 95.9% / 95.9% | 58.0% / 58.0% |
| bayrou | 6.8% | 4.0% (2.9%–5.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 5.7% | 9.1% (6.7%–11.6%) | 1.1% / 1.1% | 0.3% / 0.3% |
| chevenement | 5.3% | 8.6% (5.6%–11.7%) | 2.0% / 2.0% | 0.1% / 0.1% |
| mamere | 5.2% | 5.4% (3.5%–7.6%) | 0.2% / 0.2% | 0.1% / 0.1% |
| besancenot | 4.2% | 0.9% (0.5%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | 4.9% (2.9%–7.3%) | 0.1% / 0.1% | 0.1% / 0.1% |
| madelin | 3.9% | 3.4% (2.0%–5.0%) | 0.1% / 0.1% | 0.0% / 0.0% |
| hue | 3.4% | 5.4% (3.4%–7.5%) | 0.2% / 0.2% | 0.1% / 0.1% |
| megret | 2.3% | 1.3% (0.7%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.6% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 0.9% (0.5%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 0.9% (0.5%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 21 days out (2002-03-31)

Actual matchup: current 2.4% (winner given matchup 77.3%), fitted 2.4% (winner given matchup 77.3%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 21.6% (17.0%–26.3%) | 97.9% / 97.9% ✓ | 40.8% / 40.8% ✓ |
| jm_le_pen | 16.9% | 10.7% (8.2%–13.4%) | 3.2% / 3.2% ✓ | 0.6% / 0.6% |
| jospin | 16.2% | 19.7% (14.9%–24.6%) | 94.5% / 94.5% | 57.6% / 57.6% |
| bayrou | 6.8% | 4.5% (3.4%–5.8%) | 0.1% / 0.1% | 0.0% / 0.0% |
| laguiller | 5.7% | 9.8% (7.5%–12.3%) | 1.8% / 1.8% | 0.6% / 0.6% |
| chevenement | 5.3% | 8.4% (5.7%–11.3%) | 1.8% / 1.8% | 0.1% / 0.1% |
| mamere | 5.2% | 5.6% (3.7%–7.7%) | 0.2% / 0.2% | 0.1% / 0.1% |
| besancenot | 4.2% | 0.9% (0.5%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | 5.5% (3.3%–8.0%) | 0.2% / 0.2% | 0.1% / 0.1% |
| madelin | 3.9% | 3.4% (2.1%–4.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| hue | 3.4% | 5.3% (3.5%–7.3%) | 0.2% / 0.2% | 0.0% / 0.0% |
| megret | 2.3% | 1.3% (0.8%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.6% (0.4%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 0.9% (0.5%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 14 days out (2002-04-07)

Actual matchup: current 5.1% (winner given matchup 78.6%), fitted 5.1% (winner given matchup 78.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 21.3% (17.0%–25.8%) | 97.6% / 97.6% ✓ | 46.4% / 46.4% ✓ |
| jm_le_pen | 16.9% | 11.9% (9.3%–14.7%) | 6.5% / 6.5% ✓ | 1.2% / 1.2% |
| jospin | 16.2% | 19.0% (14.7%–23.5%) | 92.6% / 92.6% | 51.5% / 51.5% |
| bayrou | 6.8% | 5.0% (3.8%–6.3%) | 0.1% / 0.1% | 0.1% / 0.1% |
| laguiller | 5.7% | 9.7% (7.5%–12.0%) | 1.4% / 1.4% | 0.5% / 0.5% |
| chevenement | 5.3% | 7.8% (5.5%–10.3%) | 1.2% / 1.2% | 0.1% / 0.1% |
| mamere | 5.2% | 6.5% (4.5%–8.6%) | 0.4% / 0.4% | 0.2% / 0.2% |
| besancenot | 4.2% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | 4.1% (3.1%–5.2%) | 0.1% / 0.1% | 0.0% / 0.0% |
| madelin | 3.9% | 3.5% (2.2%–4.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| hue | 3.4% | 5.1% (3.5%–6.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| megret | 2.3% | 1.6% (1.0%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.7% (0.4%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 1.1% (0.7%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 1.0% (0.6%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2002, 7 days out (2002-04-14)

Actual matchup: current 7.1% (winner given matchup 78.7%), fitted 7.1% (winner given matchup 78.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| chirac | 19.9% | 20.7% (16.7%–24.8%) | 97.5% / 97.5% ✓ | 53.1% / 53.1% ✓ |
| jm_le_pen | 16.9% | 12.2% (9.6%–14.9%) | 9.0% / 9.0% ✓ | 1.6% / 1.6% |
| jospin | 16.2% | 18.1% (14.3%–22.2%) | 91.3% / 91.3% | 44.6% / 44.6% |
| bayrou | 6.8% | 5.5% (4.2%–6.9%) | 0.1% / 0.1% | 0.1% / 0.1% |
| laguiller | 5.7% | 8.9% (6.9%–11.0%) | 0.8% / 0.8% | 0.3% / 0.3% |
| chevenement | 5.3% | 7.5% (5.4%–9.7%) | 0.8% / 0.8% | 0.0% / 0.0% |
| mamere | 5.2% | 6.2% (4.5%–8.1%) | 0.2% / 0.2% | 0.1% / 0.1% |
| besancenot | 4.2% | 1.7% (1.1%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| saint_josse | 4.2% | 3.9% (3.0%–4.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| madelin | 3.9% | 3.9% (2.6%–5.3%) | 0.1% / 0.1% | 0.0% / 0.0% |
| hue | 3.4% | 5.4% (3.8%–7.0%) | 0.1% / 0.1% | 0.0% / 0.0% |
| megret | 2.3% | 2.1% (1.4%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| taubira | 2.3% | 0.9% (0.6%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lepage | 1.9% | 1.4% (0.9%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| boutin | 1.2% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| gluckstein | 0.5% | 0.6% (0.4%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 334 days out (2006-05-23)

Actual matchup: current 64.0% (winner given matchup 37.3%), fitted 64.0% (winner given matchup 37.3%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 29.1% (13.8%–46.6%) | 85.1% / 85.1% ✓ | 38.6% / 38.6% ✓ |
| royal | 25.9% | 25.1% (11.0%–41.4%) | 77.4% / 77.4% ✓ | 50.9% / 50.9% |
| bayrou | 18.6% | 7.3% (2.7%–13.3%) | 5.1% / 5.1% | 1.7% / 1.7% |
| jm_le_pen | 10.4% | 10.7% (4.2%–19.1%) | 15.1% / 15.1% | 2.9% / 2.9% |
| besancenot | 4.1% | 5.8% (1.3%–12.2%) | 4.5% / 4.5% | 1.5% / 1.5% |
| de_villiers | 2.2% | 4.6% (1.7%–8.4%) | 0.9% / 0.9% | 0.2% / 0.2% |
| buffet | 1.9% | 4.2% (0.6%–9.6%) | 3.2% / 3.2% | 1.2% / 1.2% |
| voynet | 1.6% | 3.0% (0.4%–6.7%) | 1.6% / 1.6% | 0.6% / 0.6% |
| laguiller | 1.3% | 4.2% (0.8%–9.2%) | 2.2% / 2.2% | 0.8% / 0.8% |
| bove | 1.3% | 6.0% (1.3%–12.5%) | 5.0% / 5.0% | 1.8% / 1.8% |
| nihous | 1.1% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 304 days out (2006-06-22)

Actual matchup: current 97.2% (winner given matchup 43.2%), fitted 97.2% (winner given matchup 43.2%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 29.5% (24.7%–34.5%) | 99.7% / 99.7% ✓ | 43.7% / 43.7% ✓ |
| royal | 25.9% | 26.7% (20.4%–32.7%) | 97.5% / 97.5% ✓ | 55.4% / 55.4% |
| bayrou | 18.6% | 6.8% (5.3%–8.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| jm_le_pen | 10.4% | 11.2% (8.9%–13.5%) | 0.8% / 0.8% | 0.1% / 0.1% |
| besancenot | 4.1% | 6.0% (2.8%–9.6%) | 0.1% / 0.1% | 0.0% / 0.0% |
| de_villiers | 2.2% | 4.1% (3.1%–5.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 3.5% (1.1%–6.5%) | 1.0% / 1.0% | 0.4% / 0.4% |
| voynet | 1.6% | 2.6% (0.8%–4.8%) | 0.7% / 0.7% | 0.3% / 0.3% |
| laguiller | 1.3% | 3.8% (1.5%–6.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 5.8% (2.6%–9.4%) | 0.1% / 0.1% | 0.0% / 0.0% |
| nihous | 1.1% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 273 days out (2006-07-23)

Actual matchup: current 97.6% (winner given matchup 46.5%), fitted 97.6% (winner given matchup 46.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 29.6% (24.8%–34.5%) | 99.7% / 99.7% ✓ | 46.8% / 46.8% ✓ |
| royal | 25.9% | 26.8% (20.7%–32.5%) | 97.9% / 97.9% ✓ | 52.5% / 52.5% |
| bayrou | 18.6% | 6.7% (5.3%–8.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| jm_le_pen | 10.4% | 11.1% (8.9%–13.4%) | 0.7% / 0.7% | 0.1% / 0.1% |
| besancenot | 4.1% | 6.0% (2.9%–9.5%) | 0.1% / 0.1% | 0.0% / 0.0% |
| de_villiers | 2.2% | 3.8% (3.0%–4.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 3.3% (1.1%–6.1%) | 0.8% / 0.8% | 0.3% / 0.3% |
| voynet | 1.6% | 2.7% (0.9%–4.8%) | 0.6% / 0.6% | 0.2% / 0.2% |
| laguiller | 1.3% | 3.9% (1.6%–6.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 5.9% (2.8%–9.4%) | 0.1% / 0.1% | 0.1% / 0.1% |
| nihous | 1.1% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 243 days out (2006-08-22)

Actual matchup: current 97.9% (winner given matchup 53.1%), fitted 97.9% (winner given matchup 53.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 31.1% (25.9%–36.5%) | 99.7% / 99.7% ✓ | 53.3% / 53.3% ✓ |
| royal | 25.9% | 27.5% (21.3%–33.4%) | 98.2% / 98.2% ✓ | 46.2% / 46.2% |
| bayrou | 18.6% | 6.9% (5.4%–8.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| jm_le_pen | 10.4% | 11.4% (9.0%–13.9%) | 0.8% / 0.8% | 0.1% / 0.1% |
| besancenot | 4.1% | 5.6% (2.9%–8.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 3.9% (2.9%–4.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 3.2% (1.1%–5.7%) | 0.6% / 0.6% | 0.2% / 0.2% |
| voynet | 1.6% | 2.6% (0.9%–4.6%) | 0.5% / 0.5% | 0.2% / 0.2% |
| laguiller | 1.3% | 3.7% (1.6%–6.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 4.1% (1.8%–6.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 212 days out (2006-09-22)

Actual matchup: current 97.3% (winner given matchup 49.7%), fitted 97.3% (winner given matchup 49.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 31.3% (24.0%–39.1%) | 99.5% / 99.5% ✓ | 50.1% / 50.1% ✓ |
| royal | 25.9% | 27.7% (20.2%–35.3%) | 97.8% / 97.8% ✓ | 49.4% / 49.4% |
| bayrou | 18.6% | 7.1% (4.9%–9.5%) | 0.1% / 0.1% | 0.0% / 0.0% |
| jm_le_pen | 10.4% | 11.3% (8.0%–15.0%) | 1.5% / 1.5% | 0.2% / 0.2% |
| besancenot | 4.1% | 5.0% (2.5%–7.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 4.0% (2.5%–5.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 3.0% (1.0%–5.5%) | 0.5% / 0.5% | 0.2% / 0.2% |
| voynet | 1.6% | 2.7% (0.9%–4.8%) | 0.5% / 0.5% | 0.2% / 0.2% |
| laguiller | 1.3% | 3.8% (1.7%–6.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 3.1% (1.3%–5.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 0.9% (0.3%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 182 days out (2006-10-22)

Actual matchup: current 98.4% (winner given matchup 47.4%), fitted 98.4% (winner given matchup 47.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 31.2% (26.1%–36.3%) | 99.7% / 99.7% ✓ | 47.6% / 47.6% ✓ |
| royal | 25.9% | 27.7% (22.0%–33.1%) | 98.6% / 98.6% ✓ | 51.9% / 51.9% |
| bayrou | 18.6% | 7.7% (6.0%–9.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| jm_le_pen | 10.4% | 11.5% (9.1%–13.9%) | 0.8% / 0.8% | 0.1% / 0.1% |
| besancenot | 4.1% | 4.9% (2.8%–7.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 3.8% (2.6%–5.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 3.0% (1.2%–5.1%) | 0.4% / 0.4% | 0.1% / 0.1% |
| voynet | 1.6% | 2.5% (1.0%–4.2%) | 0.4% / 0.4% | 0.1% / 0.1% |
| laguiller | 1.3% | 3.4% (1.7%–5.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 3.2% (1.6%–5.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 1.3% (0.5%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 152 days out (2006-11-21)

Actual matchup: current 98.4% (winner given matchup 47.8%), fitted 98.4% (winner given matchup 47.8%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 31.0% (25.5%–36.7%) | 99.7% / 99.7% ✓ | 48.0% / 48.0% ✓ |
| royal | 25.9% | 28.0% (22.1%–33.9%) | 98.7% / 98.7% ✓ | 51.6% / 51.6% |
| bayrou | 18.6% | 7.8% (6.0%–9.8%) | 0.1% / 0.1% | 0.0% / 0.0% |
| jm_le_pen | 10.4% | 11.8% (9.2%–14.6%) | 0.9% / 0.9% | 0.1% / 0.1% |
| besancenot | 4.1% | 4.7% (2.7%–6.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 3.4% (2.3%–4.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 3.0% (1.3%–5.1%) | 0.3% / 0.3% | 0.1% / 0.1% |
| voynet | 1.6% | 2.5% (1.0%–4.1%) | 0.3% / 0.3% | 0.1% / 0.1% |
| laguiller | 1.3% | 3.4% (1.8%–5.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 3.1% (1.6%–4.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 1.1% (0.5%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 122 days out (2006-12-21)

Actual matchup: current 98.8% (winner given matchup 46.7%), fitted 98.8% (winner given matchup 46.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 30.7% (25.3%–36.2%) | 99.7% / 99.7% ✓ | 46.8% / 46.8% ✓ |
| royal | 25.9% | 29.9% (24.1%–35.6%) | 99.1% / 99.1% ✓ | 53.0% / 53.0% |
| bayrou | 18.6% | 7.6% (5.8%–9.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| jm_le_pen | 10.4% | 11.8% (9.2%–14.4%) | 0.7% / 0.7% | 0.0% / 0.0% |
| besancenot | 4.1% | 4.4% (2.6%–6.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 3.1% (2.1%–4.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 3.0% (1.4%–4.9%) | 0.2% / 0.2% | 0.1% / 0.1% |
| voynet | 1.6% | 2.3% (1.0%–3.7%) | 0.2% / 0.2% | 0.1% / 0.1% |
| laguiller | 1.3% | 3.3% (1.8%–4.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 3.0% (1.6%–4.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 1.0% (0.5%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 91 days out (2007-01-21)

Actual matchup: current 98.5% (winner given matchup 57.5%), fitted 98.5% (winner given matchup 57.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 30.4% (24.5%–36.6%) | 99.6% / 99.6% ✓ | 57.6% / 57.6% ✓ |
| royal | 25.9% | 28.8% (22.8%–34.9%) | 98.9% / 98.9% ✓ | 42.3% / 42.3% |
| bayrou | 18.6% | 9.1% (6.8%–11.6%) | 0.2% / 0.2% | 0.0% / 0.0% |
| jm_le_pen | 10.4% | 12.3% (9.3%–15.4%) | 1.0% / 1.0% | 0.0% / 0.0% |
| besancenot | 4.1% | 4.1% (2.4%–5.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 2.5% (1.7%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 3.2% (1.5%–5.2%) | 0.2% / 0.2% | 0.1% / 0.1% |
| voynet | 1.6% | 2.2% (1.1%–3.5%) | 0.1% / 0.1% | 0.0% / 0.0% |
| laguiller | 1.3% | 3.2% (1.8%–4.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 2.7% (1.4%–4.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.6% (0.3%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 84 days out (2007-01-28)

Actual matchup: current 98.6% (winner given matchup 69.4%), fitted 98.6% (winner given matchup 69.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 31.8% (25.8%–37.9%) | 99.7% / 99.7% ✓ | 69.4% / 69.4% ✓ |
| royal | 25.9% | 28.1% (22.2%–34.0%) | 98.9% / 98.9% ✓ | 30.5% / 30.5% |
| bayrou | 18.6% | 9.8% (7.4%–12.5%) | 0.2% / 0.2% | 0.0% / 0.0% |
| jm_le_pen | 10.4% | 12.0% (9.1%–15.0%) | 0.8% / 0.8% | 0.0% / 0.0% |
| besancenot | 4.1% | 3.6% (2.2%–5.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 2.5% (1.7%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 3.5% (1.7%–5.5%) | 0.2% / 0.2% | 0.0% / 0.0% |
| voynet | 1.6% | 2.2% (1.1%–3.4%) | 0.1% / 0.1% | 0.0% / 0.0% |
| laguiller | 1.3% | 2.8% (1.6%–4.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 2.3% (1.2%–3.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 0.9% (0.4%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 77 days out (2007-02-04)

Actual matchup: current 98.4% (winner given matchup 76.1%), fitted 98.4% (winner given matchup 76.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 32.5% (26.6%–38.7%) | 99.7% / 99.7% ✓ | 76.1% / 76.1% ✓ |
| royal | 25.9% | 27.2% (21.6%–32.9%) | 98.7% / 98.7% ✓ | 23.8% / 23.8% |
| bayrou | 18.6% | 10.4% (7.8%–13.1%) | 0.3% / 0.3% | 0.0% / 0.0% |
| jm_le_pen | 10.4% | 12.2% (9.3%–15.2%) | 1.0% / 1.0% | 0.0% / 0.0% |
| besancenot | 4.1% | 3.2% (2.0%–4.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 2.5% (1.7%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 3.3% (1.6%–5.1%) | 0.2% / 0.2% | 0.0% / 0.0% |
| voynet | 1.6% | 2.2% (1.1%–3.4%) | 0.1% / 0.1% | 0.0% / 0.0% |
| laguiller | 1.3% | 2.6% (1.5%–3.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 2.4% (1.4%–3.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 1.0% (0.5%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 70 days out (2007-02-11)

Actual matchup: current 98.1% (winner given matchup 81.1%), fitted 98.1% (winner given matchup 81.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 33.0% (27.1%–39.0%) | 99.7% / 99.7% ✓ | 81.0% / 81.0% ✓ |
| royal | 25.9% | 26.5% (21.0%–31.9%) | 98.4% / 98.4% ✓ | 18.9% / 18.9% |
| bayrou | 18.6% | 10.8% (8.2%–13.7%) | 0.4% / 0.4% | 0.1% / 0.1% |
| jm_le_pen | 10.4% | 12.5% (9.6%–15.5%) | 1.2% / 1.2% | 0.0% / 0.0% |
| besancenot | 4.1% | 3.1% (1.9%–4.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 2.4% (1.7%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 3.1% (1.6%–4.8%) | 0.1% / 0.1% | 0.0% / 0.0% |
| voynet | 1.6% | 2.0% (1.0%–3.1%) | 0.1% / 0.1% | 0.0% / 0.0% |
| laguiller | 1.3% | 2.6% (1.5%–3.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 2.5% (1.4%–3.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 1.0% (0.5%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 63 days out (2007-02-18)

Actual matchup: current 98.0% (winner given matchup 87.4%), fitted 98.0% (winner given matchup 87.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 33.5% (27.7%–39.4%) | 99.7% / 99.7% ✓ | 87.1% / 87.1% ✓ |
| royal | 25.9% | 25.9% (20.7%–31.1%) | 98.3% / 98.3% ✓ | 12.7% / 12.7% |
| bayrou | 18.6% | 11.5% (8.7%–14.4%) | 0.6% / 0.6% | 0.2% / 0.2% |
| jm_le_pen | 10.4% | 12.5% (9.7%–15.5%) | 1.1% / 1.1% | 0.0% / 0.0% |
| besancenot | 4.1% | 3.3% (2.1%–4.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 2.5% (1.7%–3.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 2.8% (1.5%–4.3%) | 0.1% / 0.1% | 0.0% / 0.0% |
| voynet | 1.6% | 1.7% (0.9%–2.7%) | 0.1% / 0.1% | 0.0% / 0.0% |
| laguiller | 1.3% | 2.3% (1.4%–3.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 2.5% (1.5%–3.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 1.0% (0.5%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 56 days out (2007-02-25)

Actual matchup: current 95.6% (winner given matchup 79.5%), fitted 95.6% (winner given matchup 79.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 31.0% (24.4%–38.0%) | 99.0% / 99.0% ✓ | 78.7% / 78.7% ✓ |
| royal | 25.9% | 26.7% (20.6%–33.1%) | 96.5% / 96.5% ✓ | 20.3% / 20.3% |
| bayrou | 18.6% | 13.6% (9.9%–17.7%) | 2.6% / 2.6% | 1.0% / 1.0% |
| jm_le_pen | 10.4% | 12.7% (9.3%–16.4%) | 1.7% / 1.7% | 0.0% / 0.0% |
| besancenot | 4.1% | 3.2% (1.9%–4.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 2.4% (1.6%–3.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 2.8% (1.4%–4.5%) | 0.1% / 0.1% | 0.0% / 0.0% |
| voynet | 1.6% | 1.6% (0.8%–2.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 1.3% | 2.1% (1.2%–3.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 2.2% (1.3%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 1.0% (0.5%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 49 days out (2007-03-04)

Actual matchup: current 92.6% (winner given matchup 83.0%), fitted 92.6% (winner given matchup 83.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 30.6% (24.2%–37.2%) | 98.2% / 98.2% ✓ | 80.6% / 80.6% ✓ |
| royal | 25.9% | 26.3% (20.5%–32.5%) | 94.4% / 94.4% ✓ | 16.6% / 16.6% |
| bayrou | 18.6% | 15.4% (11.4%–19.8%) | 5.6% / 5.6% | 2.8% / 2.8% |
| jm_le_pen | 10.4% | 13.2% (9.8%–16.8%) | 1.7% / 1.7% | 0.0% / 0.0% |
| besancenot | 4.1% | 3.0% (1.8%–4.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 2.0% (1.3%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 2.8% (1.5%–4.4%) | 0.1% / 0.1% | 0.0% / 0.0% |
| voynet | 1.6% | 1.4% (0.7%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 1.3% | 1.9% (1.1%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 2.0% (1.2%–3.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 0.9% (0.5%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 42 days out (2007-03-11)

Actual matchup: current 82.8% (winner given matchup 83.5%), fitted 82.8% (winner given matchup 83.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 29.5% (23.5%–35.9%) | 96.0% / 96.0% ✓ | 73.5% / 73.5% ✓ |
| royal | 25.9% | 25.3% (19.8%–31.0%) | 86.8% / 86.8% ✓ | 15.5% / 15.5% |
| bayrou | 18.6% | 18.1% (13.7%–22.9%) | 15.6% / 15.6% | 11.0% / 11.0% |
| jm_le_pen | 10.4% | 13.3% (10.0%–16.8%) | 1.5% / 1.5% | 0.0% / 0.0% |
| besancenot | 4.1% | 2.9% (1.8%–4.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 1.9% (1.3%–2.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 2.5% (1.3%–3.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| voynet | 1.6% | 1.3% (0.7%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 1.3% | 1.9% (1.1%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 1.9% (1.1%–2.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 0.8% (0.5%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 35 days out (2007-03-18)

Actual matchup: current 64.4% (winner given matchup 81.3%), fitted 64.4% (winner given matchup 81.3%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 28.2% (22.5%–34.1%) | 91.0% / 91.0% ✓ | 56.6% / 56.6% ✓ |
| royal | 25.9% | 24.5% (19.3%–29.8%) | 73.3% / 73.3% ✓ | 13.6% / 13.6% |
| bayrou | 18.6% | 21.1% (16.4%–26.1%) | 34.6% / 34.6% | 29.8% / 29.8% |
| jm_le_pen | 10.4% | 13.0% (9.9%–16.3%) | 1.0% / 1.0% | 0.0% / 0.0% |
| besancenot | 4.1% | 2.9% (1.8%–4.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 1.7% (1.1%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 2.4% (1.3%–3.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| voynet | 1.6% | 1.2% (0.6%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 1.3% | 2.0% (1.2%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 1.9% (1.1%–2.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 0.8% (0.4%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 28 days out (2007-03-25)

Actual matchup: current 74.4% (winner given matchup 79.2%), fitted 74.4% (winner given matchup 79.2%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 27.9% (22.5%–33.5%) | 92.2% / 92.2% ✓ | 61.2% / 61.2% ✓ |
| royal | 25.9% | 25.3% (20.2%–30.5%) | 82.2% / 82.2% ✓ | 17.1% / 17.1% |
| bayrou | 18.6% | 20.1% (15.8%–24.8%) | 24.6% / 24.6% | 21.7% / 21.7% |
| jm_le_pen | 10.4% | 13.1% (10.0%–16.2%) | 1.0% / 1.0% | 0.0% / 0.0% |
| besancenot | 4.1% | 3.2% (2.1%–4.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 1.6% (1.1%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 2.5% (1.4%–3.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| voynet | 1.6% | 1.1% (0.6%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 1.3% | 2.0% (1.3%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 1.9% (1.2%–2.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 0.8% (0.5%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 21 days out (2007-04-01)

Actual matchup: current 81.0% (winner given matchup 79.0%), fitted 81.0% (winner given matchup 79.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 28.6% (23.4%–34.1%) | 95.4% / 95.4% ✓ | 66.3% / 66.3% ✓ |
| royal | 25.9% | 25.0% (20.2%–29.9%) | 85.5% / 85.5% ✓ | 17.7% / 17.7% |
| bayrou | 18.6% | 19.2% (15.2%–23.4%) | 18.1% / 18.1% | 16.0% / 16.0% |
| jm_le_pen | 10.4% | 13.0% (10.1%–16.1%) | 0.9% / 0.9% | 0.0% / 0.0% |
| besancenot | 4.1% | 3.7% (2.5%–4.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 1.5% (1.0%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 2.6% (1.5%–3.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| voynet | 1.6% | 1.1% (0.6%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 1.3% | 2.0% (1.3%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 1.9% (1.2%–2.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 0.9% (0.6%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 14 days out (2007-04-08)

Actual matchup: current 77.5% (winner given matchup 83.8%), fitted 77.5% (winner given matchup 83.8%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 28.7% (23.6%–33.8%) | 96.4% / 96.4% ✓ | 68.8% / 68.8% ✓ |
| royal | 25.9% | 23.6% (19.2%–28.1%) | 81.0% / 81.0% ✓ | 13.2% / 13.2% |
| bayrou | 18.6% | 19.1% (15.4%–23.1%) | 21.1% / 21.1% | 18.0% / 18.0% |
| jm_le_pen | 10.4% | 13.7% (10.8%–16.7%) | 1.4% / 1.4% | 0.0% / 0.0% |
| besancenot | 4.1% | 4.1% (2.9%–5.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 1.5% (1.0%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 2.6% (1.6%–3.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| voynet | 1.6% | 1.2% (0.7%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 1.3% | 2.1% (1.4%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 1.9% (1.2%–2.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 1.0% (0.7%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.6% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2007, 7 days out (2007-04-15)

Actual matchup: current 85.3% (winner given matchup 84.2%), fitted 85.3% (winner given matchup 84.2%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| sarkozy | 31.2% | 28.2% (23.4%–33.0%) | 96.6% / 96.6% ✓ | 74.1% / 74.1% ✓ |
| royal | 25.9% | 24.5% (20.2%–28.8%) | 88.6% / 88.6% ✓ | 14.1% / 14.1% |
| bayrou | 18.6% | 18.5% (15.0%–22.2%) | 13.5% / 13.5% | 11.7% / 11.7% |
| jm_le_pen | 10.4% | 13.6% (10.9%–16.4%) | 1.1% / 1.1% | 0.0% / 0.0% |
| besancenot | 4.1% | 3.9% (2.9%–5.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| de_villiers | 2.2% | 1.5% (1.1%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| buffet | 1.9% | 2.7% (1.8%–3.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| voynet | 1.6% | 1.2% (0.8%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| laguiller | 1.3% | 2.0% (1.4%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| bove | 1.3% | 1.9% (1.3%–2.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| nihous | 1.1% | 1.3% (0.9%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| schivardi | 0.3% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 334 days out (2011-05-24)

Actual matchup: current 47.3% (winner given matchup 92.7%), fitted 47.3% (winner given matchup 92.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 25.5% (16.1%–34.4%) | 74.8% / 74.8% ✓ | 70.8% / 70.8% ✓ |
| sarkozy | 27.2% | 23.1% (18.8%–27.5%) | 70.6% / 70.6% ✓ | 19.1% / 19.1% |
| le_pen | 17.9% | 21.5% (17.5%–25.6%) | 48.3% / 48.3% | 5.6% / 5.6% |
| melenchon | 11.1% | 6.0% (2.1%–10.7%) | 1.4% / 1.4% | 1.0% / 1.0% |
| bayrou | 9.1% | 10.6% (8.3%–13.0%) | 0.3% / 0.3% | 0.2% / 0.2% |
| joly | 2.3% | 9.4% (3.5%–16.3%) | 4.7% / 4.7% | 3.3% / 3.3% |
| dupont_aignan | 1.8% | 1.3% (0.9%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.3% (0.6%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.3% (0.6%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 304 days out (2011-06-23)

Actual matchup: current 47.0% (winner given matchup 93.7%), fitted 47.0% (winner given matchup 93.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 27.1% (17.4%–36.5%) | 80.6% / 80.6% ✓ | 77.1% / 77.1% ✓ |
| sarkozy | 27.2% | 22.5% (17.3%–28.0%) | 64.5% / 64.5% ✓ | 18.3% / 18.3% |
| le_pen | 17.9% | 21.2% (16.2%–26.4%) | 49.0% / 49.0% | 0.6% / 0.6% |
| melenchon | 11.1% | 6.9% (2.5%–12.2%) | 2.1% / 2.1% | 1.5% / 1.5% |
| bayrou | 9.1% | 10.7% (7.8%–13.7%) | 0.5% / 0.5% | 0.3% / 0.3% |
| joly | 2.3% | 8.0% (2.9%–14.2%) | 3.3% / 3.3% | 2.3% / 2.3% |
| dupont_aignan | 1.8% | 1.1% (0.7%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.3% (0.6%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.3% (0.6%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 273 days out (2011-07-24)

Actual matchup: current 52.1% (winner given matchup 93.4%), fitted 52.1% (winner given matchup 93.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 29.2% (18.6%–39.8%) | 86.0% / 86.0% ✓ | 81.3% / 81.3% ✓ |
| sarkozy | 27.2% | 22.1% (15.5%–29.2%) | 64.6% / 64.6% ✓ | 14.0% / 14.0% |
| le_pen | 17.9% | 19.5% (13.6%–25.9%) | 41.5% / 41.5% | 0.3% / 0.3% |
| melenchon | 11.1% | 6.8% (2.4%–12.3%) | 2.4% / 2.4% | 1.5% / 1.5% |
| bayrou | 9.1% | 11.3% (7.6%–15.5%) | 1.8% / 1.8% | 0.8% / 0.8% |
| joly | 2.3% | 7.7% (2.7%–13.9%) | 3.7% / 3.7% | 2.1% / 2.1% |
| dupont_aignan | 1.8% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.1% (0.4%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.3% (0.6%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 243 days out (2011-08-23)

Actual matchup: current 52.6% (winner given matchup 93.4%), fitted 52.6% (winner given matchup 93.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 29.3% (19.0%–39.6%) | 86.6% / 86.6% ✓ | 81.9% / 81.9% ✓ |
| sarkozy | 27.2% | 22.1% (15.5%–29.2%) | 64.6% / 64.6% ✓ | 13.7% / 13.7% |
| le_pen | 17.9% | 19.5% (13.6%–25.9%) | 41.5% / 41.5% | 0.3% / 0.3% |
| melenchon | 11.1% | 6.8% (2.5%–12.0%) | 2.2% / 2.2% | 1.3% / 1.3% |
| bayrou | 9.1% | 11.3% (7.6%–15.5%) | 1.8% / 1.8% | 0.8% / 0.8% |
| joly | 2.3% | 7.7% (2.8%–13.6%) | 3.4% / 3.4% | 1.9% / 1.9% |
| dupont_aignan | 1.8% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.1% (0.5%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.3% (0.6%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 212 days out (2011-09-23)

Actual matchup: current 64.9% (winner given matchup 94.4%), fitted 64.9% (winner given matchup 94.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 31.1% (21.3%–40.9%) | 91.3% / 91.3% ✓ | 87.0% / 87.0% ✓ |
| sarkozy | 27.2% | 23.0% (16.6%–29.9%) | 72.9% / 72.9% ✓ | 10.4% / 10.4% |
| le_pen | 17.9% | 18.7% (13.4%–24.5%) | 30.9% / 30.9% | 0.2% / 0.2% |
| melenchon | 11.1% | 6.7% (2.6%–11.6%) | 1.8% / 1.8% | 0.9% / 0.9% |
| bayrou | 9.1% | 10.3% (7.0%–13.8%) | 0.7% / 0.7% | 0.3% / 0.3% |
| joly | 2.3% | 7.2% (2.8%–12.4%) | 2.4% / 2.4% | 1.2% / 1.2% |
| dupont_aignan | 1.8% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.9% (0.4%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.2% (0.6%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 182 days out (2011-10-23)

Actual matchup: current 62.3% (winner given matchup 98.1%), fitted 62.3% (winner given matchup 98.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 35.8% (24.1%–47.7%) | 94.5% / 94.5% ✓ | 92.2% / 92.2% ✓ |
| sarkozy | 27.2% | 21.9% (14.0%–30.7%) | 67.3% / 67.3% ✓ | 5.0% / 5.0% |
| le_pen | 17.9% | 17.7% (11.1%–25.2%) | 32.1% / 32.1% | 0.2% / 0.2% |
| melenchon | 11.1% | 7.3% (2.6%–13.0%) | 3.2% / 3.2% | 1.4% / 1.4% |
| bayrou | 9.1% | 9.0% (5.3%–13.3%) | 1.2% / 1.2% | 0.4% / 0.4% |
| joly | 2.3% | 5.8% (2.0%–10.5%) | 1.8% / 1.8% | 0.8% / 0.8% |
| dupont_aignan | 1.8% | 0.8% (0.4%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.2%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.0% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 152 days out (2011-11-22)

Actual matchup: current 64.6% (winner given matchup 96.7%), fitted 64.6% (winner given matchup 96.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 33.9% (23.0%–45.0%) | 92.5% / 92.5% ✓ | 89.8% / 89.8% ✓ |
| sarkozy | 27.2% | 23.9% (15.9%–32.8%) | 71.7% / 71.7% ✓ | 8.2% / 8.2% |
| le_pen | 17.9% | 19.0% (12.3%–26.4%) | 32.3% / 32.3% | 0.2% / 0.2% |
| melenchon | 11.1% | 7.4% (2.9%–12.8%) | 2.2% / 2.2% | 1.1% / 1.1% |
| bayrou | 9.1% | 7.6% (4.6%–11.1%) | 0.3% / 0.3% | 0.1% / 0.1% |
| joly | 2.3% | 5.8% (2.2%–10.2%) | 1.1% / 1.1% | 0.6% / 0.6% |
| dupont_aignan | 1.8% | 0.9% (0.5%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.6% (0.3%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.4%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 122 days out (2011-12-22)

Actual matchup: current 62.8% (winner given matchup 95.2%), fitted 62.8% (winner given matchup 95.2%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 31.2% (21.5%–41.1%) | 91.0% / 91.0% ✓ | 87.1% / 87.1% ✓ |
| sarkozy | 27.2% | 23.4% (16.1%–31.5%) | 71.3% / 71.3% ✓ | 10.6% / 10.6% |
| le_pen | 17.9% | 18.9% (12.7%–25.6%) | 33.0% / 33.0% | 0.3% / 0.3% |
| melenchon | 11.1% | 7.8% (3.4%–13.0%) | 2.2% / 2.2% | 1.1% / 1.1% |
| bayrou | 9.1% | 11.1% (7.1%–15.6%) | 2.0% / 2.0% | 0.8% / 0.8% |
| joly | 2.3% | 5.1% (2.1%–8.6%) | 0.5% / 0.5% | 0.3% / 0.3% |
| dupont_aignan | 1.8% | 1.0% (0.6%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.3%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 91 days out (2012-01-22)

Actual matchup: current 50.6% (winner given matchup 95.2%), fitted 50.6% (winner given matchup 95.2%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 30.1% (21.6%–38.8%) | 90.4% / 90.4% ✓ | 86.5% / 86.5% ✓ |
| sarkozy | 27.2% | 21.9% (15.5%–28.8%) | 59.4% / 59.4% ✓ | 10.4% / 10.4% |
| le_pen | 17.9% | 20.3% (14.4%–26.7%) | 44.3% / 44.3% | 0.3% / 0.3% |
| melenchon | 11.1% | 8.3% (4.0%–13.1%) | 2.0% / 2.0% | 1.0% / 1.0% |
| bayrou | 9.1% | 12.9% (8.6%–17.6%) | 3.7% / 3.7% | 1.7% / 1.7% |
| joly | 2.3% | 4.1% (1.9%–6.7%) | 0.2% / 0.2% | 0.1% / 0.1% |
| dupont_aignan | 1.8% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 84 days out (2012-01-29)

Actual matchup: current 49.3% (winner given matchup 95.5%), fitted 49.3% (winner given matchup 95.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 30.3% (21.9%–38.9%) | 91.8% / 91.8% ✓ | 87.7% / 87.7% ✓ |
| sarkozy | 27.2% | 21.2% (15.1%–27.8%) | 56.5% / 56.5% ✓ | 8.6% / 8.6% |
| le_pen | 17.9% | 19.9% (14.2%–26.1%) | 44.2% / 44.2% | 0.2% / 0.2% |
| melenchon | 11.1% | 8.9% (4.5%–13.9%) | 2.5% / 2.5% | 1.2% / 1.2% |
| bayrou | 9.1% | 13.3% (9.0%–18.1%) | 4.7% / 4.7% | 2.1% / 2.1% |
| joly | 2.3% | 4.0% (1.9%–6.4%) | 0.2% / 0.2% | 0.1% / 0.1% |
| dupont_aignan | 1.8% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 77 days out (2012-02-05)

Actual matchup: current 72.8% (winner given matchup 96.5%), fitted 72.8% (winner given matchup 96.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 30.1% (22.0%–38.2%) | 92.9% / 92.9% ✓ | 89.1% / 89.1% ✓ |
| sarkozy | 27.2% | 24.1% (17.7%–31.0%) | 79.6% / 79.6% ✓ | 8.4% / 8.4% |
| le_pen | 17.9% | 18.0% (13.0%–23.6%) | 22.3% / 22.3% | 0.1% / 0.1% |
| melenchon | 11.1% | 8.4% (4.4%–13.1%) | 1.7% / 1.7% | 0.8% / 0.8% |
| bayrou | 9.1% | 13.3% (9.3%–17.7%) | 3.4% / 3.4% | 1.5% / 1.5% |
| joly | 2.3% | 3.3% (1.6%–5.3%) | 0.1% / 0.1% | 0.1% / 0.1% |
| dupont_aignan | 1.8% | 1.2% (0.8%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.4%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 70 days out (2012-02-12)

Actual matchup: current 61.4% (winner given matchup 96.8%), fitted 61.4% (winner given matchup 96.8%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 32.4% (24.2%–40.7%) | 95.3% / 95.3% ✓ | 92.4% / 92.4% ✓ |
| sarkozy | 27.2% | 21.9% (15.9%–28.4%) | 65.7% / 65.7% ✓ | 5.7% / 5.7% |
| le_pen | 17.9% | 19.0% (13.8%–24.7%) | 34.8% / 34.8% | 0.1% / 0.1% |
| melenchon | 11.1% | 9.0% (4.8%–13.8%) | 2.2% / 2.2% | 0.9% / 0.9% |
| bayrou | 9.1% | 11.7% (8.0%–15.8%) | 1.9% / 1.9% | 0.7% / 0.7% |
| joly | 2.3% | 3.5% (1.7%–5.4%) | 0.1% / 0.1% | 0.1% / 0.1% |
| dupont_aignan | 1.8% | 0.9% (0.6%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.4%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 63 days out (2012-02-19)

Actual matchup: current 79.8% (winner given matchup 96.0%), fitted 79.8% (winner given matchup 96.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 31.1% (23.4%–39.0%) | 95.4% / 95.4% ✓ | 91.5% / 91.5% ✓ |
| sarkozy | 27.2% | 24.1% (18.0%–30.6%) | 84.2% / 84.2% ✓ | 6.8% / 6.8% |
| le_pen | 17.9% | 17.3% (12.7%–22.4%) | 16.7% / 16.7% | 0.1% / 0.1% |
| melenchon | 11.1% | 9.0% (4.9%–13.5%) | 1.8% / 1.8% | 0.8% / 0.8% |
| bayrou | 9.1% | 12.3% (8.7%–16.2%) | 1.7% / 1.7% | 0.7% / 0.7% |
| joly | 2.3% | 3.3% (1.7%–5.1%) | 0.1% / 0.1% | 0.1% / 0.1% |
| dupont_aignan | 1.8% | 0.9% (0.4%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 56 days out (2012-02-26)

Actual matchup: current 79.0% (winner given matchup 96.1%), fitted 79.0% (winner given matchup 96.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 31.2% (23.6%–38.9%) | 95.4% / 95.4% ✓ | 91.7% / 91.7% ✓ |
| sarkozy | 27.2% | 24.3% (18.3%–30.7%) | 83.5% / 83.5% ✓ | 7.0% / 7.0% |
| le_pen | 17.9% | 17.9% (13.2%–23.0%) | 18.4% / 18.4% | 0.1% / 0.1% |
| melenchon | 11.1% | 9.2% (5.2%–13.7%) | 1.7% / 1.7% | 0.8% / 0.8% |
| bayrou | 9.1% | 11.5% (8.1%–15.2%) | 1.0% / 1.0% | 0.5% / 0.5% |
| joly | 2.3% | 3.3% (1.7%–5.0%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 1.8% | 0.8% (0.4%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.6% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.2%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 49 days out (2012-03-04)

Actual matchup: current 78.0% (winner given matchup 96.9%), fitted 78.0% (winner given matchup 96.9%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 30.6% (23.4%–37.9%) | 95.5% / 95.5% ✓ | 92.3% / 92.3% ✓ |
| sarkozy | 27.2% | 23.6% (18.0%–29.6%) | 82.3% / 82.3% ✓ | 6.0% / 6.0% |
| le_pen | 17.9% | 17.7% (13.2%–22.6%) | 18.5% / 18.5% | 0.1% / 0.1% |
| melenchon | 11.1% | 9.3% (5.4%–13.5%) | 1.6% / 1.6% | 0.7% / 0.7% |
| bayrou | 9.1% | 12.7% (9.2%–16.5%) | 2.0% / 2.0% | 0.9% / 0.9% |
| joly | 2.3% | 3.2% (1.8%–4.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 1.8% | 0.8% (0.5%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 42 days out (2012-03-11)

Actual matchup: current 81.9% (winner given matchup 96.1%), fitted 81.9% (winner given matchup 96.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 30.8% (23.7%–37.9%) | 96.2% / 96.2% ✓ | 92.2% / 92.2% ✓ |
| sarkozy | 27.2% | 23.8% (18.2%–29.7%) | 85.5% / 85.5% ✓ | 6.2% / 6.2% |
| le_pen | 17.9% | 17.2% (12.9%–21.8%) | 14.7% / 14.7% | 0.1% / 0.1% |
| melenchon | 11.1% | 9.6% (5.8%–13.9%) | 1.7% / 1.7% | 0.7% / 0.7% |
| bayrou | 9.1% | 12.6% (9.1%–16.3%) | 1.8% / 1.8% | 0.7% / 0.7% |
| joly | 2.3% | 3.2% (1.8%–4.8%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 1.8% | 0.8% (0.4%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 35 days out (2012-03-18)

Actual matchup: current 89.4% (winner given matchup 94.5%), fitted 89.4% (winner given matchup 94.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 28.4% (21.9%–35.1%) | 94.9% / 94.9% ✓ | 89.5% / 89.5% ✓ |
| sarkozy | 27.2% | 26.4% (20.7%–32.3%) | 94.5% / 94.5% ✓ | 9.1% / 9.1% |
| le_pen | 17.9% | 16.4% (12.5%–20.6%) | 7.6% / 7.6% | 0.1% / 0.1% |
| melenchon | 11.1% | 10.6% (6.7%–14.9%) | 1.8% / 1.8% | 0.8% / 0.8% |
| bayrou | 9.1% | 12.6% (9.3%–16.0%) | 1.1% / 1.1% | 0.5% / 0.5% |
| joly | 2.3% | 2.8% (1.6%–4.1%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 1.8% | 0.9% (0.5%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 28 days out (2012-03-25)

Actual matchup: current 92.2% (winner given matchup 93.0%), fitted 92.2% (winner given matchup 93.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 27.6% (21.3%–34.0%) | 95.3% / 95.3% ✓ | 88.3% / 88.3% ✓ |
| sarkozy | 27.2% | 27.6% (21.9%–33.5%) | 96.9% / 96.9% ✓ | 10.1% / 10.1% |
| le_pen | 17.9% | 15.3% (11.6%–19.1%) | 4.3% / 4.3% | 0.0% / 0.0% |
| melenchon | 11.1% | 11.9% (7.8%–16.4%) | 2.6% / 2.6% | 1.1% / 1.1% |
| bayrou | 9.1% | 12.2% (9.2%–15.4%) | 0.9% / 0.9% | 0.4% / 0.4% |
| joly | 2.3% | 2.7% (1.6%–3.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 1.8% | 0.9% (0.6%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.6% (0.4%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.4%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 21 days out (2012-04-01)

Actual matchup: current 88.0% (winner given matchup 93.0%), fitted 88.0% (winner given matchup 93.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 26.6% (20.0%–33.4%) | 92.2% / 92.2% ✓ | 85.3% / 85.3% ✓ |
| sarkozy | 27.2% | 28.0% (21.7%–34.7%) | 95.7% / 95.7% ✓ | 12.0% / 12.0% |
| le_pen | 17.9% | 15.3% (11.3%–19.6%) | 6.0% / 6.0% | 0.1% / 0.1% |
| melenchon | 11.1% | 13.7% (9.0%–18.7%) | 5.3% / 5.3% | 2.3% / 2.3% |
| bayrou | 9.1% | 11.3% (8.2%–14.5%) | 0.7% / 0.7% | 0.3% / 0.3% |
| joly | 2.3% | 2.3% (1.4%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 1.8% | 0.9% (0.6%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 14 days out (2012-04-08)

Actual matchup: current 91.3% (winner given matchup 90.8%), fitted 91.3% (winner given matchup 90.8%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 27.0% (20.8%–33.3%) | 94.3% / 94.3% ✓ | 85.4% / 85.4% ✓ |
| sarkozy | 27.2% | 28.5% (22.5%–34.8%) | 97.0% / 97.0% ✓ | 12.7% / 12.7% |
| le_pen | 17.9% | 15.0% (11.3%–18.9%) | 4.2% / 4.2% | 0.1% / 0.1% |
| melenchon | 11.1% | 13.8% (9.5%–18.5%) | 4.2% / 4.2% | 1.7% / 1.7% |
| bayrou | 9.1% | 10.3% (7.7%–13.2%) | 0.3% / 0.3% | 0.2% / 0.2% |
| joly | 2.3% | 2.4% (1.5%–3.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 1.8% | 0.9% (0.6%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.5%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.3%–0.7%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2012, 7 days out (2012-04-15)

Actual matchup: current 91.9% (winner given matchup 94.1%), fitted 91.9% (winner given matchup 94.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| hollande | 28.6% | 26.9% (21.3%–32.6%) | 95.2% / 95.2% ✓ | 89.2% / 89.2% ✓ |
| sarkozy | 27.2% | 27.5% (22.1%–33.0%) | 96.7% / 96.7% ✓ | 9.1% / 9.1% |
| le_pen | 17.9% | 15.4% (12.0%–19.1%) | 4.2% / 4.2% | 0.1% / 0.1% |
| melenchon | 11.1% | 14.0% (10.1%–18.2%) | 3.5% / 3.5% | 1.4% / 1.4% |
| bayrou | 9.1% | 10.5% (8.0%–13.1%) | 0.3% / 0.3% | 0.2% / 0.2% |
| joly | 2.3% | 2.5% (1.7%–3.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 1.8% | 1.0% (0.7%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.5%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.5%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 334 days out (2016-05-24)

Actual matchup: current 63.1% (winner given matchup 70.7%), fitted 63.1% (winner given matchup 70.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 24.8% (19.5%–30.3%) | 71.5% / 71.5% ✓ | 49.3% / 49.3% ✓ |
| le_pen | 21.3% | 28.4% (22.7%–34.1%) | 90.7% / 90.7% ✓ | 23.4% / 23.4% |
| fillon | 20.0% | 19.4% (14.6%–24.4%) | 17.5% / 17.5% | 15.3% / 15.3% |
| melenchon | 19.6% | 17.8% (10.5%–26.2%) | 20.2% / 20.2% | 12.0% / 12.0% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 5.9% (4.2%–7.6%) | 0.1% / 0.1% | 0.1% / 0.1% |
| lassalle | 1.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.5% (0.7%–2.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.5% (0.7%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.7% (0.2%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 304 days out (2016-06-23)

Actual matchup: current 11.2% (winner given matchup 54.0%), fitted 11.2% (winner given matchup 54.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 17.5% (13.6%–21.5%) | 12.1% / 12.1% ✓ | 6.5% / 6.5% ✓ |
| le_pen | 21.3% | 30.9% (25.1%–36.9%) | 96.9% / 96.9% ✓ | 20.5% / 20.5% |
| fillon | 20.0% | 22.6% (16.8%–28.7%) | 62.5% / 62.5% | 60.1% / 60.1% |
| melenchon | 19.6% | 19.2% (14.0%–24.7%) | 28.3% / 28.3% | 12.9% / 12.9% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 4.7% (2.8%–6.7%) | 0.1% / 0.1% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.8% (0.2%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.4% (0.7%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 1.0% (0.3%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.3% (0.6%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 273 days out (2016-07-24)

Actual matchup: current 18.8% (winner given matchup 53.2%), fitted 18.8% (winner given matchup 53.2%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 18.0% (13.7%–22.6%) | 19.8% / 19.8% ✓ | 10.4% / 10.4% ✓ |
| le_pen | 21.3% | 31.7% (25.9%–37.5%) | 97.8% / 97.8% ✓ | 23.7% / 23.7% |
| fillon | 20.0% | 21.7% (16.1%–27.6%) | 56.4% / 56.4% | 54.0% / 54.0% |
| melenchon | 19.6% | 18.5% (12.9%–24.5%) | 26.0% / 26.0% | 11.8% / 11.8% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 4.9% (3.0%–7.0%) | 0.1% / 0.1% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.8% (0.2%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.4% (0.7%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 1.0% (0.3%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.4% (0.7%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 243 days out (2016-08-23)

Actual matchup: current 23.6% (winner given matchup 55.4%), fitted 23.6% (winner given matchup 55.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 18.5% (13.3%–24.1%) | 25.0% / 25.0% ✓ | 13.8% / 13.8% ✓ |
| le_pen | 21.3% | 31.8% (25.7%–38.1%) | 97.1% / 97.1% ✓ | 25.4% / 25.4% |
| fillon | 20.0% | 21.3% (15.3%–27.7%) | 49.7% / 49.7% | 47.4% / 47.4% |
| melenchon | 19.6% | 18.4% (11.9%–25.8%) | 28.1% / 28.1% | 13.5% / 13.5% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 4.8% (2.9%–6.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.8% (0.2%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.4% (0.8%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 1.0% (0.3%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.3% (0.7%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 212 days out (2016-09-23)

Actual matchup: current 14.2% (winner given matchup 44.2%), fitted 14.2% (winner given matchup 44.2%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 17.0% (13.1%–21.2%) | 14.8% / 14.8% ✓ | 6.7% / 6.7% ✓ |
| le_pen | 21.3% | 33.1% (26.7%–39.8%) | 98.4% / 98.4% ✓ | 40.6% / 40.6% |
| fillon | 20.0% | 18.2% (13.3%–23.3%) | 25.8% / 25.8% | 24.5% / 24.5% |
| melenchon | 19.6% | 21.9% (15.4%–29.0%) | 61.0% / 61.0% | 28.1% / 28.1% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 4.6% (2.9%–6.4%) | 0.1% / 0.1% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.7% (0.2%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.5% (0.9%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.9% (0.3%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.4% (0.8%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.2%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 182 days out (2016-10-23)

Actual matchup: current 9.1% (winner given matchup 60.4%), fitted 9.1% (winner given matchup 60.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 17.2% (12.8%–21.8%) | 10.8% / 10.8% ✓ | 6.4% / 6.4% ✓ |
| le_pen | 21.3% | 28.8% (23.0%–34.8%) | 92.0% / 92.0% ✓ | 30.0% / 30.0% |
| fillon | 20.0% | 21.9% (15.4%–28.9%) | 47.1% / 47.1% | 44.6% / 44.6% |
| melenchon | 19.6% | 22.4% (15.6%–29.7%) | 50.0% / 50.0% | 18.9% / 18.9% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 4.7% (2.9%–6.6%) | 0.1% / 0.1% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.7% (0.2%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.5% (0.9%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.9% (0.3%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.3% (0.7%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.2%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 152 days out (2016-11-22)

Actual matchup: current 8.2% (winner given matchup 46.4%), fitted 8.2% (winner given matchup 46.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 15.9% (11.5%–20.6%) | 8.8% / 8.8% ✓ | 4.0% / 4.0% ✓ |
| le_pen | 21.3% | 32.5% (25.4%–39.8%) | 96.5% / 96.5% ✓ | 30.6% / 30.6% |
| fillon | 20.0% | 22.7% (14.7%–31.5%) | 55.0% / 55.0% | 52.9% / 52.9% |
| melenchon | 19.6% | 20.5% (14.3%–27.3%) | 39.7% / 39.7% | 12.4% / 12.4% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 3.7% (2.3%–5.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.6% (0.2%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.5% (0.9%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.8% (0.3%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.4% (0.8%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.2%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 122 days out (2016-12-22)

Actual matchup: current 8.6% (winner given matchup 98.6%), fitted 8.6% (winner given matchup 98.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 17.1% (10.9%–23.9%) | 19.0% / 19.0% ✓ | 11.8% / 11.8% ✓ |
| le_pen | 21.3% | 25.7% (17.4%–34.8%) | 76.4% / 76.4% ✓ | 5.6% / 5.6% |
| fillon | 20.0% | 27.9% (17.6%–39.1%) | 78.5% / 78.5% | 72.8% / 72.8% |
| melenchon | 19.6% | 16.9% (8.7%–26.4%) | 24.8% / 24.8% | 9.3% / 9.3% |
| hamon | 6.4% | 6.4% (2.3%–11.7%) | 1.2% / 1.2% | 0.4% / 0.4% |
| dupont_aignan | 4.7% | 2.4% (1.2%–3.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.5% (0.1%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.1% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.6% (0.2%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.0% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.4% (0.1%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 91 days out (2017-01-22)

Actual matchup: current 21.7% (winner given matchup 99.5%), fitted 21.7% (winner given matchup 99.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 20.3% (14.0%–27.2%) | 37.3% / 37.3% ✓ | 32.8% / 32.8% ✓ |
| le_pen | 21.3% | 26.4% (18.9%–34.4%) | 79.3% / 79.3% ✓ | 4.2% / 4.2% |
| fillon | 20.0% | 24.3% (16.9%–32.1%) | 68.0% / 68.0% | 56.6% / 56.6% |
| melenchon | 19.6% | 15.1% (8.8%–22.2%) | 13.9% / 13.9% | 5.8% / 5.8% |
| hamon | 6.4% | 8.5% (4.1%–13.9%) | 1.5% / 1.5% | 0.6% / 0.6% |
| dupont_aignan | 4.7% | 1.9% (1.0%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.5% (0.1%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.1% (0.6%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.5% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.0% (0.5%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.4% (0.2%–0.7%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 84 days out (2017-01-29)

Actual matchup: current 14.1% (winner given matchup 99.6%), fitted 14.1% (winner given matchup 99.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 18.4% (12.6%–24.8%) | 30.3% / 30.3% ✓ | 26.5% / 26.5% ✓ |
| le_pen | 21.3% | 23.5% (16.5%–31.0%) | 70.2% / 70.2% ✓ | 5.4% / 5.4% |
| fillon | 20.0% | 24.1% (16.0%–32.8%) | 69.1% / 69.1% | 54.8% / 54.8% |
| melenchon | 19.6% | 15.0% (8.3%–22.5%) | 17.3% / 17.3% | 7.6% / 7.6% |
| hamon | 6.4% | 13.6% (7.0%–21.2%) | 13.1% / 13.1% | 5.7% / 5.7% |
| dupont_aignan | 4.7% | 2.0% (1.1%–3.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.5% (0.1%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.0% (0.5%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.6% (0.2%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.5%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.4% (0.2%–0.7%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 77 days out (2017-02-05)

Actual matchup: current 20.3% (winner given matchup 99.6%), fitted 20.3% (winner given matchup 99.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 19.0% (12.6%–25.9%) | 34.8% / 34.8% ✓ | 30.9% / 30.9% ✓ |
| le_pen | 21.3% | 24.4% (17.0%–32.4%) | 73.0% / 73.0% ✓ | 9.8% / 9.8% |
| fillon | 20.0% | 20.8% (13.8%–28.3%) | 49.3% / 49.3% | 38.6% / 38.6% |
| melenchon | 19.6% | 13.1% (6.8%–20.3%) | 11.1% / 11.1% | 5.1% / 5.1% |
| hamon | 6.4% | 17.8% (10.2%–26.4%) | 31.7% / 31.7% | 15.6% / 15.6% |
| dupont_aignan | 4.7% | 1.9% (1.0%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.4% (0.1%–0.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.9% (0.5%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.5% (0.2%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.4%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.4% (0.2%–0.7%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 70 days out (2017-02-12)

Actual matchup: current 27.7% (winner given matchup 99.7%), fitted 27.7% (winner given matchup 99.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 19.3% (12.1%–27.2%) | 40.0% / 40.0% ✓ | 36.7% / 36.7% ✓ |
| le_pen | 21.3% | 26.6% (17.7%–36.4%) | 79.6% / 79.6% ✓ | 11.3% / 11.3% |
| fillon | 20.0% | 19.8% (11.9%–28.6%) | 43.8% / 43.8% | 35.3% / 35.3% |
| melenchon | 19.6% | 14.0% (7.0%–21.9%) | 15.6% / 15.6% | 7.2% / 7.2% |
| hamon | 6.4% | 15.4% (8.2%–23.6%) | 21.1% / 21.1% | 9.6% / 9.6% |
| dupont_aignan | 4.7% | 2.0% (1.0%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.4% (0.1%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.6% (0.2%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.3%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.4% (0.2%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 63 days out (2017-02-19)

Actual matchup: current 27.4% (winner given matchup 99.6%), fitted 27.4% (winner given matchup 99.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 19.0% (12.2%–26.4%) | 39.3% / 39.3% ✓ | 35.9% / 35.9% ✓ |
| le_pen | 21.3% | 26.0% (17.6%–35.2%) | 79.4% / 79.4% ✓ | 13.3% / 13.3% |
| fillon | 20.0% | 18.8% (11.8%–26.7%) | 38.6% / 38.6% | 31.0% / 31.0% |
| melenchon | 19.6% | 14.6% (7.7%–22.4%) | 18.0% / 18.0% | 8.5% / 8.5% |
| hamon | 6.4% | 16.1% (9.0%–24.2%) | 24.6% / 24.6% | 11.3% / 11.3% |
| dupont_aignan | 4.7% | 2.6% (1.3%–4.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.5% (0.1%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.6% (0.2%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.2%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 56 days out (2017-02-26)

Actual matchup: current 56.8% (winner given matchup 99.4%), fitted 56.8% (winner given matchup 99.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 23.8% (16.1%–32.3%) | 70.6% / 70.6% ✓ | 67.4% / 67.4% ✓ |
| le_pen | 21.3% | 27.1% (19.0%–35.7%) | 84.7% / 84.7% ✓ | 5.0% / 5.0% |
| fillon | 20.0% | 18.4% (12.1%–25.5%) | 30.8% / 30.8% | 21.7% / 21.7% |
| melenchon | 19.6% | 12.1% (6.6%–18.4%) | 6.5% / 6.5% | 2.8% / 2.8% |
| hamon | 6.4% | 12.6% (7.1%–18.7%) | 7.4% / 7.4% | 3.1% / 3.1% |
| dupont_aignan | 4.7% | 2.6% (1.3%–4.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.6% (0.1%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.9% (0.4%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.7% (0.2%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.3%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 49 days out (2017-03-05)

Actual matchup: current 76.3% (winner given matchup 99.4%), fitted 76.3% (winner given matchup 99.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 24.9% (18.7%–31.6%) | 83.3% / 83.3% ✓ | 81.4% / 81.4% ✓ |
| le_pen | 21.3% | 27.8% (21.4%–34.6%) | 92.6% / 92.6% ✓ | 3.3% / 3.3% |
| fillon | 20.0% | 16.9% (11.3%–23.0%) | 16.6% / 16.6% | 12.1% / 12.1% |
| melenchon | 19.6% | 11.2% (6.7%–16.1%) | 2.4% / 2.4% | 1.0% / 1.0% |
| hamon | 6.4% | 13.2% (8.3%–18.5%) | 5.2% / 5.2% | 2.2% / 2.2% |
| dupont_aignan | 4.7% | 2.5% (1.4%–3.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.6% (0.2%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.9% (0.5%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.7% (0.2%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.5%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.2%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 42 days out (2017-03-12)

Actual matchup: current 60.5% (winner given matchup 99.6%), fitted 60.5% (winner given matchup 99.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 25.1% (19.1%–31.6%) | 78.9% / 78.9% ✓ | 77.9% / 77.9% ✓ |
| le_pen | 21.3% | 25.4% (19.5%–31.7%) | 80.8% / 80.8% ✓ | 2.1% / 2.1% |
| fillon | 20.0% | 20.1% (13.9%–26.7%) | 33.9% / 33.9% | 17.3% / 17.3% |
| melenchon | 19.6% | 10.9% (6.6%–15.5%) | 1.9% / 1.9% | 0.8% / 0.8% |
| hamon | 6.4% | 12.9% (8.1%–18.2%) | 4.6% / 4.6% | 1.9% / 1.9% |
| dupont_aignan | 4.7% | 2.3% (1.3%–3.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.5% (0.1%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.8% (0.5%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.8% (0.3%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.5%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.4% (0.2%–0.7%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 35 days out (2017-03-19)

Actual matchup: current 71.8% (winner given matchup 99.5%), fitted 71.8% (winner given matchup 99.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 25.4% (19.6%–31.5%) | 85.0% / 85.0% ✓ | 83.8% / 83.8% ✓ |
| le_pen | 21.3% | 25.5% (19.8%–31.5%) | 86.1% / 86.1% ✓ | 2.2% / 2.2% |
| fillon | 20.0% | 18.5% (13.2%–24.0%) | 21.7% / 21.7% | 10.9% / 10.9% |
| melenchon | 19.6% | 11.5% (7.0%–16.5%) | 2.7% / 2.7% | 1.2% / 1.2% |
| hamon | 6.4% | 12.6% (7.9%–17.9%) | 4.4% / 4.4% | 1.9% / 1.9% |
| dupont_aignan | 4.7% | 3.1% (1.9%–4.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.6% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.8% (0.5%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 28 days out (2017-03-26)

Actual matchup: current 81.8% (winner given matchup 99.7%), fitted 81.8% (winner given matchup 99.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 25.5% (19.9%–31.3%) | 90.4% / 90.4% ✓ | 89.1% / 89.1% ✓ |
| le_pen | 21.3% | 25.5% (20.0%–31.2%) | 90.8% / 90.8% ✓ | 2.2% / 2.2% |
| fillon | 20.0% | 16.6% (12.2%–21.2%) | 10.5% / 10.5% | 5.2% / 5.2% |
| melenchon | 19.6% | 13.3% (8.5%–18.5%) | 5.5% / 5.5% | 2.5% / 2.5% |
| hamon | 6.4% | 11.6% (7.3%–16.4%) | 2.7% / 2.7% | 1.1% / 1.1% |
| dupont_aignan | 4.7% | 4.2% (2.8%–5.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 1.0% (0.6%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.6% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 21 days out (2017-04-02)

Actual matchup: current 76.2% (winner given matchup 99.6%), fitted 76.2% (winner given matchup 99.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 25.1% (19.8%–30.5%) | 88.5% / 88.5% ✓ | 86.4% / 86.4% ✓ |
| le_pen | 21.3% | 24.6% (19.4%–29.8%) | 86.9% / 86.9% ✓ | 2.4% / 2.4% |
| fillon | 20.0% | 17.0% (12.8%–21.4%) | 11.6% / 11.6% | 4.9% / 4.9% |
| melenchon | 19.6% | 15.4% (10.4%–20.8%) | 11.6% / 11.6% | 5.7% / 5.7% |
| hamon | 6.4% | 10.6% (6.8%–15.0%) | 1.3% / 1.3% | 0.5% / 0.5% |
| dupont_aignan | 4.7% | 3.8% (2.5%–5.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.9% (0.5%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.8% (0.5%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.5%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 14 days out (2017-04-09)

Actual matchup: current 59.0% (winner given matchup 99.7%), fitted 59.0% (winner given matchup 99.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 23.2% (18.6%–27.9%) | 78.4% / 78.4% ✓ | 74.1% / 74.1% ✓ |
| le_pen | 21.3% | 23.2% (18.7%–27.9%) | 79.2% / 79.2% ✓ | 1.0% / 1.0% |
| fillon | 20.0% | 17.3% (13.5%–21.3%) | 13.6% / 13.6% | 5.9% / 5.9% |
| melenchon | 19.6% | 18.5% (13.7%–23.6%) | 28.3% / 28.3% | 18.7% / 18.7% |
| hamon | 6.4% | 9.1% (6.1%–12.4%) | 0.4% / 0.4% | 0.2% / 0.2% |
| dupont_aignan | 4.7% | 3.9% (2.6%–5.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 1.0% (0.6%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.4% (0.9%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.1% (0.7%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 7 days out (2017-04-16)

Actual matchup: current 28.7% (winner given matchup 99.6%), fitted 28.7% (winner given matchup 99.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 21.9% (17.3%–26.7%) | 61.3% / 61.3% ✓ | 60.4% / 60.4% ✓ |
| le_pen | 21.3% | 21.6% (17.3%–26.1%) | 59.4% / 59.4% ✓ | 0.7% / 0.7% |
| fillon | 20.0% | 19.0% (14.7%–23.4%) | 29.1% / 29.1% | 9.9% / 9.9% |
| melenchon | 19.6% | 20.8% (16.0%–25.8%) | 49.9% / 49.9% | 28.9% / 28.9% |
| hamon | 6.4% | 7.8% (5.2%–10.6%) | 0.2% / 0.2% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 3.5% (2.4%–4.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 1.2% (0.7%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.7% (1.2%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.6%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 334 days out (2021-05-11)

Actual matchup: current 87.8% (winner given matchup 88.0%), fitted 87.8% (winner given matchup 88.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 27.9% (18.3%–38.4%) | 91.8% / 91.8% ✓ | 79.9% / 79.9% ✓ |
| le_pen | 23.1% | 26.6% (20.0%–33.5%) | 95.9% / 95.9% ✓ | 12.8% / 12.8% |
| melenchon | 21.9% | 10.7% (4.9%–17.2%) | 5.2% / 5.2% | 2.8% / 2.8% |
| zemmour | 7.1% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| pecresse | 4.8% | 11.1% (6.1%–17.0%) | 5.2% / 5.2% | 3.4% / 3.4% |
| jadot | 4.6% | 6.0% (2.1%–10.5%) | 0.9% / 0.9% | 0.5% / 0.5% |
| lassalle | 3.1% | 2.5% (0.8%–4.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 1.2% (0.3%–2.3%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 5.9% (3.0%–9.3%) | 0.2% / 0.2% | 0.1% / 0.1% |
| hidalgo | 1.8% | 5.7% (2.0%–10.0%) | 0.8% / 0.8% | 0.4% / 0.4% |
| poutou | 0.8% | 1.6% (0.8%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.3%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 304 days out (2021-06-10)

Actual matchup: current 82.5% (winner given matchup 77.3%), fitted 82.5% (winner given matchup 77.3%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 25.8% (15.6%–37.2%) | 89.7% / 89.7% ✓ | 69.5% / 69.5% ✓ |
| le_pen | 23.1% | 27.2% (17.4%–37.3%) | 92.6% / 92.6% ✓ | 24.2% / 24.2% |
| melenchon | 21.9% | 10.2% (4.5%–16.7%) | 7.4% / 7.4% | 2.8% / 2.8% |
| zemmour | 7.1% | 6.8% (2.0%–13.2%) | 5.1% / 5.1% | 1.2% / 1.2% |
| pecresse | 4.8% | 9.2% (5.6%–13.3%) | 2.4% / 2.4% | 1.2% / 1.2% |
| jadot | 4.6% | 5.6% (2.0%–10.0%) | 1.2% / 1.2% | 0.5% / 0.5% |
| lassalle | 3.1% | 1.5% (0.5%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 1.5% (0.4%–2.8%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 4.3% (2.4%–6.6%) | 0.1% / 0.1% | 0.0% / 0.0% |
| hidalgo | 1.8% | 5.9% (2.1%–10.6%) | 1.4% / 1.4% | 0.5% / 0.5% |
| poutou | 0.8% | 1.1% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.3%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 273 days out (2021-07-11)

Actual matchup: current 78.2% (winner given matchup 87.6%), fitted 78.2% (winner given matchup 87.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 24.9% (17.5%–32.7%) | 91.8% / 91.8% ✓ | 78.0% / 78.0% ✓ |
| le_pen | 23.1% | 23.9% (15.4%–32.8%) | 85.9% / 85.9% ✓ | 11.9% / 11.9% |
| melenchon | 21.9% | 8.9% (3.8%–14.8%) | 4.5% / 4.5% | 1.9% / 1.9% |
| zemmour | 7.1% | 6.2% (2.0%–11.7%) | 3.5% / 3.5% | 0.6% / 0.6% |
| pecresse | 4.8% | 12.0% (6.7%–18.3%) | 10.2% / 10.2% | 5.7% / 5.7% |
| jadot | 4.6% | 6.9% (2.7%–11.8%) | 2.0% / 2.0% | 0.9% / 0.9% |
| lassalle | 3.1% | 1.7% (0.6%–3.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.2% (0.7%–4.1%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 4.7% (2.6%–7.0%) | 0.1% / 0.1% | 0.0% / 0.0% |
| hidalgo | 1.8% | 6.6% (2.5%–11.5%) | 1.9% / 1.9% | 0.8% / 0.8% |
| poutou | 0.8% | 1.1% (0.5%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.4%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 243 days out (2021-08-10)

Actual matchup: current 75.8% (winner given matchup 87.6%), fitted 75.8% (winner given matchup 87.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 24.9% (17.1%–33.2%) | 90.4% / 90.4% ✓ | 76.5% / 76.5% ✓ |
| le_pen | 23.1% | 23.5% (15.1%–32.4%) | 84.7% / 84.7% ✓ | 11.8% / 11.8% |
| melenchon | 21.9% | 9.3% (4.1%–15.3%) | 5.4% / 5.4% | 2.4% / 2.4% |
| zemmour | 7.1% | 6.0% (2.0%–11.2%) | 3.0% / 3.0% | 0.5% / 0.5% |
| pecresse | 4.8% | 12.0% (6.3%–18.9%) | 12.1% / 12.1% | 6.9% / 6.9% |
| jadot | 4.6% | 6.9% (2.8%–11.8%) | 2.1% / 2.1% | 1.0% / 1.0% |
| lassalle | 3.1% | 1.7% (0.6%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.2% (0.7%–4.0%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 4.6% (2.4%–7.3%) | 0.1% / 0.1% | 0.1% / 0.1% |
| hidalgo | 1.8% | 6.7% (2.6%–11.6%) | 2.0% / 2.0% | 0.9% / 0.9% |
| poutou | 0.8% | 1.2% (0.5%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.4%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 212 days out (2021-09-10)

Actual matchup: current 77.9% (winner given matchup 88.0%), fitted 77.9% (winner given matchup 88.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 24.5% (18.7%–30.5%) | 95.2% / 95.2% ✓ | 80.3% / 80.3% ✓ |
| le_pen | 23.1% | 21.3% (14.2%–28.6%) | 82.2% / 82.2% ✓ | 10.3% / 10.3% |
| melenchon | 21.9% | 9.6% (4.6%–15.2%) | 5.7% / 5.7% | 2.3% / 2.3% |
| zemmour | 7.1% | 7.5% (3.0%–12.8%) | 4.2% / 4.2% | 0.6% / 0.6% |
| pecresse | 4.8% | 12.0% (7.8%–16.8%) | 7.6% / 7.6% | 4.2% / 4.2% |
| jadot | 4.6% | 7.3% (3.2%–12.1%) | 2.4% / 2.4% | 1.0% / 1.0% |
| lassalle | 3.1% | 1.6% (0.6%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.3% (0.9%–4.0%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 4.5% (2.8%–6.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 7.5% (3.3%–12.3%) | 2.5% / 2.5% | 1.1% / 1.1% |
| poutou | 0.8% | 1.1% (0.6%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.4%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 182 days out (2021-10-10)

Actual matchup: current 44.9% (winner given matchup 91.1%), fitted 44.9% (winner given matchup 91.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 25.8% (17.1%–35.2%) | 89.5% / 89.5% ✓ | 83.2% / 83.2% ✓ |
| le_pen | 23.1% | 17.6% (9.1%–27.0%) | 53.3% / 53.3% ✓ | 7.7% / 7.7% |
| melenchon | 21.9% | 8.1% (3.4%–13.5%) | 5.0% / 5.0% | 0.8% / 0.8% |
| zemmour | 7.1% | 15.0% (7.0%–24.1%) | 35.3% / 35.3% | 4.2% / 4.2% |
| pecresse | 4.8% | 11.5% (6.9%–16.9%) | 11.3% / 11.3% | 3.0% / 3.0% |
| jadot | 4.6% | 7.6% (3.2%–12.9%) | 4.0% / 4.0% | 0.8% / 0.8% |
| lassalle | 3.1% | 1.3% (0.4%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.2% (0.8%–4.1%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 3.5% (1.8%–5.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 5.4% (2.0%–9.5%) | 1.4% / 1.4% | 0.2% / 0.2% |
| poutou | 0.8% | 1.1% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.0% (0.4%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 152 days out (2021-11-09)

Actual matchup: current 39.7% (winner given matchup 90.4%), fitted 39.7% (winner given matchup 90.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 24.8% (16.8%–33.5%) | 88.5% / 88.5% ✓ | 82.8% / 82.8% ✓ |
| le_pen | 23.1% | 17.1% (8.9%–26.1%) | 48.7% / 48.7% ✓ | 7.6% / 7.6% |
| melenchon | 21.9% | 9.1% (4.3%–14.9%) | 7.0% / 7.0% | 1.2% / 1.2% |
| zemmour | 7.1% | 15.9% (8.1%–24.8%) | 40.4% / 40.4% | 5.0% / 5.0% |
| pecresse | 4.8% | 11.1% (6.3%–16.7%) | 10.2% / 10.2% | 2.4% / 2.4% |
| jadot | 4.6% | 7.7% (3.4%–12.8%) | 3.9% / 3.9% | 0.8% / 0.8% |
| lassalle | 3.1% | 1.2% (0.5%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.2% (0.8%–3.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 3.4% (1.9%–5.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 5.3% (2.1%–9.2%) | 1.2% / 1.2% | 0.2% / 0.2% |
| poutou | 0.8% | 1.2% (0.6%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.0% (0.4%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 122 days out (2021-12-09)

Actual matchup: current 20.8% (winner given matchup 90.4%), fitted 20.8% (winner given matchup 90.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 22.7% (15.5%–30.4%) | 80.5% / 80.5% ✓ | 68.3% / 68.3% ✓ |
| le_pen | 23.1% | 15.5% (8.7%–23.0%) | 32.6% / 32.6% ✓ | 4.3% / 4.3% |
| melenchon | 21.9% | 9.8% (4.9%–15.4%) | 6.8% / 6.8% | 1.1% / 1.1% |
| zemmour | 7.1% | 13.6% (7.1%–20.9%) | 21.1% / 21.1% | 3.2% / 3.2% |
| pecresse | 4.8% | 19.3% (12.1%–27.4%) | 56.9% / 56.9% | 22.6% / 22.6% |
| jadot | 4.6% | 6.7% (3.1%–11.0%) | 1.5% / 1.5% | 0.3% / 0.3% |
| lassalle | 3.1% | 1.1% (0.5%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.0% (0.8%–3.5%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.7% (1.5%–4.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 4.6% (1.9%–7.8%) | 0.4% / 0.4% | 0.1% / 0.1% |
| poutou | 0.8% | 1.1% (0.6%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.4%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 91 days out (2022-01-09)

Actual matchup: current 40.3% (winner given matchup 95.5%), fitted 40.3% (winner given matchup 95.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 25.1% (17.8%–32.7%) | 90.9% / 90.9% ✓ | 84.1% / 84.1% ✓ |
| le_pen | 23.1% | 16.9% (10.4%–23.8%) | 47.2% / 47.2% ✓ | 3.7% / 3.7% |
| melenchon | 21.9% | 10.8% (6.2%–16.0%) | 8.5% / 8.5% | 1.0% / 1.0% |
| zemmour | 7.1% | 13.3% (7.6%–19.7%) | 20.0% / 20.0% | 1.8% / 1.8% |
| pecresse | 4.8% | 15.6% (10.8%–20.8%) | 31.8% / 31.8% | 9.3% / 9.3% |
| jadot | 4.6% | 6.9% (3.5%–10.8%) | 1.4% / 1.4% | 0.2% / 0.2% |
| lassalle | 3.1% | 1.2% (0.6%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.4% (1.1%–3.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.3% (1.4%–3.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 3.6% (1.7%–5.8%) | 0.1% / 0.1% | 0.0% / 0.0% |
| poutou | 0.8% | 1.2% (0.7%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.4%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 84 days out (2022-01-16)

Actual matchup: current 33.6% (winner given matchup 95.0%), fitted 33.6% (winner given matchup 95.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 23.9% (17.0%–31.2%) | 88.1% / 88.1% ✓ | 81.1% / 81.1% ✓ |
| le_pen | 23.1% | 16.4% (10.0%–23.4%) | 41.9% / 41.9% ✓ | 4.1% / 4.1% |
| melenchon | 21.9% | 11.3% (6.4%–16.8%) | 11.1% / 11.1% | 1.5% / 1.5% |
| zemmour | 7.1% | 14.0% (8.0%–20.8%) | 24.0% / 24.0% | 2.6% / 2.6% |
| pecresse | 4.8% | 15.8% (10.9%–21.2%) | 33.5% / 33.5% | 10.4% / 10.4% |
| jadot | 4.6% | 6.6% (3.3%–10.4%) | 1.1% / 1.1% | 0.2% / 0.2% |
| lassalle | 3.1% | 1.4% (0.7%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.5% (1.1%–4.1%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.3% (1.4%–3.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 3.9% (1.8%–6.4%) | 0.2% / 0.2% | 0.0% / 0.0% |
| poutou | 0.8% | 1.2% (0.7%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.3%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 77 days out (2022-01-23)

Actual matchup: current 35.0% (winner given matchup 95.4%), fitted 35.0% (winner given matchup 95.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 24.2% (17.4%–31.4%) | 89.6% / 89.6% ✓ | 83.2% / 83.2% ✓ |
| le_pen | 23.1% | 16.6% (10.3%–23.4%) | 42.4% / 42.4% ✓ | 3.5% / 3.5% |
| melenchon | 21.9% | 12.0% (6.8%–17.8%) | 13.8% / 13.8% | 1.7% / 1.7% |
| zemmour | 7.1% | 12.8% (7.4%–18.9%) | 15.7% / 15.7% | 1.4% / 1.4% |
| pecresse | 4.8% | 16.2% (10.9%–22.1%) | 37.0% / 37.0% | 9.9% / 9.9% |
| jadot | 4.6% | 6.9% (3.5%–10.8%) | 1.2% / 1.2% | 0.2% / 0.2% |
| lassalle | 3.1% | 1.2% (0.6%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.8% (1.3%–4.6%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 1.9% (1.1%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 3.6% (1.7%–5.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| poutou | 0.8% | 1.1% (0.7%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 70 days out (2022-01-30)

Actual matchup: current 38.8% (winner given matchup 95.1%), fitted 38.8% (winner given matchup 95.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 23.6% (17.1%–30.4%) | 88.6% / 88.6% ✓ | 82.5% / 82.5% ✓ |
| le_pen | 23.1% | 17.2% (10.8%–24.1%) | 47.4% / 47.4% ✓ | 4.2% / 4.2% |
| melenchon | 21.9% | 11.2% (6.4%–16.6%) | 10.1% / 10.1% | 1.4% / 1.4% |
| zemmour | 7.1% | 13.4% (7.8%–19.7%) | 18.8% / 18.8% | 2.1% / 2.1% |
| pecresse | 4.8% | 15.9% (11.0%–21.4%) | 33.6% / 33.6% | 9.5% / 9.5% |
| jadot | 4.6% | 6.9% (3.5%–10.7%) | 1.3% / 1.3% | 0.2% / 0.2% |
| lassalle | 3.1% | 1.2% (0.6%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.1% (1.5%–4.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.1% (1.2%–3.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 3.7% (1.8%–5.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| poutou | 0.8% | 1.1% (0.7%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 63 days out (2022-02-06)

Actual matchup: current 41.4% (winner given matchup 94.9%), fitted 41.4% (winner given matchup 94.9%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 24.5% (18.2%–31.1%) | 92.0% / 92.0% ✓ | 86.9% / 86.9% ✓ |
| le_pen | 23.1% | 17.1% (10.9%–23.7%) | 47.6% / 47.6% ✓ | 4.2% / 4.2% |
| melenchon | 21.9% | 11.4% (6.9%–16.5%) | 9.8% / 9.8% | 1.0% / 1.0% |
| zemmour | 7.1% | 14.5% (8.8%–20.8%) | 25.4% / 25.4% | 2.0% / 2.0% |
| pecresse | 4.8% | 14.9% (10.5%–19.7%) | 24.4% / 24.4% | 5.7% / 5.7% |
| jadot | 4.6% | 6.3% (3.4%–9.8%) | 0.8% / 0.8% | 0.1% / 0.1% |
| lassalle | 3.1% | 1.2% (0.7%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.3% (1.6%–5.1%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.0% (1.2%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 3.2% (1.6%–5.1%) | 0.1% / 0.1% | 0.0% / 0.0% |
| poutou | 0.8% | 1.0% (0.6%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 56 days out (2022-02-13)

Actual matchup: current 39.0% (winner given matchup 95.4%), fitted 39.0% (winner given matchup 95.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 25.1% (18.8%–31.6%) | 93.0% / 93.0% ✓ | 88.1% / 88.1% ✓ |
| le_pen | 23.1% | 16.9% (11.0%–23.2%) | 44.2% / 44.2% ✓ | 3.4% / 3.4% |
| melenchon | 21.9% | 10.5% (6.4%–15.1%) | 6.0% / 6.0% | 0.5% / 0.5% |
| zemmour | 7.1% | 14.7% (9.2%–20.7%) | 25.0% / 25.0% | 1.8% / 1.8% |
| pecresse | 4.8% | 15.8% (11.1%–20.9%) | 31.1% / 31.1% | 6.1% / 6.1% |
| jadot | 4.6% | 5.9% (3.2%–8.9%) | 0.4% / 0.4% | 0.1% / 0.1% |
| lassalle | 3.1% | 1.2% (0.7%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 4.2% (2.2%–6.4%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 1.6% (1.0%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.7% (1.4%–4.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 49 days out (2022-02-20)

Actual matchup: current 28.5% (winner given matchup 95.1%), fitted 28.5% (winner given matchup 95.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 23.5% (17.7%–29.7%) | 90.6% / 90.6% ✓ | 87.4% / 87.4% ✓ |
| le_pen | 23.1% | 15.7% (10.3%–21.4%) | 34.3% / 34.3% ✓ | 3.2% / 3.2% |
| melenchon | 21.9% | 11.8% (7.2%–17.0%) | 12.0% / 12.0% | 1.3% / 1.3% |
| zemmour | 7.1% | 15.7% (10.1%–21.8%) | 34.3% / 34.3% | 2.7% / 2.7% |
| pecresse | 4.8% | 15.2% (10.5%–20.3%) | 28.0% / 28.0% | 5.2% / 5.2% |
| jadot | 4.6% | 5.8% (3.1%–8.9%) | 0.4% / 0.4% | 0.1% / 0.1% |
| lassalle | 3.1% | 1.4% (0.8%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 4.7% (2.5%–7.2%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 1.9% (1.1%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.6% (1.4%–4.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 0.9% (0.6%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 42 days out (2022-02-27)

Actual matchup: current 42.2% (winner given matchup 95.1%), fitted 42.2% (winner given matchup 95.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 25.3% (19.4%–31.4%) | 94.9% / 94.9% ✓ | 92.2% / 92.2% ✓ |
| le_pen | 23.1% | 16.8% (11.4%–22.6%) | 46.2% / 46.2% ✓ | 3.7% / 3.7% |
| melenchon | 21.9% | 12.7% (8.1%–17.8%) | 14.9% / 14.9% | 1.1% / 1.1% |
| zemmour | 7.1% | 15.7% (10.3%–21.4%) | 33.9% / 33.9% | 1.7% / 1.7% |
| pecresse | 4.8% | 12.7% (9.0%–16.8%) | 9.8% / 9.8% | 1.3% / 1.3% |
| jadot | 4.6% | 5.3% (2.9%–8.0%) | 0.2% / 0.2% | 0.0% / 0.0% |
| lassalle | 3.1% | 1.4% (0.8%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 4.1% (2.2%–6.3%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.1% (1.3%–3.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.3% (1.2%–3.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 0.9% (0.6%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.4%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 35 days out (2022-03-06)

Actual matchup: current 61.1% (winner given matchup 97.7%), fitted 61.1% (winner given matchup 97.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 29.9% (23.5%–36.4%) | 99.0% / 99.0% ✓ | 97.4% / 97.4% ✓ |
| le_pen | 23.1% | 16.9% (11.8%–22.3%) | 62.0% / 62.0% ✓ | 1.8% / 1.8% |
| melenchon | 21.9% | 11.2% (7.1%–15.6%) | 10.4% / 10.4% | 0.1% / 0.1% |
| zemmour | 7.1% | 13.4% (8.9%–18.3%) | 20.0% / 20.0% | 0.3% / 0.3% |
| pecresse | 4.8% | 11.7% (8.3%–15.4%) | 8.2% / 8.2% | 0.2% / 0.2% |
| jadot | 4.6% | 6.0% (3.4%–8.9%) | 0.4% / 0.4% | 0.0% / 0.0% |
| lassalle | 3.1% | 1.6% (0.9%–2.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.4% (1.9%–5.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.1% (1.3%–3.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.3% (1.2%–3.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.0% (0.7%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 28 days out (2022-03-13)

Actual matchup: current 61.7% (winner given matchup 98.6%), fitted 61.7% (winner given matchup 98.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 31.4% (25.3%–37.7%) | 99.5% / 99.5% ✓ | 98.5% / 98.5% ✓ |
| le_pen | 23.1% | 16.1% (11.6%–20.9%) | 62.2% / 62.2% ✓ | 1.1% / 1.1% |
| melenchon | 21.9% | 11.5% (7.8%–15.6%) | 12.2% / 12.2% | 0.1% / 0.1% |
| zemmour | 7.1% | 12.6% (8.5%–17.0%) | 16.7% / 16.7% | 0.2% / 0.2% |
| pecresse | 4.8% | 11.6% (8.3%–15.1%) | 9.3% / 9.3% | 0.1% / 0.1% |
| jadot | 4.6% | 5.3% (3.2%–7.6%) | 0.1% / 0.1% | 0.0% / 0.0% |
| lassalle | 3.1% | 2.1% (1.3%–3.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.4% (2.0%–5.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.0% (1.2%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.1% (1.2%–3.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.2% (0.8%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 21 days out (2022-03-20)

Actual matchup: current 74.5% (winner given matchup 99.1%), fitted 74.5% (winner given matchup 99.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 30.7% (24.9%–36.6%) | 99.5% / 99.5% ✓ | 98.8% / 98.8% ✓ |
| le_pen | 23.1% | 17.1% (12.6%–21.7%) | 74.9% / 74.9% ✓ | 0.9% / 0.9% |
| melenchon | 21.9% | 12.6% (8.9%–16.5%) | 15.6% / 15.6% | 0.1% / 0.1% |
| zemmour | 7.1% | 11.1% (7.6%–15.0%) | 7.1% / 7.1% | 0.1% / 0.1% |
| pecresse | 4.8% | 10.2% (7.5%–13.1%) | 2.7% / 2.7% | 0.1% / 0.1% |
| jadot | 4.6% | 5.6% (3.5%–7.8%) | 0.2% / 0.2% | 0.0% / 0.0% |
| lassalle | 3.1% | 2.5% (1.6%–3.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.8% (2.4%–5.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.3% (1.4%–3.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.5% (1.5%–3.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.0% (0.7%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.4%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 14 days out (2022-03-27)

Actual matchup: current 74.9% (winner given matchup 97.9%), fitted 74.9% (winner given matchup 97.9%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 27.3% (22.2%–32.5%) | 98.6% / 98.6% ✓ | 97.0% / 97.0% ✓ |
| le_pen | 23.1% | 18.7% (14.3%–23.3%) | 76.2% / 76.2% ✓ | 2.1% / 2.1% |
| melenchon | 21.9% | 14.5% (10.6%–18.6%) | 19.7% / 19.7% | 0.6% / 0.6% |
| zemmour | 7.1% | 10.7% (7.5%–14.1%) | 3.0% / 3.0% | 0.1% / 0.1% |
| pecresse | 4.8% | 11.2% (8.3%–14.2%) | 2.4% / 2.4% | 0.1% / 0.1% |
| jadot | 4.6% | 4.9% (3.2%–6.8%) | 0.1% / 0.1% | 0.0% / 0.0% |
| lassalle | 3.1% | 2.6% (1.7%–3.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.9% (2.5%–5.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.0% (1.3%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.3% (1.5%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.0% (0.7%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.5%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 7 days out (2022-04-03)

Actual matchup: current 83.8% (winner given matchup 93.7%), fitted 83.8% (winner given matchup 93.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 26.1% (21.4%–30.9%) | 97.7% / 97.7% ✓ | 92.3% / 92.3% ✓ |
| le_pen | 23.1% | 21.3% (16.8%–25.8%) | 86.1% / 86.1% ✓ | 6.3% / 6.3% |
| melenchon | 21.9% | 15.5% (11.7%–19.4%) | 14.3% / 14.3% | 1.2% / 1.2% |
| zemmour | 7.1% | 10.8% (7.8%–14.0%) | 1.7% / 1.7% | 0.1% / 0.1% |
| pecresse | 4.8% | 8.8% (6.8%–11.0%) | 0.2% / 0.2% | 0.0% / 0.0% |
| jadot | 4.6% | 5.3% (3.6%–7.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 3.1% | 2.5% (1.7%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.5% (2.3%–4.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.2% (1.5%–3.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.1% (1.4%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.1% (0.8%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.5%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

## Signal layers

Every replay fitted again with news and attention spikes allowed to add variance (model/src/signals.rs), `shock_sd` chosen by marginal likelihood from [0.0, 0.1, 0.2, 0.3, 0.5], and simulated at the current election-day errors. Contenders are those of the polls-only replays. The first row is the published settings, the signal weights in config/model.yaml, which the owner sets; the others are alternatives. "No worse" marks a row that changes something (a `shock_sd` above zero, or a momentum tilt, in at least one replay) and scores at least as well as the polls only on both elections, on the share log score, the runoff log score and the qualification and win log-loss.

| | Qualify Brier | Qualify log-loss | Win Brier | Win log-loss | Share MAE (pts) | 80% coverage | Share log score | Round 2 log score | P(winner \| matchup) |
|---|---|---|---|---|---|---|---|---|---|
| Polls only | 0.048 | 0.178 | 0.023 | 0.078 | 2.30 | 68.3% | -110.1 | -43.91 | 84.3% |
| published settings (config/model.yaml) | 0.049 | 0.179 | 0.023 | 0.080 | 2.31 | 67.0% | -113.9 | -45.17 | 84.1% |
| page views and news | 0.048 | 0.179 | 0.023 | 0.079 | 2.30 | 68.3% | -109.9 | -43.91 | 84.3% |
| page views | 0.048 | 0.179 | 0.023 | 0.079 | 2.30 | 68.3% | -109.1 | -43.91 | 84.3% |
| news (no effect) | 0.048 | 0.178 | 0.023 | 0.078 | 2.30 | 68.3% | -110.1 | -43.91 | 84.3% |
| both, spike at 2× | 0.048 | 0.179 | 0.023 | 0.078 | 2.30 | 68.3% | -110.1 | -43.94 | 84.3% |
| both, spike at 5× | 0.048 | 0.178 | 0.023 | 0.078 | 2.30 | 67.2% | -115.9 | -43.90 | 84.3% |
| both, half-life 5 days | 0.048 | 0.178 | 0.023 | 0.078 | 2.30 | 68.0% | -110.1 | -43.87 | 84.3% |
| both, half-life 20 days | 0.048 | 0.179 | 0.023 | 0.078 | 2.30 | 68.3% | -109.5 | -43.95 | 84.3% |
| both, higher floors | 0.048 | 0.178 | 0.023 | 0.078 | 2.30 | 67.3% | -115.3 | -43.87 | 84.3% |
| momentum +0.05 | 0.048 | 0.178 | 0.023 | 0.078 | 2.30 | 68.3% | -109.5 | -43.93 | 84.3% |
| momentum +0.15 | 0.048 | 0.178 | 0.023 | 0.078 | 2.30 | 68.1% | -108.4 | -43.95 | 84.3% |
| momentum −0.05 | 0.048 | 0.178 | 0.023 | 0.078 | 2.31 | 68.4% | -111.1 | -43.91 | 84.3% |
| momentum −0.15 | 0.049 | 0.180 | 0.023 | 0.079 | 2.31 | 68.1% | -115.2 | -43.90 | 84.3% |
| tone +0.02 | 0.048 | 0.178 | 0.023 | 0.078 | 2.30 | 68.3% | -110.0 | -43.92 | 84.3% |
| tone +0.05 | 0.048 | 0.178 | 0.023 | 0.078 | 2.30 | 68.2% | -109.8 | -43.92 | 84.3% |
| tone +0.10 | 0.048 | 0.179 | 0.023 | 0.078 | 2.31 | 68.1% | -109.3 | -43.93 | 84.3% |
| tone +0.20 | 0.049 | 0.181 | 0.023 | 0.078 | 2.32 | 67.8% | -111.6 | -43.93 | 84.3% |
| tone −0.05 | 0.048 | 0.178 | 0.023 | 0.078 | 2.30 | 68.4% | -110.7 | -43.93 | 84.3% |
| momentum +0.15, tone +0.05 | 0.048 | 0.178 | 0.023 | 0.078 | 2.30 | 68.3% | -107.4 | -43.96 | 84.3% |

By election:

| | Qualify Brier | Qualify log-loss | Win Brier | Win log-loss | Share MAE (pts) | 80% coverage | Share log score | Round 2 log score | P(winner \| matchup) |
|---|---|---|---|---|---|---|---|---|---|
| 2002, polls only | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, published settings (config/model.yaml) | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, page views and news | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, page views | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, news | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, both, spike at 2× | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, both, spike at 5× | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, both, half-life 5 days | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, both, half-life 20 days | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, both, higher floors | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, momentum +0.05 | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, momentum +0.15 | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, momentum −0.05 | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, momentum −0.15 | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, tone +0.02 | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, tone +0.05 | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, tone +0.10 | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, tone +0.20 | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, tone −0.05 | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2002, momentum +0.15, tone +0.05 | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2007, polls only | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, published settings (config/model.yaml) | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, page views and news | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, page views | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, news | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, both, spike at 2× | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, both, spike at 5× | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, both, half-life 5 days | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, both, half-life 20 days | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, both, higher floors | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, momentum +0.05 | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, momentum +0.15 | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, momentum −0.05 | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, momentum −0.15 | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, tone +0.02 | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, tone +0.05 | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, tone +0.10 | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, tone +0.20 | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, tone −0.05 | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2007, momentum +0.15, tone +0.05 | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2012, polls only | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, published settings (config/model.yaml) | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, page views and news | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, page views | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, news | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, both, spike at 2× | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, both, spike at 5× | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, both, half-life 5 days | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, both, half-life 20 days | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, both, higher floors | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, momentum +0.05 | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, momentum +0.15 | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, momentum −0.05 | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, momentum −0.15 | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, tone +0.02 | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, tone +0.05 | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, tone +0.10 | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, tone +0.20 | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, tone −0.05 | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2012, momentum +0.15, tone +0.05 | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2017, polls only | 0.057 | 0.178 | 0.053 | 0.168 | 2.17 | 68.9% | -17.1 | -9.65 | 85.8% |
| 2017, published settings (config/model.yaml) | 0.061 | 0.188 | 0.056 | 0.180 | 2.17 | 66.2% | -21.5 | -10.85 | 84.6% |
| 2017, page views and news | 0.058 | 0.181 | 0.054 | 0.172 | 2.15 | 68.9% | -16.9 | -9.64 | 85.8% |
| 2017, page views | 0.059 | 0.183 | 0.055 | 0.175 | 2.15 | 68.9% | -16.2 | -9.65 | 85.8% |
| 2017, news | 0.057 | 0.178 | 0.053 | 0.168 | 2.17 | 68.9% | -17.1 | -9.65 | 85.8% |
| 2017, both, spike at 2× | 0.058 | 0.181 | 0.054 | 0.171 | 2.16 | 68.9% | -17.1 | -9.68 | 85.8% |
| 2017, both, spike at 5× | 0.059 | 0.184 | 0.055 | 0.174 | 2.16 | 69.4% | -16.6 | -9.66 | 85.8% |
| 2017, both, half-life 5 days | 0.056 | 0.175 | 0.054 | 0.171 | 2.16 | 67.1% | -17.2 | -9.60 | 85.8% |
| 2017, both, half-life 20 days | 0.058 | 0.180 | 0.054 | 0.170 | 2.15 | 68.9% | -16.6 | -9.68 | 85.8% |
| 2017, both, higher floors | 0.059 | 0.183 | 0.055 | 0.174 | 2.17 | 68.5% | -16.8 | -9.62 | 85.8% |
| 2017, momentum +0.05 | 0.056 | 0.176 | 0.053 | 0.167 | 2.16 | 68.9% | -16.4 | -9.65 | 85.7% |
| 2017, momentum +0.15 | 0.056 | 0.175 | 0.053 | 0.166 | 2.16 | 69.4% | -15.7 | -9.62 | 85.9% |
| 2017, momentum −0.05 | 0.057 | 0.179 | 0.053 | 0.169 | 2.18 | 69.4% | -18.1 | -9.66 | 85.8% |
| 2017, momentum −0.15 | 0.059 | 0.185 | 0.054 | 0.172 | 2.21 | 69.4% | -20.8 | -9.70 | 85.7% |
| 2017, tone +0.02 | 0.057 | 0.178 | 0.053 | 0.168 | 2.17 | 68.9% | -17.2 | -9.64 | 85.8% |
| 2017, tone +0.05 | 0.057 | 0.179 | 0.053 | 0.168 | 2.16 | 69.4% | -17.3 | -9.61 | 85.8% |
| 2017, tone +0.10 | 0.058 | 0.181 | 0.053 | 0.168 | 2.16 | 68.9% | -17.9 | -9.59 | 85.8% |
| 2017, tone +0.20 | 0.062 | 0.188 | 0.053 | 0.168 | 2.19 | 68.0% | -20.1 | -9.51 | 85.9% |
| 2017, tone −0.05 | 0.057 | 0.177 | 0.053 | 0.168 | 2.18 | 69.4% | -17.3 | -9.70 | 85.8% |
| 2017, momentum +0.15, tone +0.05 | 0.057 | 0.176 | 0.053 | 0.166 | 2.15 | 68.9% | -15.9 | -9.59 | 85.8% |
| 2022, polls only | 0.026 | 0.099 | 0.004 | 0.029 | 3.30 | 64.1% | -132.4 | 14.18 | 92.8% |
| 2022, published settings (config/model.yaml) | 0.025 | 0.097 | 0.004 | 0.027 | 3.34 | 60.2% | -131.8 | 14.13 | 92.8% |
| 2022, page views and news | 0.026 | 0.099 | 0.004 | 0.029 | 3.30 | 64.1% | -132.4 | 14.18 | 92.8% |
| 2022, page views | 0.026 | 0.099 | 0.004 | 0.029 | 3.30 | 64.1% | -132.4 | 14.18 | 92.8% |
| 2022, news | 0.026 | 0.099 | 0.004 | 0.029 | 3.30 | 64.1% | -132.4 | 14.18 | 92.8% |
| 2022, both, spike at 2× | 0.026 | 0.099 | 0.004 | 0.029 | 3.30 | 64.1% | -132.4 | 14.18 | 92.8% |
| 2022, both, spike at 5× | 0.025 | 0.094 | 0.003 | 0.025 | 3.31 | 58.2% | -138.7 | 14.20 | 92.8% |
| 2022, both, half-life 5 days | 0.026 | 0.099 | 0.004 | 0.029 | 3.30 | 64.1% | -132.4 | 14.18 | 92.8% |
| 2022, both, half-life 20 days | 0.026 | 0.099 | 0.004 | 0.029 | 3.30 | 64.1% | -132.4 | 14.18 | 92.8% |
| 2022, both, higher floors | 0.025 | 0.095 | 0.003 | 0.026 | 3.31 | 59.4% | -138.0 | 14.20 | 92.8% |
| 2022, momentum +0.05 | 0.026 | 0.099 | 0.004 | 0.029 | 3.30 | 63.7% | -132.5 | 14.16 | 92.8% |
| 2022, momentum +0.15 | 0.026 | 0.100 | 0.004 | 0.030 | 3.31 | 62.5% | -132.1 | 14.11 | 92.8% |
| 2022, momentum −0.05 | 0.026 | 0.100 | 0.004 | 0.029 | 3.31 | 64.1% | -132.4 | 14.20 | 92.9% |
| 2022, momentum −0.15 | 0.027 | 0.101 | 0.004 | 0.028 | 3.32 | 62.5% | -133.8 | 14.24 | 92.9% |
| 2022, tone +0.02 | 0.026 | 0.100 | 0.004 | 0.029 | 3.31 | 63.7% | -132.3 | 14.16 | 92.8% |
| 2022, tone +0.05 | 0.026 | 0.100 | 0.004 | 0.029 | 3.31 | 62.9% | -131.9 | 14.14 | 92.8% |
| 2022, tone +0.10 | 0.027 | 0.101 | 0.004 | 0.030 | 3.33 | 62.9% | -130.8 | 14.10 | 92.8% |
| 2022, tone +0.20 | 0.029 | 0.105 | 0.004 | 0.032 | 3.36 | 62.2% | -131.0 | 14.02 | 92.7% |
| 2022, tone −0.05 | 0.026 | 0.099 | 0.004 | 0.029 | 3.30 | 64.1% | -132.9 | 14.22 | 92.9% |
| 2022, momentum +0.15, tone +0.05 | 0.026 | 0.100 | 0.004 | 0.030 | 3.32 | 63.7% | -130.9 | 14.07 | 92.8% |

Spikes detected and `shock_sd` chosen, per replay (year, days before round 1):

| Replay | published settings (config/model.yaml) | page views and news | page views | news | both, spike at 2× | both, spike at 5× | both, half-life 5 days | both, half-life 20 days | both, higher floors | momentum +0.05 | momentum +0.15 | momentum −0.05 | momentum −0.15 | tone +0.02 | tone +0.05 | tone +0.10 | tone +0.20 | tone −0.05 | momentum +0.15, tone +0.05 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 2002 334 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 304 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 273 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 243 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 212 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 182 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 152 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 122 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 91 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 84 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 77 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 70 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 63 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 56 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 49 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 42 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 35 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 28 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 21 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 14 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 7 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 334 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 304 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 273 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 243 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 212 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 182 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 152 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 122 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 91 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 84 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 77 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 70 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 63 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 56 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 49 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 42 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 35 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 28 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 21 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 14 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 7 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 334 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 304 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 273 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 243 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 212 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 182 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 152 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 122 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 91 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 84 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 77 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 70 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 63 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 56 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 49 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 42 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 35 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 28 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 21 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 14 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 7 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2017 334 | 21 spikes, 0.1 | 21 spikes, 0 | 22 spikes, 0 | 33 spikes, 0 | 37 spikes, 0 | 8 spikes, 0 | 21 spikes, 0 | 21 spikes, 0 | 7 spikes, 0 | 21 spikes, 0 | 21 spikes, 0 | 21 spikes, 0 | 21 spikes, 0 | 21 spikes, 0 | 21 spikes, 0 | 21 spikes, 0 | 21 spikes, 0 | 21 spikes, 0 | 21 spikes, 0 |
| 2017 304 | 34 spikes, 0.1 | 34 spikes, 0 | 37 spikes, 0 | 54 spikes, 0 | 61 spikes, 0 | 12 spikes, 0 | 34 spikes, 0 | 34 spikes, 0 | 10 spikes, 0 | 34 spikes, 0 | 34 spikes, 0 | 34 spikes, 0 | 34 spikes, 0 | 34 spikes, 0 | 34 spikes, 0 | 34 spikes, 0 | 34 spikes, 0 | 34 spikes, 0 | 34 spikes, 0 |
| 2017 273 | 44 spikes, 0.1 | 44 spikes, 0 | 53 spikes, 0 | 73 spikes, 0 | 81 spikes, 0 | 14 spikes, 0 | 44 spikes, 0 | 44 spikes, 0 | 13 spikes, 0 | 44 spikes, 0 | 44 spikes, 0 | 44 spikes, 0 | 44 spikes, 0 | 44 spikes, 0 | 44 spikes, 0 | 44 spikes, 0 | 44 spikes, 0 | 44 spikes, 0 | 44 spikes, 0 |
| 2017 243 | 52 spikes, 0.1 | 52 spikes, 0 | 65 spikes, 0 | 87 spikes, 0 | 98 spikes, 0 | 19 spikes, 0 | 52 spikes, 0 | 52 spikes, 0 | 18 spikes, 0 | 52 spikes, 0 | 52 spikes, 0 | 52 spikes, 0 | 52 spikes, 0 | 52 spikes, 0 | 52 spikes, 0 | 52 spikes, 0 | 52 spikes, 0 | 52 spikes, 0 | 52 spikes, 0 |
| 2017 212 | 74 spikes, 0.1 | 74 spikes, 0 | 92 spikes, 0 | 110 spikes, 0 | 134 spikes, 0 | 30 spikes, 0 | 74 spikes, 0 | 74 spikes, 0 | 25 spikes, 0 | 74 spikes, 0 | 74 spikes, 0 | 74 spikes, 0 | 74 spikes, 0 | 74 spikes, 0 | 74 spikes, 0 | 74 spikes, 0 | 74 spikes, 0 | 74 spikes, 0 | 74 spikes, 0 |
| 2017 182 | 94 spikes, 0.1 | 94 spikes, 0 | 112 spikes, 0 | 132 spikes, 0 | 165 spikes, 0 | 39 spikes, 0 | 94 spikes, 0 | 94 spikes, 0 | 36 spikes, 0 | 94 spikes, 0 | 94 spikes, 0 | 94 spikes, 0 | 94 spikes, 0 | 94 spikes, 0 | 94 spikes, 0 | 94 spikes, 0 | 94 spikes, 0 | 94 spikes, 0 | 94 spikes, 0 |
| 2017 152 | 110 spikes, 0.1 | 110 spikes, 0 | 136 spikes, 0 | 150 spikes, 0 | 191 spikes, 0 | 49 spikes, 0 | 110 spikes, 0 | 110 spikes, 0 | 47 spikes, 0 | 110 spikes, 0 | 110 spikes, 0 | 110 spikes, 0 | 110 spikes, 0 | 110 spikes, 0 | 110 spikes, 0 | 110 spikes, 0 | 110 spikes, 0 | 110 spikes, 0 | 110 spikes, 0 |
| 2017 122 | 125 spikes, 0.1 | 125 spikes, 0.1 | 158 spikes, 0.1 | 167 spikes, 0 | 216 spikes, 0.1 | 60 spikes, 0.1 | 125 spikes, 0.1 | 125 spikes, 0.1 | 55 spikes, 0.1 | 125 spikes, 0 | 125 spikes, 0 | 125 spikes, 0 | 125 spikes, 0 | 125 spikes, 0 | 125 spikes, 0 | 125 spikes, 0 | 125 spikes, 0 | 125 spikes, 0 | 125 spikes, 0 |
| 2017 91 | 139 spikes, 0.1 | 139 spikes, 0.1 | 176 spikes, 0.1 | 189 spikes, 0 | 244 spikes, 0 | 65 spikes, 0.1 | 139 spikes, 0.1 | 139 spikes, 0.1 | 65 spikes, 0.1 | 139 spikes, 0 | 139 spikes, 0 | 139 spikes, 0 | 139 spikes, 0 | 139 spikes, 0 | 139 spikes, 0 | 139 spikes, 0 | 139 spikes, 0 | 139 spikes, 0 | 139 spikes, 0 |
| 2017 84 | 146 spikes, 0.1 | 146 spikes, 0.1 | 182 spikes, 0.1 | 192 spikes, 0 | 250 spikes, 0 | 70 spikes, 0.1 | 146 spikes, 0.1 | 146 spikes, 0.1 | 68 spikes, 0.1 | 146 spikes, 0 | 146 spikes, 0 | 146 spikes, 0 | 146 spikes, 0 | 146 spikes, 0 | 146 spikes, 0 | 146 spikes, 0 | 146 spikes, 0 | 146 spikes, 0 | 146 spikes, 0 |
| 2017 77 | 152 spikes, 0.1 | 152 spikes, 0.1 | 189 spikes, 0.1 | 196 spikes, 0 | 256 spikes, 0.1 | 72 spikes, 0.1 | 152 spikes, 0.1 | 152 spikes, 0.1 | 72 spikes, 0.2 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 |
| 2017 70 | 155 spikes, 0.1 | 155 spikes, 0.1 | 192 spikes, 0.1 | 201 spikes, 0 | 261 spikes, 0 | 74 spikes, 0.1 | 155 spikes, 0.1 | 155 spikes, 0.1 | 75 spikes, 0.1 | 155 spikes, 0 | 155 spikes, 0 | 155 spikes, 0 | 155 spikes, 0 | 155 spikes, 0 | 155 spikes, 0 | 155 spikes, 0 | 155 spikes, 0 | 155 spikes, 0 | 155 spikes, 0 |
| 2017 63 | 156 spikes, 0.1 | 156 spikes, 0.1 | 195 spikes, 0 | 203 spikes, 0 | 266 spikes, 0 | 74 spikes, 0.1 | 156 spikes, 0.1 | 156 spikes, 0 | 75 spikes, 0.1 | 156 spikes, 0 | 156 spikes, 0 | 156 spikes, 0 | 156 spikes, 0 | 156 spikes, 0 | 156 spikes, 0 | 156 spikes, 0 | 156 spikes, 0 | 156 spikes, 0 | 156 spikes, 0 |
| 2017 56 | 160 spikes, 0.1 | 160 spikes, 0.1 | 202 spikes, 0 | 211 spikes, 0 | 277 spikes, 0 | 76 spikes, 0.1 | 160 spikes, 0.1 | 160 spikes, 0 | 77 spikes, 0.1 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 |
| 2017 49 | 167 spikes, 0.1 | 167 spikes, 0.1 | 209 spikes, 0 | 221 spikes, 0 | 286 spikes, 0 | 78 spikes, 0.1 | 167 spikes, 0.1 | 167 spikes, 0 | 80 spikes, 0.1 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 |
| 2017 42 | 171 spikes, 0.1 | 171 spikes, 0.1 | 215 spikes, 0 | 224 spikes, 0 | 290 spikes, 0 | 81 spikes, 0.1 | 171 spikes, 0.1 | 171 spikes, 0 | 81 spikes, 0.1 | 171 spikes, 0 | 171 spikes, 0 | 171 spikes, 0 | 171 spikes, 0 | 171 spikes, 0 | 171 spikes, 0 | 171 spikes, 0 | 171 spikes, 0 | 171 spikes, 0 | 171 spikes, 0 |
| 2017 35 | 177 spikes, 0.1 | 177 spikes, 0 | 222 spikes, 0 | 232 spikes, 0 | 300 spikes, 0 | 84 spikes, 0 | 177 spikes, 0.1 | 177 spikes, 0 | 84 spikes, 0.1 | 177 spikes, 0 | 177 spikes, 0 | 177 spikes, 0 | 177 spikes, 0 | 177 spikes, 0 | 177 spikes, 0 | 177 spikes, 0 | 177 spikes, 0 | 177 spikes, 0 | 177 spikes, 0 |
| 2017 28 | 180 spikes, 0.1 | 180 spikes, 0 | 228 spikes, 0 | 234 spikes, 0 | 304 spikes, 0 | 84 spikes, 0 | 180 spikes, 0.1 | 180 spikes, 0 | 86 spikes, 0.1 | 180 spikes, 0 | 180 spikes, 0 | 180 spikes, 0 | 180 spikes, 0 | 180 spikes, 0 | 180 spikes, 0 | 180 spikes, 0 | 180 spikes, 0 | 180 spikes, 0 | 180 spikes, 0 |
| 2017 21 | 183 spikes, 0.1 | 183 spikes, 0 | 232 spikes, 0 | 241 spikes, 0 | 312 spikes, 0 | 85 spikes, 0 | 183 spikes, 0.1 | 183 spikes, 0 | 87 spikes, 0.1 | 183 spikes, 0 | 183 spikes, 0 | 183 spikes, 0 | 183 spikes, 0 | 183 spikes, 0 | 183 spikes, 0 | 183 spikes, 0 | 183 spikes, 0 | 183 spikes, 0 | 183 spikes, 0 |
| 2017 14 | 190 spikes, 0.1 | 190 spikes, 0 | 239 spikes, 0 | 245 spikes, 0 | 323 spikes, 0 | 89 spikes, 0 | 190 spikes, 0.1 | 190 spikes, 0 | 93 spikes, 0.1 | 190 spikes, 0 | 190 spikes, 0 | 190 spikes, 0 | 190 spikes, 0 | 190 spikes, 0 | 190 spikes, 0 | 190 spikes, 0 | 190 spikes, 0 | 190 spikes, 0 | 190 spikes, 0 |
| 2017 7 | 192 spikes, 0.1 | 192 spikes, 0 | 240 spikes, 0 | 249 spikes, 0 | 327 spikes, 0 | 89 spikes, 0.1 | 192 spikes, 0.1 | 192 spikes, 0 | 93 spikes, 0.1 | 192 spikes, 0 | 192 spikes, 0 | 192 spikes, 0 | 192 spikes, 0 | 192 spikes, 0 | 192 spikes, 0 | 192 spikes, 0 | 192 spikes, 0 | 192 spikes, 0 | 192 spikes, 0 |
| 2022 334 | 19 spikes, 0.1 | 19 spikes, 0 | 30 spikes, 0 | 22 spikes, 0 | 38 spikes, 0 | 9 spikes, 0 | 19 spikes, 0 | 19 spikes, 0 | 8 spikes, 0 | 19 spikes, 0 | 19 spikes, 0 | 19 spikes, 0 | 19 spikes, 0 | 19 spikes, 0 | 19 spikes, 0 | 19 spikes, 0 | 19 spikes, 0 | 19 spikes, 0 | 19 spikes, 0 |
| 2022 304 | 26 spikes, 0.1 | 26 spikes, 0 | 45 spikes, 0 | 36 spikes, 0 | 58 spikes, 0 | 11 spikes, 0 | 26 spikes, 0 | 26 spikes, 0 | 12 spikes, 0 | 26 spikes, 0 | 26 spikes, 0 | 26 spikes, 0 | 26 spikes, 0 | 26 spikes, 0 | 26 spikes, 0 | 26 spikes, 0 | 26 spikes, 0 | 26 spikes, 0 | 26 spikes, 0 |
| 2022 273 | 46 spikes, 0.1 | 46 spikes, 0 | 69 spikes, 0 | 52 spikes, 0 | 82 spikes, 0 | 19 spikes, 0 | 46 spikes, 0 | 46 spikes, 0 | 18 spikes, 0 | 46 spikes, 0 | 46 spikes, 0 | 46 spikes, 0 | 46 spikes, 0 | 46 spikes, 0 | 46 spikes, 0 | 46 spikes, 0 | 46 spikes, 0 | 46 spikes, 0 | 46 spikes, 0 |
| 2022 243 | 56 spikes, 0.1 | 56 spikes, 0 | 81 spikes, 0 | 63 spikes, 0 | 96 spikes, 0 | 23 spikes, 0 | 56 spikes, 0 | 56 spikes, 0 | 22 spikes, 0 | 56 spikes, 0 | 56 spikes, 0 | 56 spikes, 0 | 56 spikes, 0 | 56 spikes, 0 | 56 spikes, 0 | 56 spikes, 0 | 56 spikes, 0 | 56 spikes, 0 | 56 spikes, 0 |
| 2022 212 | 77 spikes, 0.1 | 77 spikes, 0 | 103 spikes, 0 | 86 spikes, 0 | 126 spikes, 0 | 33 spikes, 0.1 | 77 spikes, 0 | 77 spikes, 0 | 32 spikes, 0.1 | 77 spikes, 0 | 77 spikes, 0 | 77 spikes, 0 | 77 spikes, 0 | 77 spikes, 0 | 77 spikes, 0 | 77 spikes, 0 | 77 spikes, 0 | 77 spikes, 0 | 77 spikes, 0 |
| 2022 182 | 87 spikes, 0.1 | 87 spikes, 0 | 123 spikes, 0 | 100 spikes, 0 | 147 spikes, 0 | 41 spikes, 0.1 | 87 spikes, 0 | 87 spikes, 0 | 39 spikes, 0.1 | 87 spikes, 0 | 87 spikes, 0 | 87 spikes, 0 | 87 spikes, 0 | 87 spikes, 0 | 87 spikes, 0 | 87 spikes, 0 | 87 spikes, 0 | 87 spikes, 0 | 87 spikes, 0 |
| 2022 152 | 105 spikes, 0.1 | 105 spikes, 0 | 147 spikes, 0 | 112 spikes, 0 | 180 spikes, 0 | 46 spikes, 0.1 | 105 spikes, 0 | 105 spikes, 0 | 44 spikes, 0.1 | 105 spikes, 0 | 105 spikes, 0 | 105 spikes, 0 | 105 spikes, 0 | 105 spikes, 0 | 105 spikes, 0 | 105 spikes, 0 | 105 spikes, 0 | 105 spikes, 0 | 105 spikes, 0 |
| 2022 122 | 126 spikes, 0.1 | 126 spikes, 0 | 171 spikes, 0 | 130 spikes, 0 | 212 spikes, 0 | 56 spikes, 0.1 | 126 spikes, 0 | 126 spikes, 0 | 56 spikes, 0.1 | 126 spikes, 0 | 126 spikes, 0 | 126 spikes, 0 | 126 spikes, 0 | 126 spikes, 0 | 126 spikes, 0 | 126 spikes, 0 | 126 spikes, 0 | 126 spikes, 0 | 126 spikes, 0 |
| 2022 91 | 134 spikes, 0.1 | 134 spikes, 0 | 183 spikes, 0 | 142 spikes, 0 | 229 spikes, 0 | 58 spikes, 0.1 | 134 spikes, 0 | 134 spikes, 0 | 59 spikes, 0.1 | 134 spikes, 0 | 134 spikes, 0 | 134 spikes, 0 | 134 spikes, 0 | 134 spikes, 0 | 134 spikes, 0 | 134 spikes, 0 | 134 spikes, 0 | 134 spikes, 0 | 134 spikes, 0 |
| 2022 84 | 140 spikes, 0.1 | 140 spikes, 0 | 188 spikes, 0 | 147 spikes, 0 | 237 spikes, 0 | 59 spikes, 0.1 | 140 spikes, 0 | 140 spikes, 0 | 60 spikes, 0.1 | 140 spikes, 0 | 140 spikes, 0 | 140 spikes, 0 | 140 spikes, 0 | 140 spikes, 0 | 140 spikes, 0 | 140 spikes, 0 | 140 spikes, 0 | 140 spikes, 0 | 140 spikes, 0 |
| 2022 77 | 145 spikes, 0.1 | 145 spikes, 0 | 200 spikes, 0 | 154 spikes, 0 | 247 spikes, 0 | 65 spikes, 0.1 | 145 spikes, 0 | 145 spikes, 0 | 65 spikes, 0.1 | 145 spikes, 0 | 145 spikes, 0 | 145 spikes, 0 | 145 spikes, 0 | 145 spikes, 0 | 145 spikes, 0 | 145 spikes, 0 | 145 spikes, 0 | 145 spikes, 0 | 145 spikes, 0 |
| 2022 70 | 148 spikes, 0.1 | 148 spikes, 0 | 202 spikes, 0 | 156 spikes, 0 | 251 spikes, 0 | 65 spikes, 0.1 | 148 spikes, 0 | 148 spikes, 0 | 66 spikes, 0.1 | 148 spikes, 0 | 148 spikes, 0 | 148 spikes, 0 | 148 spikes, 0 | 148 spikes, 0 | 148 spikes, 0 | 148 spikes, 0 | 148 spikes, 0 | 148 spikes, 0 | 148 spikes, 0 |
| 2022 63 | 150 spikes, 0.1 | 150 spikes, 0 | 207 spikes, 0 | 165 spikes, 0 | 259 spikes, 0 | 67 spikes, 0.1 | 150 spikes, 0 | 150 spikes, 0 | 67 spikes, 0.1 | 150 spikes, 0 | 150 spikes, 0 | 150 spikes, 0 | 150 spikes, 0 | 150 spikes, 0 | 150 spikes, 0 | 150 spikes, 0 | 150 spikes, 0 | 150 spikes, 0 | 150 spikes, 0 |
| 2022 56 | 152 spikes, 0.1 | 152 spikes, 0 | 211 spikes, 0 | 170 spikes, 0 | 268 spikes, 0 | 69 spikes, 0.1 | 152 spikes, 0 | 152 spikes, 0 | 69 spikes, 0.1 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 | 152 spikes, 0 |
| 2022 49 | 160 spikes, 0.1 | 160 spikes, 0 | 217 spikes, 0 | 179 spikes, 0 | 276 spikes, 0 | 72 spikes, 0.1 | 160 spikes, 0 | 160 spikes, 0 | 73 spikes, 0.1 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 | 160 spikes, 0 |
| 2022 42 | 161 spikes, 0.1 | 161 spikes, 0 | 220 spikes, 0 | 183 spikes, 0 | 278 spikes, 0 | 73 spikes, 0.1 | 161 spikes, 0 | 161 spikes, 0 | 75 spikes, 0 | 161 spikes, 0 | 161 spikes, 0 | 161 spikes, 0 | 161 spikes, 0 | 161 spikes, 0 | 161 spikes, 0 | 161 spikes, 0 | 161 spikes, 0 | 161 spikes, 0 | 161 spikes, 0 |
| 2022 35 | 166 spikes, 0.1 | 166 spikes, 0 | 221 spikes, 0 | 187 spikes, 0 | 284 spikes, 0 | 73 spikes, 0 | 166 spikes, 0 | 166 spikes, 0 | 75 spikes, 0 | 166 spikes, 0 | 166 spikes, 0 | 166 spikes, 0 | 166 spikes, 0 | 166 spikes, 0 | 166 spikes, 0 | 166 spikes, 0 | 166 spikes, 0 | 166 spikes, 0 | 166 spikes, 0 |
| 2022 28 | 167 spikes, 0.1 | 167 spikes, 0 | 223 spikes, 0 | 190 spikes, 0 | 289 spikes, 0 | 74 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 76 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 | 167 spikes, 0 |
| 2022 21 | 169 spikes, 0.1 | 169 spikes, 0 | 228 spikes, 0 | 194 spikes, 0 | 293 spikes, 0 | 74 spikes, 0 | 169 spikes, 0 | 169 spikes, 0 | 77 spikes, 0 | 169 spikes, 0 | 169 spikes, 0 | 169 spikes, 0 | 169 spikes, 0 | 169 spikes, 0 | 169 spikes, 0 | 169 spikes, 0 | 169 spikes, 0 | 169 spikes, 0 | 169 spikes, 0 |
| 2022 14 | 172 spikes, 0.1 | 172 spikes, 0 | 231 spikes, 0 | 197 spikes, 0 | 298 spikes, 0 | 74 spikes, 0 | 172 spikes, 0 | 172 spikes, 0 | 77 spikes, 0 | 172 spikes, 0 | 172 spikes, 0 | 172 spikes, 0 | 172 spikes, 0 | 172 spikes, 0 | 172 spikes, 0 | 172 spikes, 0 | 172 spikes, 0 | 172 spikes, 0 | 172 spikes, 0 |
| 2022 7 | 174 spikes, 0.1 | 174 spikes, 0 | 234 spikes, 0 | 200 spikes, 0 | 304 spikes, 0 | 75 spikes, 0 | 174 spikes, 0 | 174 spikes, 0 | 78 spikes, 0 | 174 spikes, 0 | 174 spikes, 0 | 174 spikes, 0 | 174 spikes, 0 | 174 spikes, 0 | 174 spikes, 0 | 174 spikes, 0 | 174 spikes, 0 | 174 spikes, 0 | 174 spikes, 0 |

## Market blend

The model at the current settings pooled with the last market prices published by each replay's date (model/src/blend.rs), at each weight on the model; mean log-loss of who qualifies and who wins. Like the model, the markets are conditioned on the field that ran: each venue's prices on the runners are scaled to sum to 1. Elections with market history: 2017 (17 replays), 2022 (21 replays). The best weight by the backtest's rule (among the weights at least as good as the model alone on every such election, the lowest total): **1**. The published weight is config/model.yaml's `market_blend.model_weight`, set by the owner.

| Weight on the model | 2017 qualify | 2017 win | 2022 qualify | 2022 win | Total |
|---|---|---|---|---|---|
| 0 | 0.178 | 0.257 | 0.097 | 0.057 | 0.588 |
| 0.1 | 0.177 | 0.246 | 0.097 | 0.052 | 0.573 |
| 0.2 | 0.177 | 0.234 | 0.097 | 0.048 | 0.556 |
| 0.3 | 0.177 | 0.224 | 0.097 | 0.044 | 0.542 |
| 0.4 | 0.178 | 0.211 | 0.097 | 0.041 | 0.526 |
| 0.5 | 0.178 | 0.199 | 0.097 | 0.038 | 0.511 |
| 0.6 | 0.179 | 0.188 | 0.097 | 0.035 | 0.498 |
| 0.7 | 0.179 | 0.178 | 0.097 | 0.033 | 0.487 |
| 0.8 | 0.180 | 0.170 | 0.097 | 0.031 | 0.477 |
| 0.9 | 0.180 | 0.163 | 0.097 | 0.029 | 0.470 |
| 1 | 0.181 | 0.159 | 0.097 | 0.027 | 0.464 |
