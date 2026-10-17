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
| Model, current | 0.048 | 0.179 | 0.023 | 0.079 | 2.31 | 67.1% | -116.4 | -44.61 | 84.2% |
| Model, fitted | 0.048 | 0.179 | 0.023 | 0.079 | 2.31 | 67.1% | -116.4 | -44.61 | 84.2% |

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
| 2017, current | 0.061 | 0.188 | 0.056 | 0.178 | 2.18 | 67.6% | -19.1 | -10.34 | 85.3% |
| 2022, current | 0.025 | 0.094 | 0.003 | 0.026 | 3.34 | 59.4% | -136.7 | 14.17 | 92.8% |
| 2002, fitted | 0.112 | 0.422 | 0.027 | 0.080 | 2.47 | 62.3% | -24.1 | -65.44 | 81.6% |
| 2007, fitted | 0.003 | 0.018 | 0.025 | 0.083 | 1.62 | 77.5% | 35.5 | 18.76 | 66.5% |
| 2012, fitted | 0.017 | 0.074 | 0.003 | 0.028 | 1.75 | 71.2% | 28.0 | -1.76 | 94.9% |
| 2017, fitted | 0.061 | 0.188 | 0.056 | 0.178 | 2.18 | 67.6% | -19.1 | -10.34 | 85.3% |
| 2022, fitted | 0.025 | 0.094 | 0.003 | 0.026 | 3.34 | 59.4% | -136.7 | 14.17 | 92.8% |

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
| 334, current | 0.040 | 0.185 | 0.027 | 0.090 | 2.80 | 68.8% | -3.3 | -2.67 | 73.4% |
| 304, current | 0.055 | 0.217 | 0.041 | 0.126 | 2.64 | 69.2% | 0.2 | -3.66 | 69.7% |
| 273, current | 0.052 | 0.212 | 0.036 | 0.112 | 2.67 | 66.7% | -0.2 | -3.02 | 72.9% |
| 243, current | 0.051 | 0.213 | 0.033 | 0.106 | 2.58 | 68.5% | 0.2 | -2.88 | 73.3% |
| 212, current | 0.052 | 0.201 | 0.035 | 0.117 | 2.63 | 69.1% | -4.8 | -3.65 | 73.8% |
| 182, current | 0.059 | 0.202 | 0.035 | 0.118 | 2.73 | 72.7% | -2.5 | -3.84 | 77.2% |
| 152, current | 0.059 | 0.194 | 0.034 | 0.115 | 2.79 | 69.1% | -3.5 | -3.82 | 74.7% |
| 122, current | 0.069 | 0.221 | 0.040 | 0.122 | 2.88 | 70.2% | -6.8 | -2.24 | 83.8% |
| 91, current | 0.059 | 0.190 | 0.025 | 0.079 | 2.53 | 71.2% | -8.1 | -0.82 | 86.8% |
| 84, current | 0.063 | 0.203 | 0.023 | 0.077 | 2.57 | 67.8% | -9.6 | -0.73 | 89.0% |
| 77, current | 0.052 | 0.182 | 0.019 | 0.069 | 2.51 | 67.8% | -13.7 | -1.07 | 90.1% |
| 70, current | 0.048 | 0.171 | 0.015 | 0.058 | 2.46 | 68.3% | -12.5 | -1.39 | 90.4% |
| 63, current | 0.047 | 0.168 | 0.014 | 0.055 | 2.31 | 67.2% | -12.1 | -1.35 | 92.1% |
| 56, current | 0.042 | 0.162 | 0.011 | 0.047 | 2.18 | 68.9% | -9.9 | -2.13 | 90.1% |
| 49, current | 0.043 | 0.165 | 0.012 | 0.047 | 2.28 | 63.9% | -11.0 | -2.51 | 90.7% |
| 42, current | 0.046 | 0.175 | 0.016 | 0.055 | 2.06 | 62.3% | -7.2 | -1.89 | 90.4% |
| 35, current | 0.040 | 0.161 | 0.017 | 0.058 | 2.04 | 60.7% | -7.2 | -1.36 | 90.6% |
| 28, current | 0.038 | 0.158 | 0.016 | 0.056 | 1.86 | 60.7% | -7.8 | -0.61 | 89.6% |
| 21, current | 0.033 | 0.132 | 0.015 | 0.052 | 1.72 | 65.6% | -4.2 | -1.12 | 89.6% |
| 14, current | 0.034 | 0.123 | 0.014 | 0.052 | 1.51 | 65.6% | 0.1 | -1.24 | 90.2% |
| 7, current | 0.037 | 0.123 | 0.012 | 0.050 | 1.21 | 67.2% | 7.5 | -2.60 | 90.0% |
| 334, fitted | 0.040 | 0.185 | 0.027 | 0.090 | 2.80 | 68.8% | -3.3 | -2.67 | 73.4% |
| 304, fitted | 0.055 | 0.217 | 0.041 | 0.126 | 2.64 | 69.2% | 0.2 | -3.66 | 69.7% |
| 273, fitted | 0.052 | 0.212 | 0.036 | 0.112 | 2.67 | 66.7% | -0.2 | -3.02 | 72.9% |
| 243, fitted | 0.051 | 0.213 | 0.033 | 0.106 | 2.58 | 68.5% | 0.2 | -2.88 | 73.3% |
| 212, fitted | 0.052 | 0.201 | 0.035 | 0.117 | 2.63 | 69.1% | -4.8 | -3.65 | 73.8% |
| 182, fitted | 0.059 | 0.202 | 0.035 | 0.118 | 2.73 | 72.7% | -2.5 | -3.84 | 77.2% |
| 152, fitted | 0.059 | 0.194 | 0.034 | 0.115 | 2.79 | 69.1% | -3.5 | -3.82 | 74.7% |
| 122, fitted | 0.069 | 0.221 | 0.040 | 0.122 | 2.88 | 70.2% | -6.8 | -2.24 | 83.8% |
| 91, fitted | 0.059 | 0.190 | 0.025 | 0.079 | 2.53 | 71.2% | -8.1 | -0.82 | 86.8% |
| 84, fitted | 0.063 | 0.203 | 0.023 | 0.077 | 2.57 | 67.8% | -9.6 | -0.73 | 89.0% |
| 77, fitted | 0.052 | 0.182 | 0.019 | 0.069 | 2.51 | 67.8% | -13.7 | -1.07 | 90.1% |
| 70, fitted | 0.048 | 0.171 | 0.015 | 0.058 | 2.46 | 68.3% | -12.5 | -1.39 | 90.4% |
| 63, fitted | 0.047 | 0.168 | 0.014 | 0.055 | 2.31 | 67.2% | -12.1 | -1.35 | 92.1% |
| 56, fitted | 0.042 | 0.162 | 0.011 | 0.047 | 2.18 | 68.9% | -9.9 | -2.13 | 90.1% |
| 49, fitted | 0.043 | 0.165 | 0.012 | 0.047 | 2.28 | 63.9% | -11.0 | -2.51 | 90.7% |
| 42, fitted | 0.046 | 0.175 | 0.016 | 0.055 | 2.06 | 62.3% | -7.2 | -1.89 | 90.4% |
| 35, fitted | 0.040 | 0.161 | 0.017 | 0.058 | 2.04 | 60.7% | -7.2 | -1.36 | 90.6% |
| 28, fitted | 0.038 | 0.158 | 0.016 | 0.056 | 1.86 | 60.7% | -7.8 | -0.61 | 89.6% |
| 21, fitted | 0.033 | 0.132 | 0.015 | 0.052 | 1.72 | 65.6% | -4.2 | -1.12 | 89.6% |
| 14, fitted | 0.034 | 0.123 | 0.014 | 0.052 | 1.51 | 65.6% | 0.1 | -1.24 | 90.2% |
| 7, fitted | 0.037 | 0.123 | 0.012 | 0.050 | 1.21 | 67.2% | 7.5 | -2.60 | 90.0% |

## Reliability

Qualification and win events pooled over every replay.

| Forecast | Events (current) | Mean forecast | Observed | Events (fitted) | Mean forecast | Observed |
|---|---|---|---|---|---|---|
| 0–10% | 2047 | 0.6% | 1.3% | 2047 | 0.6% | 1.3% |
| 10–20% | 92 | 13.8% | 7.6% | 92 | 13.8% | 7.6% |
| 20–30% | 44 | 24.9% | 11.4% | 44 | 24.9% | 11.4% |
| 30–40% | 35 | 34.5% | 25.7% | 35 | 34.5% | 25.7% |
| 40–50% | 46 | 44.7% | 43.5% | 46 | 44.7% | 43.5% |
| 50–60% | 43 | 54.5% | 46.5% | 43 | 54.5% | 46.5% |
| 60–70% | 23 | 65.1% | 91.3% | 23 | 65.1% | 91.3% |
| 70–80% | 30 | 74.8% | 86.7% | 30 | 74.8% | 86.7% |
| 80–90% | 75 | 85.2% | 89.3% | 75 | 85.2% | 89.3% |
| 90–100% | 127 | 95.4% | 89.8% | 127 | 95.4% | 89.8% |

## Fit

The rule, in each stage: among the grid points that score at least as well as the stage's starting setting on every source of evidence (each election's replays, and the final-week polls of other elections: none), take the one with the highest total log score; if none does, keep the starting setting. The first stage starts from config/model.yaml and each later one from the setting the previous one chose; the three stages repeat until a pass changes nothing (1 passes here, at most 4). Every stage keeps or improves every source, so the fitted setting scores at least as well as the current one on all of them. Round 1 stages score contenders' shares; the round 2 stage scores the realised runoff share given the actual matchup. Final-week polls are taken as election-eve polls, with no horizon error.


### Pass 1: Round 1 horizon error and tails

Evidence: share log score of the contenders. Starts from horizon_bloc_sd 0, horizon_candidate_sd 0.2, student_t_df 3.

| Evidence | Start | Chosen | Best on this source alone |
|---|---|---|---|
| 2002 replays | -24.07 | -24.07 | -16.96 (horizon_bloc_sd 0.1, horizon_candidate_sd 0.2, student_t_df 10) |
| 2007 replays | 35.47 | 35.47 | 37.98 (horizon_bloc_sd 0, horizon_candidate_sd 0.02, student_t_df 2.5) |
| 2012 replays | 28.03 | 28.03 | 33.64 (horizon_bloc_sd 0, horizon_candidate_sd 0, student_t_df 2.5) |
| 2017 replays | -19.13 | -19.13 | -11.18 (horizon_bloc_sd 0.04, horizon_candidate_sd 0.4, student_t_df 10) |
| 2022 replays | -136.68 | -136.68 | -89.81 (horizon_bloc_sd 0.27, horizon_candidate_sd 0.27, student_t_df 10) |
| 334-day replays, every election (not used) | -3.29 | -3.29 | -2.38 (horizon_bloc_sd 0, horizon_candidate_sd 0.2, student_t_df 10) |
| 304-day replays, every election (not used) | 0.24 | 0.24 | 0.63 (horizon_bloc_sd 0, horizon_candidate_sd 0.2, student_t_df 10) |
| 273-day replays, every election (not used) | -0.21 | -0.21 | -0.02 (horizon_bloc_sd 0, horizon_candidate_sd 0.14, student_t_df 10) |
| 243-day replays, every election (not used) | 0.17 | 0.17 | 0.37 (horizon_bloc_sd 0, horizon_candidate_sd 0.2, student_t_df 10) |
| 212-day replays, every election (not used) | -4.83 | -4.83 | -3.86 (horizon_bloc_sd 0.1, horizon_candidate_sd 0.27, student_t_df 2.5) |
| 182-day replays, every election (not used) | -2.54 | -2.54 | -2.11 (horizon_bloc_sd 0.02, horizon_candidate_sd 0.14, student_t_df 10) |
| 152-day replays, every election (not used) | -3.50 | -3.50 | -3.17 (horizon_bloc_sd 0, horizon_candidate_sd 0.2, student_t_df 5) |
| 122-day replays, every election (not used) | -6.84 | -6.84 | -6.02 (horizon_bloc_sd 0.1, horizon_candidate_sd 0.33, student_t_df 2.5) |
| 91-day replays, every election (not used) | -8.08 | -8.08 | -7.00 (horizon_bloc_sd 0.1, horizon_candidate_sd 0.4, student_t_df 2.5) |
| 84-day replays, every election (not used) | -9.59 | -9.59 | -8.30 (horizon_bloc_sd 0.14, horizon_candidate_sd 0.33, student_t_df 3) |
| 77-day replays, every election (not used) | -13.65 | -13.65 | -10.91 (horizon_bloc_sd 0.2, horizon_candidate_sd 0.4, student_t_df 3) |
| 70-day replays, every election (not used) | -12.53 | -12.53 | -10.30 (horizon_bloc_sd 0.2, horizon_candidate_sd 0.33, student_t_df 4) |
| 63-day replays, every election (not used) | -12.12 | -12.12 | -9.52 (horizon_bloc_sd 0.14, horizon_candidate_sd 0.4, student_t_df 3) |
| 56-day replays, every election (not used) | -9.90 | -9.90 | -7.83 (horizon_bloc_sd 0.07, horizon_candidate_sd 0.4, student_t_df 3) |
| 49-day replays, every election (not used) | -10.96 | -10.96 | -8.64 (horizon_bloc_sd 0.14, horizon_candidate_sd 0.4, student_t_df 3) |
| 42-day replays, every election (not used) | -7.21 | -7.21 | -5.44 (horizon_bloc_sd 0.14, horizon_candidate_sd 0.4, student_t_df 5) |
| 35-day replays, every election (not used) | -7.18 | -7.18 | -5.36 (horizon_bloc_sd 0.2, horizon_candidate_sd 0.4, student_t_df 3) |
| 28-day replays, every election (not used) | -7.77 | -7.77 | -5.42 (horizon_bloc_sd 0.27, horizon_candidate_sd 0.4, student_t_df 3) |
| 21-day replays, every election (not used) | -4.19 | -4.19 | -2.39 (horizon_bloc_sd 0.2, horizon_candidate_sd 0.33, student_t_df 5) |
| 14-day replays, every election (not used) | 0.07 | 0.07 | 0.76 (horizon_bloc_sd 0.14, horizon_candidate_sd 0.2, student_t_df 5) |
| 7-day replays, every election (not used) | 7.54 | 7.54 | 7.86 (horizon_bloc_sd 0, horizon_candidate_sd 0.4, student_t_df 2.5) |
| Total | -116.37 | -116.37 | -103.32 (horizon_bloc_sd 0.04, horizon_candidate_sd 0.4, student_t_df 3) |

Total log score by `horizon_bloc_sd` (rows) and `horizon_candidate_sd` (columns). `*` marks the points that score at least as well as the start on every source; bold is the chosen one.

student_t_df = 2.5:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -159.2 | -158.1 | -156.5 | -151.6 | -145.0 | -137.5 | -127.2 | -117.7 | -112.4 | -107.7 |
| 0.02 | -157.5 | -156.6 | -154.8 | -150.1 | -143.9 | -136.5 | -126.4 | -117.1 | -111.9 | -107.3 |
| 0.04 | -154.6 | -154.0 | -152.3 | -148.1 | -142.2 | -135.2 | -125.6 | -116.8 | -111.7 | -107.3 |
| 0.07 | -145.8 | -145.2 | -143.8 | -140.3 | -135.4 | -129.3 | -120.4 | -112.5 | -107.9 | -103.8 |
| 0.1 | -140.5 | -140.1 | -139.1 | -136.3 | -132.1 | -126.9 | -118.8 | -112.0 | -108.1 | -104.4 |
| 0.14 | -138.1 | -137.8 | -137.0 | -134.8 | -131.9 | -127.9 | -121.6 | -116.1 | -112.9 | -109.8 |
| 0.2 | -138.3 | -138.1 | -137.6 | -136.1 | -134.1 | -131.1 | -126.9 | -122.9 | -120.6 | -118.9 |
| 0.27 | -145.5 | -145.4 | -145.2 | -144.3 | -143.3 | -141.5 | -138.4 | -135.8 | -134.7 | -134.2 |

student_t_df = 3:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -147.1 | -146.1 | -144.0 | -139.3 | -133.7 | -126.0 | **-116.4*** | -108.9 | -105.7 | -103.8 |
| 0.02 | -146.4 | -145.5 | -143.3 | -138.8 | -133.3 | -125.8 | -116.4 | -109.0 | -105.9 | -104.0 |
| 0.04 | -143.2 | -142.4 | -140.5 | -136.1 | -130.9 | -123.8 | -115.0 | -108.0 | -105.1 | -103.3 |
| 0.07 | -137.5 | -136.9 | -135.5 | -132.2 | -127.4 | -121.4 | -113.9 | -107.7 | -105.2 | -103.6 |
| 0.1 | -135.0 | -134.6 | -133.4 | -130.5 | -126.7 | -121.7 | -115.0 | -109.9 | -107.9 | -106.5 |
| 0.14 | -134.1 | -133.8 | -132.9 | -130.8 | -128.0 | -124.1 | -119.2 | -115.2 | -113.7 | -112.9 |
| 0.2 | -139.7 | -139.5 | -139.0 | -137.7 | -135.9 | -133.6 | -130.8 | -128.5 | -127.8 | -127.9 |
| 0.27 | -154.9 | -154.7 | -154.4 | -153.7 | -152.6 | -151.5 | -150.2 | -149.2 | -149.2 | -150.4 |

student_t_df = 4:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -141.1 | -140.2 | -138.4 | -133.4 | -127.9 | -120.4 | -111.9 | -106.6 | -105.0 | -105.9 |
| 0.02 | -141.0 | -140.2 | -138.6 | -133.6 | -128.2 | -120.9 | -112.5 | -107.2 | -105.7 | -106.6 |
| 0.04 | -139.5 | -139.0 | -137.4 | -133.2 | -128.4 | -121.7 | -113.6 | -108.6 | -107.2 | -108.2 |
| 0.07 | -136.6 | -136.1 | -134.8 | -131.6 | -127.6 | -121.5 | -114.5 | -110.3 | -109.2 | -110.4 |
| 0.1 | -134.6 | -134.2 | -133.2 | -130.7 | -127.2 | -122.1 | -116.7 | -113.3 | -112.7 | -113.9 |
| 0.14 | -134.8 | -134.5 | -133.8 | -132.1 | -129.7 | -126.4 | -122.8 | -120.6 | -120.5 | -122.1 |
| 0.2 | -144.2 | -144.1 | -143.7 | -142.7 | -141.3 | -139.5 | -137.4 | -136.5 | -137.7 | -140.0 |
| 0.27 | -167.0 | -166.9 | -166.7 | -166.2 | -165.5 | -164.5 | -163.6 | -164.1 | -166.2 | -169.0 |

student_t_df = 5:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -143.7 | -142.9 | -140.1 | -134.5 | -127.9 | -120.8 | -113.0 | -109.1 | -108.7 | -110.9 |
| 0.02 | -142.4 | -141.6 | -139.1 | -133.6 | -127.3 | -120.3 | -112.8 | -108.9 | -108.6 | -110.8 |
| 0.04 | -140.4 | -139.6 | -137.3 | -132.2 | -126.8 | -120.2 | -113.2 | -109.7 | -109.4 | -111.8 |
| 0.07 | -134.9 | -134.3 | -132.6 | -129.1 | -124.9 | -119.1 | -113.3 | -110.4 | -110.4 | -113.0 |
| 0.1 | -131.2 | -130.8 | -129.6 | -126.9 | -123.5 | -119.1 | -114.5 | -112.0 | -112.4 | -115.2 |
| 0.14 | -133.8 | -133.6 | -132.8 | -130.9 | -128.5 | -125.3 | -122.0 | -120.6 | -121.6 | -124.7 |
| 0.2 | -146.0 | -145.8 | -145.4 | -144.2 | -142.8 | -141.0 | -139.6 | -140.0 | -141.8 | -145.2 |
| 0.27 | -170.6 | -170.5 | -170.3 | -169.7 | -169.0 | -168.2 | -168.5 | -170.2 | -173.0 | -176.9 |

student_t_df = 10:

| horizon_bloc_sd \ horizon_candidate_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 | 0.27 | 0.33 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | -154.8 | -153.8 | -151.2 | -145.4 | -138.6 | -131.2 | -123.5 | -120.3 | -120.5 | -123.1 |
| 0.02 | -151.6 | -150.7 | -148.4 | -142.8 | -136.2 | -128.9 | -121.4 | -118.3 | -118.5 | -121.1 |
| 0.04 | -145.4 | -144.8 | -142.8 | -137.6 | -131.8 | -125.0 | -117.9 | -115.1 | -115.3 | -118.0 |
| 0.07 | -137.2 | -136.6 | -135.1 | -131.2 | -126.8 | -121.0 | -114.8 | -112.6 | -113.0 | -116.0 |
| 0.1 | -136.3 | -135.9 | -134.9 | -132.0 | -128.5 | -123.6 | -118.5 | -117.2 | -117.9 | -121.1 |
| 0.14 | -137.7 | -137.5 | -136.8 | -134.9 | -132.5 | -129.0 | -125.9 | -125.6 | -126.9 | -130.4 |
| 0.2 | -151.8 | -151.6 | -151.2 | -150.3 | -149.0 | -147.2 | -146.0 | -146.7 | -148.7 | -152.7 |
| 0.27 | -177.8 | -177.7 | -177.5 | -177.1 | -176.5 | -175.9 | -175.9 | -177.6 | -180.6 | -184.9 |

### Pass 1: Round 1 election-day error

Evidence: share log score of the contenders. Starts from bloc_sd 0.18, candidate_sd 0.2.

| Evidence | Start | Chosen | Best on this source alone |
|---|---|---|---|
| 2002 replays | -24.07 | -24.07 | -19.68 (bloc_sd 0.33, candidate_sd 0.4) |
| 2007 replays | 35.47 | 35.47 | 38.46 (bloc_sd 0.1, candidate_sd 0.03) |
| 2012 replays | 28.03 | 28.03 | 33.43 (bloc_sd 0.08, candidate_sd 0.03) |
| 2017 replays | -19.13 | -19.13 | -16.14 (bloc_sd 0.18, candidate_sd 0.4) |
| 2022 replays | -136.68 | -136.68 | -91.73 (bloc_sd 0.4, candidate_sd 0.4) |
| 334-day replays, every election (not used) | -3.29 | -3.29 | -3.06 (bloc_sd 0.22, candidate_sd 0.4) |
| 304-day replays, every election (not used) | 0.24 | 0.24 | 0.37 (bloc_sd 0.15, candidate_sd 0.4) |
| 273-day replays, every election (not used) | -0.21 | -0.21 | -0.20 (bloc_sd 0.18, candidate_sd 0.16) |
| 243-day replays, every election (not used) | 0.17 | 0.17 | 0.30 (bloc_sd 0.15, candidate_sd 0.3) |
| 212-day replays, every election (not used) | -4.83 | -4.83 | -4.04 (bloc_sd 0.27, candidate_sd 0.4) |
| 182-day replays, every election (not used) | -2.54 | -2.54 | -2.54 (bloc_sd 0.18, candidate_sd 0.16) |
| 152-day replays, every election (not used) | -3.50 | -3.50 | -3.36 (bloc_sd 0.15, candidate_sd 0.4) |
| 122-day replays, every election (not used) | -6.84 | -6.84 | -6.28 (bloc_sd 0.27, candidate_sd 0.3) |
| 91-day replays, every election (not used) | -8.08 | -8.08 | -7.10 (bloc_sd 0.27, candidate_sd 0.4) |
| 84-day replays, every election (not used) | -9.59 | -9.59 | -8.32 (bloc_sd 0.27, candidate_sd 0.4) |
| 77-day replays, every election (not used) | -13.65 | -13.65 | -11.08 (bloc_sd 0.4, candidate_sd 0.4) |
| 70-day replays, every election (not used) | -12.53 | -12.53 | -10.52 (bloc_sd 0.4, candidate_sd 0.4) |
| 63-day replays, every election (not used) | -12.12 | -12.12 | -9.90 (bloc_sd 0.27, candidate_sd 0.4) |
| 56-day replays, every election (not used) | -9.90 | -9.90 | -8.57 (bloc_sd 0.27, candidate_sd 0.4) |
| 49-day replays, every election (not used) | -10.96 | -10.96 | -9.23 (bloc_sd 0.27, candidate_sd 0.4) |
| 42-day replays, every election (not used) | -7.21 | -7.21 | -5.95 (bloc_sd 0.18, candidate_sd 0.4) |
| 35-day replays, every election (not used) | -7.18 | -7.18 | -5.36 (bloc_sd 0.27, candidate_sd 0.4) |
| 28-day replays, every election (not used) | -7.77 | -7.77 | -5.38 (bloc_sd 0.33, candidate_sd 0.4) |
| 21-day replays, every election (not used) | -4.19 | -4.19 | -2.53 (bloc_sd 0.27, candidate_sd 0.4) |
| 14-day replays, every election (not used) | 0.07 | 0.07 | 0.59 (bloc_sd 0.27, candidate_sd 0.25) |
| 7-day replays, every election (not used) | 7.54 | 7.54 | 7.99 (bloc_sd 0.1, candidate_sd 0.4) |
| Total | -116.37 | -116.37 | -99.70 (bloc_sd 0.27, candidate_sd 0.4) |

Total log score by `bloc_sd` (rows) and `candidate_sd` (columns). `*` marks the points that score at least as well as the start on every source; bold is the chosen one.

| bloc_sd \ candidate_sd | 0.03 | 0.05 | 0.07 | 0.1 | 0.13 | 0.16 | 0.2 | 0.25 | 0.3 | 0.4 |
|---|---|---|---|---|---|---|---|---|---|---|
| 0.04 | -670.6 | -668.1 | -665.9 | -662.0 | -657.8 | -652.6 | -646.2 | -638.9 | -632.1 | -622.6 |
| 0.06 | -581.9 | -580.3 | -578.7 | -575.1 | -570.9 | -566.0 | -560.5 | -554.1 | -547.9 | -538.3 |
| 0.08 | -394.0 | -392.6 | -390.8 | -387.4 | -383.6 | -379.8 | -375.2 | -369.8 | -364.2 | -355.9 |
| 0.1 | -179.9 | -178.8 | -177.3 | -174.3 | -171.1 | -167.9 | -163.6 | -158.0 | -152.7 | -144.7 |
| 0.12 | -169.5 | -168.6 | -167.2 | -164.4 | -161.5 | -158.8 | -154.9 | -149.7 | -145.0 | -137.8 |
| 0.15 | -141.9 | -141.2 | -140.1 | -137.9 | -135.5 | -133.1 | -129.5 | -124.7 | -120.8 | -114.4 |
| 0.18 | -126.6 | -126.1 | -125.2 | -123.4 | -121.3 | -119.3 | **-116.4*** | -112.6 | -109.4 | -104.1 |
| 0.22 | -119.2 | -118.7 | -118.1 | -116.8 | -115.1 | -113.5 | -111.2 | -108.2 | -105.6 | -101.4 |
| 0.27 | -112.6 | -112.3 | -111.9 | -111.0 | -109.9 | -108.8 | -107.0 | -104.6 | -102.6 | -99.7 |
| 0.33 | -115.2 | -115.0 | -114.7 | -114.0 | -113.4 | -112.7 | -111.4 | -109.9 | -108.8 | -107.4 |
| 0.4 | -121.7 | -121.5 | -121.4 | -121.0 | -120.7 | -120.4 | -119.8 | -119.2 | -118.7 | -118.4 |

### Pass 1: Round 2 errors

Evidence: log score of the realised runoff share. Starts from error_sd 0.12, horizon_error_sd 0.

| Evidence | Start | Chosen | Best on this source alone |
|---|---|---|---|
| 2002 replays | -65.44 | -65.44 | -43.92 (error_sd 0.4, horizon_error_sd 0.2) |
| 2007 replays | 18.76 | 18.76 | 25.09 (error_sd 0.02, horizon_error_sd 0) |
| 2012 replays | -1.76 | -1.76 | 2.18 (error_sd 0.12, horizon_error_sd 0.1) |
| 2017 replays | -10.34 | -10.34 | -5.84 (error_sd 0.15, horizon_error_sd 0.14) |
| 2022 replays | 14.17 | 14.17 | 14.68 (error_sd 0.08, horizon_error_sd 0) |
| 334-day replays, every election (not used) | -2.67 | -2.67 | -2.62 (error_sd 0.18, horizon_error_sd 0.02) |
| 304-day replays, every election (not used) | -3.66 | -3.66 | -3.26 (error_sd 0.27, horizon_error_sd 0.04) |
| 273-day replays, every election (not used) | -3.02 | -3.02 | -2.88 (error_sd 0.05, horizon_error_sd 0.07) |
| 243-day replays, every election (not used) | -2.88 | -2.88 | -2.80 (error_sd 0.02, horizon_error_sd 0.07) |
| 212-day replays, every election (not used) | -3.65 | -3.65 | -3.35 (error_sd 0.02, horizon_error_sd 0.1) |
| 182-day replays, every election (not used) | -3.84 | -3.84 | -3.19 (error_sd 0.18, horizon_error_sd 0.1) |
| 152-day replays, every election (not used) | -3.82 | -3.82 | -3.28 (error_sd 0.27, horizon_error_sd 0.04) |
| 122-day replays, every election (not used) | -2.24 | -2.24 | -1.78 (error_sd 0.08, horizon_error_sd 0.1) |
| 91-day replays, every election (not used) | -0.82 | -0.82 | -0.79 (error_sd 0.12, horizon_error_sd 0.04) |
| 84-day replays, every election (not used) | -0.73 | -0.73 | -0.72 (error_sd 0.05, horizon_error_sd 0.07) |
| 77-day replays, every election (not used) | -1.07 | -1.07 | -1.04 (error_sd 0.08, horizon_error_sd 0.07) |
| 70-day replays, every election (not used) | -1.39 | -1.39 | -1.34 (error_sd 0.15, horizon_error_sd 0.02) |
| 63-day replays, every election (not used) | -1.35 | -1.35 | -1.24 (error_sd 0.02, horizon_error_sd 0.1) |
| 56-day replays, every election (not used) | -2.13 | -2.13 | -1.76 (error_sd 0.12, horizon_error_sd 0.1) |
| 49-day replays, every election (not used) | -2.51 | -2.51 | -2.01 (error_sd 0.08, horizon_error_sd 0.14) |
| 42-day replays, every election (not used) | -1.89 | -1.89 | -1.66 (error_sd 0.18, horizon_error_sd 0) |
| 35-day replays, every election (not used) | -1.36 | -1.36 | -1.28 (error_sd 0.15, horizon_error_sd 0.02) |
| 28-day replays, every election (not used) | -0.61 | -0.61 | -0.60 (error_sd 0.1, horizon_error_sd 0.04) |
| 21-day replays, every election (not used) | -1.12 | -1.12 | -1.11 (error_sd 0.12, horizon_error_sd 0.04) |
| 14-day replays, every election (not used) | -1.24 | -1.24 | -1.22 (error_sd 0.02, horizon_error_sd 0.14) |
| 7-day replays, every election (not used) | -2.60 | -2.60 | -2.07 (error_sd 0.18, horizon_error_sd 0.1) |
| Total | -44.61 | -44.61 | -41.21 (error_sd 0.12, horizon_error_sd 0.07) |

Total log score by `error_sd` (rows) and `horizon_error_sd` (columns). `*` marks the points that score at least as well as the start on every source; bold is the chosen one.

| error_sd \ horizon_error_sd | 0 | 0.02 | 0.04 | 0.07 | 0.1 | 0.14 | 0.2 |
|---|---|---|---|---|---|---|---|
| 0.02 | -70.1 | -65.9 | -57.1 | -46.7 | -42.7 | -43.9 | -51.2 |
| 0.03 | -67.5 | -63.7 | -55.7 | -46.1 | -42.6 | -43.9 | -51.2 |
| 0.05 | -61.0 | -58.1 | -52.1 | -44.7 | -42.2 | -44.0 | -51.4 |
| 0.08 | -52.0 | -50.3 | -46.9 | -42.7 | -41.7 | -44.3 | -51.9 |
| 0.1 | -47.6 | -46.5 | -44.3 | -41.7 | -41.6 | -44.6 | -52.3 |
| 0.12 | **-44.6*** | -44.0 | -42.6 | -41.2 | -41.8 | -45.1 | -52.8 |
| 0.15 | -42.2 | -42.0 | -41.4 | -41.2 | -42.5 | -46.1 | -53.8 |
| 0.18 | -41.7 | -41.6 | -41.5 | -42.0 | -43.6 | -47.4 | -54.9 |
| 0.22 | -42.6 | -42.7 | -42.9 | -43.8 | -45.7 | -49.4 | -56.7 |
| 0.27 | -45.3 | -45.4 | -45.8 | -46.9 | -48.8 | -52.3 | -59.2 |
| 0.33 | -49.4 | -49.5 | -50.0 | -51.1 | -52.9 | -56.2 | -62.5 |
| 0.4 | -54.7 | -54.8 | -55.2 | -56.3 | -58.0 | -60.9 | -66.6 |

## Leave one election out

The whole fit rerun without one election's replays, then that election's replays scored at the setting it gives, and at the current one.

| Held out | Passes | bloc_sd | candidate_sd | horizon_bloc_sd | horizon_candidate_sd | student_t_df | error_sd | horizon_error_sd | Round 1 log score (chosen without / current) | Round 2 log score (chosen without / current) |
|---|---|---|---|---|---|---|---|---|---|---|
| 2002 | 2 | 0.18 | 0.2 | 0.02 | 0.33 | 2.5 | 0.12 | 0 | -24.47 / -24.07 | -66.72 / -65.44 |
| 2007 | 1 | 0.18 | 0.2 | 0 | 0.2 | 3 | 0.12 | 0 | 35.47 / 35.47 | 18.76 / 18.76 |
| 2012 | 1 | 0.18 | 0.2 | 0 | 0.2 | 3 | 0.12 | 0 | 28.03 / 28.03 | -1.76 / -1.76 |
| 2017 | 1 | 0.18 | 0.2 | 0 | 0.2 | 3 | 0.12 | 0 | -19.13 / -19.13 | -10.34 / -10.34 |
| 2022 | 1 | 0.18 | 0.2 | 0 | 0.2 | 3 | 0.12 | 0 | -136.68 / -136.68 | 14.17 / 14.17 |

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
| 2017 | 304 | 2016-06-23 | 7 | 0 | 0.007 | 2 | melenchon, macron, fillon, le_pen | hamon |
| 2017 | 273 | 2016-07-24 | 8 | 0 | 0.007 | 2 | melenchon, macron, fillon, le_pen | hamon |
| 2017 | 243 | 2016-08-23 | 8 | 0 | 0.007 | 2 | melenchon, macron, fillon, le_pen | hamon |
| 2017 | 212 | 2016-09-23 | 13 | 0 | 0.007 | 2 | melenchon, macron, fillon, le_pen | hamon |
| 2017 | 182 | 2016-10-23 | 16 | 0 | 0.007 | 2 | melenchon, macron, fillon, le_pen | hamon |
| 2017 | 152 | 2016-11-22 | 19 | 0 | 0.012 | 2 | melenchon, macron, fillon, le_pen | hamon |
| 2017 | 122 | 2016-12-22 | 27 | 0 | 0.02 | 2 | melenchon, macron, fillon, le_pen | – |
| 2017 | 91 | 2017-01-22 | 32 | 0 | 0.02 | 2 | melenchon, macron, fillon, le_pen | – |
| 2017 | 84 | 2017-01-29 | 33 | 0 | 0.02 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 77 | 2017-02-05 | 36 | 3 | 0.03 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 70 | 2017-02-12 | 43 | 9 | 0.03 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 63 | 2017-02-19 | 47 | 16 | 0.03 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 56 | 2017-02-26 | 56 | 22 | 0.03 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 49 | 2017-03-05 | 64 | 30 | 0.03 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 42 | 2017-03-12 | 72 | 37 | 0.03 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 35 | 2017-03-19 | 81 | 45 | 0.03 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 28 | 2017-03-26 | 89 | 51 | 0.03 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 21 | 2017-04-02 | 97 | 62 | 0.02 | 2 | melenchon, hamon, macron, fillon, le_pen | – |
| 2017 | 14 | 2017-04-09 | 108 | 69 | 0.03 | 2 | melenchon, macron, fillon, le_pen | – |
| 2017 | 7 | 2017-04-16 | 120 | 83 | 0.03 | 2 | melenchon, macron, fillon, le_pen | – |
| 2022 | 334 | 2021-05-11 | 2 | 0 | 0.002 | 2 | melenchon, macron, pecresse, le_pen | zemmour |
| 2022 | 304 | 2021-06-10 | 8 | 0 | 0.012 | 1.5 | melenchon, macron, le_pen | – |
| 2022 | 273 | 2021-07-11 | 14 | 0 | 0.012 | 1.5 | macron, pecresse, le_pen | – |
| 2022 | 243 | 2021-08-10 | 14 | 0 | 0.012 | 1.5 | macron, pecresse, le_pen | – |
| 2022 | 212 | 2021-09-10 | 18 | 0 | 0.004 | 1.5 | melenchon, macron, pecresse, le_pen | – |
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

Actual matchup: current 65.1% (winner given matchup 70.8%), fitted 65.1% (winner given matchup 70.8%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 24.7% (19.4%–30.1%) | 71.8% / 71.8% ✓ | 50.0% / 50.0% ✓ |
| le_pen | 21.3% | 28.6% (23.3%–33.9%) | 92.7% / 92.7% ✓ | 24.0% / 24.0% |
| fillon | 20.0% | 19.5% (15.2%–23.8%) | 15.8% / 15.8% | 14.2% / 14.2% |
| melenchon | 19.6% | 17.8% (10.8%–25.7%) | 19.6% / 19.6% | 11.7% / 11.7% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 5.8% (4.3%–7.4%) | 0.1% / 0.1% | 0.1% / 0.1% |
| lassalle | 1.2% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.5% (0.7%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.4% (0.7%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.7% (0.2%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 304 days out (2016-06-23)

Actual matchup: current 15.2% (winner given matchup 57.1%), fitted 15.2% (winner given matchup 57.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 17.8% (13.4%–22.4%) | 16.9% / 16.9% ✓ | 9.5% / 9.5% ✓ |
| le_pen | 21.3% | 30.7% (24.5%–37.2%) | 96.1% / 96.1% ✓ | 21.4% / 21.4% |
| fillon | 20.0% | 22.2% (16.4%–28.4%) | 59.5% / 59.5% | 56.9% / 56.9% |
| melenchon | 19.6% | 19.0% (14.0%–24.4%) | 27.5% / 27.5% | 12.2% / 12.2% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 4.9% (2.8%–7.1%) | 0.1% / 0.1% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.8% (0.2%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.5% (0.7%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 1.0% (0.3%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.4% (0.7%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 273 days out (2016-07-24)

Actual matchup: current 21.1% (winner given matchup 55.9%), fitted 21.1% (winner given matchup 55.9%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 18.3% (13.5%–23.4%) | 22.8% / 22.8% ✓ | 12.6% / 12.6% ✓ |
| le_pen | 21.3% | 31.4% (25.0%–38.1%) | 96.7% / 96.7% ✓ | 23.6% / 23.6% |
| fillon | 20.0% | 21.7% (15.8%–27.9%) | 55.1% / 55.1% | 52.6% / 52.6% |
| melenchon | 19.6% | 18.4% (12.9%–24.3%) | 25.3% / 25.3% | 11.2% / 11.2% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 4.9% (2.8%–7.2%) | 0.1% / 0.1% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.8% (0.2%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.5% (0.7%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 1.0% (0.3%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.4% (0.7%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 243 days out (2016-08-23)

Actual matchup: current 22.6% (winner given matchup 56.2%), fitted 22.6% (winner given matchup 56.2%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 18.6% (13.6%–24.0%) | 24.8% / 24.8% ✓ | 13.8% / 13.8% ✓ |
| le_pen | 21.3% | 30.9% (24.3%–37.7%) | 95.5% / 95.5% ✓ | 23.5% / 23.5% |
| fillon | 20.0% | 21.6% (15.3%–28.4%) | 51.4% / 51.4% | 48.8% / 48.8% |
| melenchon | 19.6% | 18.6% (12.1%–25.9%) | 28.3% / 28.3% | 13.8% / 13.8% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 5.0% (2.8%–7.4%) | 0.1% / 0.1% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.8% (0.2%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.5% (0.8%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 1.0% (0.3%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.4% (0.7%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 212 days out (2016-09-23)

Actual matchup: current 13.6% (winner given matchup 47.3%), fitted 13.6% (winner given matchup 47.3%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 17.0% (13.0%–21.2%) | 14.3% / 14.3% ✓ | 6.8% / 6.8% ✓ |
| le_pen | 21.3% | 33.1% (26.6%–39.8%) | 98.2% / 98.2% ✓ | 40.0% / 40.0% |
| fillon | 20.0% | 18.7% (13.6%–24.0%) | 29.8% / 29.8% | 28.4% / 28.4% |
| melenchon | 19.6% | 21.3% (15.9%–27.1%) | 57.8% / 57.8% | 24.7% / 24.7% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 4.6% (2.8%–6.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.8% (0.2%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.6% (0.8%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 1.0% (0.3%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.5% (0.8%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.2%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 182 days out (2016-10-23)

Actual matchup: current 7.6% (winner given matchup 58.8%), fitted 7.6% (winner given matchup 58.8%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 17.0% (13.0%–21.1%) | 9.0% / 9.0% ✓ | 5.3% / 5.3% ✓ |
| le_pen | 21.3% | 29.2% (23.1%–35.6%) | 92.4% / 92.4% ✓ | 31.2% / 31.2% |
| fillon | 20.0% | 21.7% (15.7%–28.2%) | 47.7% / 47.7% | 45.2% / 45.2% |
| melenchon | 19.6% | 22.1% (15.8%–29.0%) | 50.9% / 50.9% | 18.3% / 18.3% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 4.8% (2.9%–6.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.8% (0.2%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.5% (0.9%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 1.0% (0.3%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.4% (0.7%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 152 days out (2016-11-22)

Actual matchup: current 10.2% (winner given matchup 52.0%), fitted 10.2% (winner given matchup 52.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 16.3% (11.5%–21.7%) | 11.5% / 11.5% ✓ | 5.9% / 5.9% ✓ |
| le_pen | 21.3% | 31.7% (24.4%–39.3%) | 94.8% / 94.8% ✓ | 29.4% / 29.4% |
| fillon | 20.0% | 22.6% (15.0%–30.9%) | 54.0% / 54.0% | 51.7% / 51.7% |
| melenchon | 19.6% | 20.7% (14.2%–27.6%) | 39.7% / 39.7% | 13.0% / 13.0% |
| hamon | 6.4% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 3.8% (2.2%–5.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.7% (0.2%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.5% (0.8%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.8% (0.3%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.4% (0.8%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.2%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 122 days out (2016-12-22)

Actual matchup: current 7.8% (winner given matchup 98.5%), fitted 7.8% (winner given matchup 98.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 16.7% (10.8%–23.1%) | 15.7% / 15.7% ✓ | 10.2% / 10.2% ✓ |
| le_pen | 21.3% | 26.8% (18.5%–35.8%) | 81.0% / 81.0% ✓ | 5.9% / 5.9% |
| fillon | 20.0% | 27.7% (18.0%–38.0%) | 79.8% / 79.8% | 75.2% / 75.2% |
| melenchon | 19.6% | 16.7% (9.0%–25.6%) | 22.6% / 22.6% | 8.3% / 8.3% |
| hamon | 6.4% | 6.1% (2.2%–11.1%) | 0.9% / 0.9% | 0.3% / 0.3% |
| dupont_aignan | 4.7% | 2.3% (1.2%–3.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.5% (0.1%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.1% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.6% (0.2%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.0% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.4% (0.1%–0.7%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 91 days out (2017-01-22)

Actual matchup: current 19.6% (winner given matchup 99.5%), fitted 19.6% (winner given matchup 99.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 20.1% (14.3%–26.4%) | 34.1% / 34.1% ✓ | 30.0% / 30.0% ✓ |
| le_pen | 21.3% | 26.4% (19.3%–34.0%) | 80.5% / 80.5% ✓ | 3.7% / 3.7% |
| fillon | 20.0% | 24.7% (17.6%–32.2%) | 71.4% / 71.4% | 60.5% / 60.5% |
| melenchon | 19.6% | 14.9% (8.8%–21.8%) | 12.5% / 12.5% | 5.2% / 5.2% |
| hamon | 6.4% | 8.5% (4.1%–13.8%) | 1.4% / 1.4% | 0.6% / 0.6% |
| dupont_aignan | 4.7% | 1.9% (1.0%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.5% (0.1%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.1% (0.6%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.5% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.0% (0.5%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.4% (0.2%–0.7%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 84 days out (2017-01-29)

Actual matchup: current 12.6% (winner given matchup 99.6%), fitted 12.6% (winner given matchup 99.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 18.7% (13.2%–24.5%) | 28.7% / 28.7% ✓ | 25.1% / 25.1% ✓ |
| le_pen | 21.3% | 23.9% (17.4%–31.0%) | 73.6% / 73.6% ✓ | 4.1% / 4.1% |
| fillon | 20.0% | 24.9% (17.6%–32.7%) | 76.1% / 76.1% | 61.9% / 61.9% |
| melenchon | 19.6% | 14.8% (8.6%–21.4%) | 13.7% / 13.7% | 5.7% / 5.7% |
| hamon | 6.4% | 12.5% (6.7%–19.1%) | 8.0% / 8.0% | 3.1% / 3.1% |
| dupont_aignan | 4.7% | 1.9% (1.1%–3.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.5% (0.1%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.0% (0.5%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.5% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.5%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.4% (0.2%–0.7%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 77 days out (2017-02-05)

Actual matchup: current 22.6% (winner given matchup 99.5%), fitted 22.6% (winner given matchup 99.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 19.3% (12.3%–26.9%) | 38.3% / 38.3% ✓ | 34.1% / 34.1% ✓ |
| le_pen | 21.3% | 25.2% (16.7%–34.6%) | 73.4% / 73.4% ✓ | 10.3% / 10.3% |
| fillon | 20.0% | 20.7% (12.9%–29.3%) | 48.1% / 48.1% | 37.2% / 37.2% |
| melenchon | 19.6% | 12.8% (6.2%–20.5%) | 11.6% / 11.6% | 5.2% / 5.2% |
| hamon | 6.4% | 17.1% (9.2%–26.0%) | 28.6% / 28.6% | 13.2% / 13.2% |
| dupont_aignan | 4.7% | 2.0% (0.9%–3.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.4% (0.1%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.5% (0.1%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.4% (0.1%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 70 days out (2017-02-12)

Actual matchup: current 35.5% (winner given matchup 99.7%), fitted 35.5% (winner given matchup 99.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 20.0% (12.9%–27.8%) | 45.8% / 45.8% ✓ | 43.0% / 43.0% ✓ |
| le_pen | 21.3% | 27.8% (19.0%–37.3%) | 84.7% / 84.7% ✓ | 11.9% / 11.9% |
| fillon | 20.0% | 18.7% (11.7%–26.4%) | 36.7% / 36.7% | 30.3% / 30.3% |
| melenchon | 19.6% | 13.6% (7.0%–21.1%) | 13.6% / 13.6% | 6.2% / 6.2% |
| hamon | 6.4% | 15.0% (8.2%–22.8%) | 19.1% / 19.1% | 8.6% / 8.6% |
| dupont_aignan | 4.7% | 2.0% (1.0%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.4% (0.1%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.6% (0.2%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.3%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.4% (0.2%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 63 days out (2017-02-19)

Actual matchup: current 28.1% (winner given matchup 99.6%), fitted 28.1% (winner given matchup 99.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 19.2% (12.6%–26.4%) | 39.8% / 39.8% ✓ | 36.7% / 36.7% ✓ |
| le_pen | 21.3% | 26.3% (18.2%–35.2%) | 81.1% / 81.1% ✓ | 12.1% / 12.1% |
| fillon | 20.0% | 19.5% (12.6%–27.1%) | 43.1% / 43.1% | 35.1% / 35.1% |
| melenchon | 19.6% | 14.0% (7.4%–21.3%) | 14.3% / 14.3% | 6.5% / 6.5% |
| hamon | 6.4% | 15.7% (8.9%–23.3%) | 21.7% / 21.7% | 9.7% / 9.7% |
| dupont_aignan | 4.7% | 2.5% (1.2%–4.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.5% (0.1%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.7% (0.2%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.5% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.4% (0.2%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 56 days out (2017-02-26)

Actual matchup: current 55.4% (winner given matchup 99.3%), fitted 55.4% (winner given matchup 99.3%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 23.1% (15.7%–31.1%) | 67.4% / 67.4% ✓ | 64.5% / 64.5% ✓ |
| le_pen | 21.3% | 27.4% (19.5%–35.9%) | 86.5% / 86.5% ✓ | 5.7% / 5.7% |
| fillon | 20.0% | 18.6% (12.4%–25.3%) | 32.3% / 32.3% | 24.1% / 24.1% |
| melenchon | 19.6% | 12.5% (6.9%–18.6%) | 7.1% / 7.1% | 3.0% / 3.0% |
| hamon | 6.4% | 12.5% (7.2%–18.3%) | 6.6% / 6.6% | 2.7% / 2.7% |
| dupont_aignan | 4.7% | 2.7% (1.3%–4.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.6% (0.1%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.7% (0.2%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 49 days out (2017-03-05)

Actual matchup: current 69.9% (winner given matchup 99.5%), fitted 69.9% (winner given matchup 99.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 24.9% (17.6%–32.7%) | 80.1% / 80.1% ✓ | 77.7% / 77.7% ✓ |
| le_pen | 21.3% | 27.5% (19.9%–35.6%) | 89.0% / 89.0% ✓ | 4.4% / 4.4% |
| fillon | 20.0% | 16.6% (10.7%–23.1%) | 19.2% / 19.2% | 13.3% / 13.3% |
| melenchon | 19.6% | 11.3% (6.5%–16.6%) | 3.5% / 3.5% | 1.4% / 1.4% |
| hamon | 6.4% | 13.6% (8.3%–19.5%) | 8.2% / 8.2% | 3.2% / 3.2% |
| dupont_aignan | 4.7% | 2.4% (1.2%–3.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.6% (0.1%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.0% (0.5%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.7% (0.2%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 42 days out (2017-03-12)

Actual matchup: current 43.6% (winner given matchup 99.6%), fitted 43.6% (winner given matchup 99.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 24.9% (17.9%–32.4%) | 73.5% / 73.5% ✓ | 72.5% / 72.5% ✓ |
| le_pen | 21.3% | 24.0% (17.4%–31.2%) | 68.5% / 68.5% ✓ | 2.0% / 2.0% |
| fillon | 20.0% | 21.9% (14.8%–29.5%) | 50.5% / 50.5% | 22.8% / 22.8% |
| melenchon | 19.6% | 10.4% (6.1%–15.1%) | 1.6% / 1.6% | 0.5% / 0.5% |
| hamon | 6.4% | 13.2% (8.2%–18.7%) | 5.8% / 5.8% | 2.1% / 2.1% |
| dupont_aignan | 4.7% | 2.3% (1.2%–3.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.5% (0.1%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.8% (0.4%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.7% (0.2%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.5%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.5% (0.2%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 35 days out (2017-03-19)

Actual matchup: current 71.0% (winner given matchup 99.5%), fitted 71.0% (winner given matchup 99.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 25.6% (18.9%–32.7%) | 84.7% / 84.7% ✓ | 83.5% / 83.5% ✓ |
| le_pen | 21.3% | 25.8% (19.2%–32.9%) | 85.7% / 85.7% ✓ | 2.6% / 2.6% |
| fillon | 20.0% | 17.7% (12.2%–23.6%) | 21.8% / 21.8% | 10.9% / 10.9% |
| melenchon | 19.6% | 11.5% (7.0%–16.4%) | 2.9% / 2.9% | 1.1% / 1.1% |
| hamon | 6.4% | 12.7% (8.0%–17.9%) | 4.8% / 4.8% | 1.9% / 1.9% |
| dupont_aignan | 4.7% | 3.1% (1.8%–4.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 0.6% (0.2%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.8% (0.4%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.0% (0.6%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.2%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 28 days out (2017-03-26)

Actual matchup: current 66.7% (winner given matchup 99.7%), fitted 66.7% (winner given matchup 99.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 24.6% (18.3%–31.1%) | 83.2% / 83.2% ✓ | 81.4% / 81.4% ✓ |
| le_pen | 21.3% | 24.4% (18.3%–30.8%) | 82.4% / 82.4% ✓ | 3.2% / 3.2% |
| fillon | 20.0% | 17.4% (12.4%–22.8%) | 21.4% / 21.4% | 10.2% / 10.2% |
| melenchon | 19.6% | 14.1% (9.0%–19.7%) | 9.6% / 9.6% | 3.9% / 3.9% |
| hamon | 6.4% | 11.7% (7.2%–16.7%) | 3.5% / 3.5% | 1.4% / 1.4% |
| dupont_aignan | 4.7% | 4.2% (2.6%–6.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 1.1% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 21 days out (2017-04-02)

Actual matchup: current 78.6% (winner given matchup 99.6%), fitted 78.6% (winner given matchup 99.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 24.5% (19.5%–29.7%) | 88.4% / 88.4% ✓ | 87.0% / 87.0% ✓ |
| le_pen | 21.3% | 24.8% (19.8%–30.0%) | 89.7% / 89.7% ✓ | 2.4% / 2.4% |
| fillon | 20.0% | 17.0% (13.0%–21.2%) | 11.8% / 11.8% | 5.8% / 5.8% |
| melenchon | 19.6% | 15.2% (10.7%–20.0%) | 9.0% / 9.0% | 4.4% / 4.4% |
| hamon | 6.4% | 10.8% (7.1%–14.9%) | 1.1% / 1.1% | 0.4% / 0.4% |
| dupont_aignan | 4.7% | 3.9% (2.7%–5.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 1.0% (0.6%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 0.8% (0.5%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.5%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 14 days out (2017-04-09)

Actual matchup: current 63.7% (winner given matchup 99.7%), fitted 63.7% (winner given matchup 99.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 23.3% (18.3%–28.6%) | 80.2% / 80.2% ✓ | 76.7% / 76.7% ✓ |
| le_pen | 21.3% | 23.7% (18.7%–29.0%) | 82.5% / 82.5% ✓ | 1.0% / 1.0% |
| fillon | 20.0% | 16.5% (12.5%–20.7%) | 11.9% / 11.9% | 5.4% / 5.4% |
| melenchon | 19.6% | 17.9% (13.1%–22.9%) | 25.0% / 25.0% | 16.7% / 16.7% |
| hamon | 6.4% | 9.0% (5.9%–12.5%) | 0.4% / 0.4% | 0.2% / 0.2% |
| dupont_aignan | 4.7% | 4.1% (2.7%–5.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 1.2% (0.7%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.7% (1.1%–2.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.7% (0.4%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.2% (0.7%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.7% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2017, 7 days out (2017-04-16)

Actual matchup: current 33.6% (winner given matchup 99.6%), fitted 33.6% (winner given matchup 99.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 24.0% | 22.1% (17.7%–26.6%) | 65.2% / 65.2% ✓ | 64.4% / 64.4% ✓ |
| le_pen | 21.3% | 21.8% (17.5%–26.2%) | 62.1% / 62.1% ✓ | 0.7% / 0.7% |
| fillon | 20.0% | 19.0% (15.0%–23.2%) | 28.8% / 28.8% | 10.3% / 10.3% |
| melenchon | 19.6% | 20.2% (15.7%–24.9%) | 43.7% / 43.7% | 24.5% / 24.5% |
| hamon | 6.4% | 8.2% (5.5%–11.0%) | 0.2% / 0.2% | 0.0% / 0.0% |
| dupont_aignan | 4.7% | 3.6% (2.5%–4.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 1.2% | 1.1% (0.7%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 1.1% | 1.7% (1.2%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| asselineau | 0.9% | 0.9% (0.5%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.5%–1.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| cheminade | 0.2% | 0.6% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 334 days out (2021-05-11)

Actual matchup: current 88.4% (winner given matchup 88.0%), fitted 88.4% (winner given matchup 88.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 27.9% (18.3%–38.3%) | 92.0% / 92.0% ✓ | 80.3% / 80.3% ✓ |
| le_pen | 23.1% | 26.7% (20.1%–33.6%) | 96.4% / 96.4% ✓ | 13.0% / 13.0% |
| melenchon | 21.9% | 10.8% (5.0%–17.3%) | 5.3% / 5.3% | 2.9% / 2.9% |
| zemmour | 7.1% | not polled | 0.0% / 0.0% | 0.0% / 0.0% |
| pecresse | 4.8% | 11.0% (6.2%–16.6%) | 4.3% / 4.3% | 2.8% / 2.8% |
| jadot | 4.6% | 6.0% (2.2%–10.5%) | 0.9% / 0.9% | 0.5% / 0.5% |
| lassalle | 3.1% | 2.5% (0.8%–4.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 1.2% (0.3%–2.3%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 6.0% (3.0%–9.3%) | 0.2% / 0.2% | 0.1% / 0.1% |
| hidalgo | 1.8% | 5.6% (2.0%–10.0%) | 0.7% / 0.7% | 0.4% / 0.4% |
| poutou | 0.8% | 1.6% (0.8%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.3%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 304 days out (2021-06-10)

Actual matchup: current 83.1% (winner given matchup 77.5%), fitted 83.1% (winner given matchup 77.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 26.1% (15.8%–37.5%) | 90.3% / 90.3% ✓ | 70.1% / 70.1% ✓ |
| le_pen | 23.1% | 27.1% (17.3%–37.2%) | 92.6% / 92.6% ✓ | 23.9% / 23.9% |
| melenchon | 21.9% | 10.3% (4.5%–16.8%) | 7.4% / 7.4% | 2.8% / 2.8% |
| zemmour | 7.1% | 6.8% (2.0%–13.2%) | 5.0% / 5.0% | 1.2% / 1.2% |
| pecresse | 4.8% | 9.1% (5.7%–13.0%) | 1.9% / 1.9% | 1.0% / 1.0% |
| jadot | 4.6% | 5.6% (2.0%–10.0%) | 1.2% / 1.2% | 0.5% / 0.5% |
| lassalle | 3.1% | 1.5% (0.5%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 1.5% (0.4%–2.8%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 4.3% (2.4%–6.6%) | 0.1% / 0.1% | 0.0% / 0.0% |
| hidalgo | 1.8% | 5.9% (2.1%–10.5%) | 1.4% / 1.4% | 0.5% / 0.5% |
| poutou | 0.8% | 1.0% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.3%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 273 days out (2021-07-11)

Actual matchup: current 80.4% (winner given matchup 87.6%), fitted 80.4% (winner given matchup 87.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 24.9% (17.6%–32.7%) | 92.8% / 92.8% ✓ | 79.3% / 79.3% ✓ |
| le_pen | 23.1% | 24.0% (15.6%–32.7%) | 87.1% / 87.1% ✓ | 12.0% / 12.0% |
| melenchon | 21.9% | 8.9% (3.8%–14.7%) | 4.3% / 4.3% | 1.9% / 1.9% |
| zemmour | 7.1% | 6.3% (2.0%–11.8%) | 3.5% / 3.5% | 0.6% / 0.6% |
| pecresse | 4.8% | 11.8% (7.0%–17.4%) | 8.2% / 8.2% | 4.4% / 4.4% |
| jadot | 4.6% | 6.9% (2.7%–11.8%) | 1.9% / 1.9% | 0.9% / 0.9% |
| lassalle | 3.1% | 1.7% (0.6%–3.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.2% (0.7%–4.0%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 4.6% (2.6%–6.8%) | 0.1% / 0.1% | 0.0% / 0.0% |
| hidalgo | 1.8% | 6.6% (2.5%–11.5%) | 1.9% / 1.9% | 0.8% / 0.8% |
| poutou | 0.8% | 1.2% (0.6%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.0% (0.4%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 243 days out (2021-08-10)

Actual matchup: current 78.5% (winner given matchup 87.5%), fitted 78.5% (winner given matchup 87.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 25.0% (17.2%–33.2%) | 91.4% / 91.4% ✓ | 77.8% / 77.8% ✓ |
| le_pen | 23.1% | 23.9% (15.5%–32.6%) | 86.6% / 86.6% ✓ | 12.1% / 12.1% |
| melenchon | 21.9% | 9.2% (4.1%–15.1%) | 5.0% / 5.0% | 2.2% / 2.2% |
| zemmour | 7.1% | 6.1% (2.0%–11.2%) | 3.0% / 3.0% | 0.5% / 0.5% |
| pecresse | 4.8% | 11.8% (6.5%–18.1%) | 10.0% / 10.0% | 5.6% / 5.6% |
| jadot | 4.6% | 6.8% (2.7%–11.7%) | 1.9% / 1.9% | 0.9% / 0.9% |
| lassalle | 3.1% | 1.7% (0.6%–3.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.2% (0.7%–4.0%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 4.6% (2.5%–7.1%) | 0.1% / 0.1% | 0.0% / 0.0% |
| hidalgo | 1.8% | 6.6% (2.6%–11.5%) | 1.9% / 1.9% | 0.8% / 0.8% |
| poutou | 0.8% | 1.2% (0.6%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.4%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 212 days out (2021-09-10)

Actual matchup: current 79.5% (winner given matchup 88.2%), fitted 79.5% (winner given matchup 88.2%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 24.3% (19.1%–29.7%) | 95.8% / 95.8% ✓ | 81.2% / 81.2% ✓ |
| le_pen | 23.1% | 21.2% (14.7%–27.7%) | 83.4% / 83.4% ✓ | 10.3% / 10.3% |
| melenchon | 21.9% | 9.8% (4.9%–15.4%) | 5.4% / 5.4% | 2.2% / 2.2% |
| zemmour | 7.1% | 7.3% (3.1%–12.3%) | 3.9% / 3.9% | 0.5% / 0.5% |
| pecresse | 4.8% | 12.2% (8.0%–17.0%) | 7.3% / 7.3% | 4.0% / 4.0% |
| jadot | 4.6% | 7.2% (3.2%–11.8%) | 1.9% / 1.9% | 0.8% / 0.8% |
| lassalle | 3.1% | 1.6% (0.7%–2.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.3% (0.9%–3.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 4.6% (3.0%–6.3%) | 0.1% / 0.1% | 0.0% / 0.0% |
| hidalgo | 1.8% | 7.4% (3.4%–12.1%) | 2.1% / 2.1% | 0.9% / 0.9% |
| poutou | 0.8% | 1.1% (0.6%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.4%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 182 days out (2021-10-10)

Actual matchup: current 45.7% (winner given matchup 91.1%), fitted 45.7% (winner given matchup 91.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 26.0% (17.2%–35.4%) | 90.0% / 90.0% ✓ | 83.7% / 83.7% ✓ |
| le_pen | 23.1% | 17.6% (9.1%–27.1%) | 53.8% / 53.8% ✓ | 7.6% / 7.6% |
| melenchon | 21.9% | 8.0% (3.4%–13.4%) | 4.8% / 4.8% | 0.8% / 0.8% |
| zemmour | 7.1% | 14.9% (6.9%–24.0%) | 34.8% / 34.8% | 4.1% / 4.1% |
| pecresse | 4.8% | 11.6% (7.0%–16.8%) | 11.1% / 11.1% | 2.8% / 2.8% |
| jadot | 4.6% | 7.5% (3.2%–12.8%) | 3.9% / 3.9% | 0.7% / 0.7% |
| lassalle | 3.1% | 1.3% (0.4%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.3% (0.8%–4.1%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 3.5% (1.8%–5.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 5.4% (2.1%–9.5%) | 1.4% / 1.4% | 0.2% / 0.2% |
| poutou | 0.8% | 1.1% (0.5%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.0% (0.4%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 152 days out (2021-11-09)

Actual matchup: current 39.1% (winner given matchup 90.4%), fitted 39.1% (winner given matchup 90.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 24.7% (16.8%–33.3%) | 88.4% / 88.4% ✓ | 82.9% / 82.9% ✓ |
| le_pen | 23.1% | 16.9% (8.9%–25.9%) | 48.1% / 48.1% ✓ | 7.6% / 7.6% |
| melenchon | 21.9% | 9.2% (4.4%–14.8%) | 6.8% / 6.8% | 1.0% / 1.0% |
| zemmour | 7.1% | 16.0% (8.2%–24.8%) | 41.1% / 41.1% | 5.1% / 5.1% |
| pecresse | 4.8% | 11.0% (6.2%–16.7%) | 10.6% / 10.6% | 2.5% / 2.5% |
| jadot | 4.6% | 7.9% (3.6%–12.9%) | 3.8% / 3.8% | 0.7% / 0.7% |
| lassalle | 3.1% | 1.2% (0.5%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.2% (0.8%–3.8%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 3.4% (1.9%–5.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 5.2% (2.1%–8.9%) | 1.0% / 1.0% | 0.2% / 0.2% |
| poutou | 0.8% | 1.2% (0.6%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 1.0% (0.4%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 122 days out (2021-12-09)

Actual matchup: current 22.1% (winner given matchup 90.4%), fitted 22.1% (winner given matchup 90.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 22.8% (15.7%–30.3%) | 80.9% / 80.9% ✓ | 68.9% / 68.9% ✓ |
| le_pen | 23.1% | 15.7% (8.9%–23.2%) | 33.8% / 33.8% ✓ | 4.5% / 4.5% |
| melenchon | 21.9% | 9.5% (4.8%–14.8%) | 5.6% / 5.6% | 0.9% / 0.9% |
| zemmour | 7.1% | 13.8% (7.3%–21.3%) | 22.8% / 22.8% | 3.4% / 3.4% |
| pecresse | 4.8% | 18.9% (12.0%–26.7%) | 54.9% / 54.9% | 21.8% / 21.8% |
| jadot | 4.6% | 6.8% (3.2%–11.1%) | 1.6% / 1.6% | 0.3% / 0.3% |
| lassalle | 3.1% | 1.1% (0.5%–2.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.0% (0.8%–3.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.8% (1.5%–4.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 4.6% (1.9%–7.7%) | 0.4% / 0.4% | 0.1% / 0.1% |
| poutou | 0.8% | 1.2% (0.6%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.9% (0.4%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 91 days out (2022-01-09)

Actual matchup: current 39.8% (winner given matchup 95.7%), fitted 39.8% (winner given matchup 95.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 25.8% (18.4%–33.6%) | 92.7% / 92.7% ✓ | 85.1% / 85.1% ✓ |
| le_pen | 23.1% | 16.5% (10.2%–23.3%) | 45.4% / 45.4% ✓ | 3.1% / 3.1% |
| melenchon | 21.9% | 10.6% (6.1%–15.5%) | 7.3% / 7.3% | 0.7% / 0.7% |
| zemmour | 7.1% | 12.8% (7.3%–19.0%) | 17.7% / 17.7% | 1.3% / 1.3% |
| pecresse | 4.8% | 15.8% (11.1%–21.1%) | 35.6% / 35.6% | 9.6% / 9.6% |
| jadot | 4.6% | 6.8% (3.5%–10.6%) | 1.1% / 1.1% | 0.1% / 0.1% |
| lassalle | 3.1% | 1.2% (0.5%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.4% (1.1%–3.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.5% (1.4%–3.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 3.6% (1.7%–5.7%) | 0.1% / 0.1% | 0.0% / 0.0% |
| poutou | 0.8% | 1.1% (0.7%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.8% (0.4%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 84 days out (2022-01-16)

Actual matchup: current 33.5% (winner given matchup 95.1%), fitted 33.5% (winner given matchup 95.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 24.0% (17.2%–31.2%) | 89.1% / 89.1% ✓ | 81.7% / 81.7% ✓ |
| le_pen | 23.1% | 16.2% (10.0%–22.8%) | 41.0% / 41.0% ✓ | 3.6% / 3.6% |
| melenchon | 21.9% | 11.2% (6.6%–16.5%) | 10.0% / 10.0% | 1.3% / 1.3% |
| zemmour | 7.1% | 13.9% (8.1%–20.3%) | 23.2% / 23.2% | 2.2% / 2.2% |
| pecresse | 4.8% | 15.9% (10.8%–21.5%) | 35.4% / 35.4% | 11.0% / 11.0% |
| jadot | 4.6% | 6.7% (3.4%–10.4%) | 1.0% / 1.0% | 0.2% / 0.2% |
| lassalle | 3.1% | 1.5% (0.7%–2.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.5% (1.2%–4.1%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.5% (1.4%–3.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 3.7% (1.8%–6.0%) | 0.2% / 0.2% | 0.0% / 0.0% |
| poutou | 0.8% | 1.2% (0.7%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.1%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 77 days out (2022-01-23)

Actual matchup: current 34.2% (winner given matchup 95.5%), fitted 34.2% (winner given matchup 95.5%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 23.9% (17.2%–30.9%) | 88.7% / 88.7% ✓ | 82.4% / 82.4% ✓ |
| le_pen | 23.1% | 16.6% (10.4%–23.2%) | 42.1% / 42.1% ✓ | 3.5% / 3.5% |
| melenchon | 21.9% | 11.7% (6.7%–17.4%) | 12.4% / 12.4% | 1.6% / 1.6% |
| zemmour | 7.1% | 13.4% (7.9%–19.5%) | 18.3% / 18.3% | 1.7% / 1.7% |
| pecresse | 4.8% | 16.3% (10.8%–22.4%) | 36.9% / 36.9% | 10.4% / 10.4% |
| jadot | 4.6% | 7.1% (3.7%–11.1%) | 1.5% / 1.5% | 0.3% / 0.3% |
| lassalle | 3.1% | 1.2% (0.6%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.7% (1.3%–4.4%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 1.9% (1.1%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 3.5% (1.6%–5.7%) | 0.1% / 0.1% | 0.0% / 0.0% |
| poutou | 0.8% | 1.1% (0.7%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 70 days out (2022-01-30)

Actual matchup: current 40.4% (winner given matchup 95.0%), fitted 40.4% (winner given matchup 95.0%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 23.7% (17.3%–30.4%) | 88.9% / 88.9% ✓ | 82.8% / 82.8% ✓ |
| le_pen | 23.1% | 17.4% (11.0%–24.2%) | 48.9% / 48.9% ✓ | 4.3% / 4.3% |
| melenchon | 21.9% | 11.6% (6.9%–16.7%) | 10.1% / 10.1% | 1.3% / 1.3% |
| zemmour | 7.1% | 13.3% (7.8%–19.4%) | 17.8% / 17.8% | 1.9% / 1.9% |
| pecresse | 4.8% | 15.9% (10.9%–21.4%) | 33.1% / 33.1% | 9.5% / 9.5% |
| jadot | 4.6% | 6.7% (3.6%–10.3%) | 0.9% / 0.9% | 0.2% / 0.2% |
| lassalle | 3.1% | 1.2% (0.6%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 2.9% (1.4%–4.7%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.0% (1.2%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 3.7% (1.8%–5.8%) | 0.1% / 0.1% | 0.0% / 0.0% |
| poutou | 0.8% | 1.0% (0.6%–1.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 63 days out (2022-02-06)

Actual matchup: current 43.9% (winner given matchup 94.8%), fitted 43.9% (winner given matchup 94.8%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 24.2% (17.9%–30.8%) | 91.0% / 91.0% ✓ | 85.9% / 85.9% ✓ |
| le_pen | 23.1% | 17.6% (11.3%–24.3%) | 51.1% / 51.1% ✓ | 4.6% / 4.6% |
| melenchon | 21.9% | 11.2% (6.7%–16.2%) | 8.7% / 8.7% | 1.1% / 1.1% |
| zemmour | 7.1% | 14.4% (8.8%–20.6%) | 23.6% / 23.6% | 2.1% / 2.1% |
| pecresse | 4.8% | 15.1% (10.6%–20.0%) | 24.7% / 24.7% | 6.2% / 6.2% |
| jadot | 4.6% | 6.2% (3.3%–9.6%) | 0.7% / 0.7% | 0.1% / 0.1% |
| lassalle | 3.1% | 1.2% (0.7%–1.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.4% (1.7%–5.3%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.0% (1.2%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 3.1% (1.5%–5.0%) | 0.1% / 0.1% | 0.0% / 0.0% |
| poutou | 0.8% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.5% (0.3%–0.8%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 56 days out (2022-02-13)

Actual matchup: current 41.9% (winner given matchup 95.4%), fitted 41.9% (winner given matchup 95.4%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 25.4% (19.0%–32.0%) | 93.6% / 93.6% ✓ | 88.8% / 88.8% ✓ |
| le_pen | 23.1% | 17.1% (11.1%–23.4%) | 46.9% / 46.9% ✓ | 3.4% / 3.4% |
| melenchon | 21.9% | 10.6% (6.3%–15.4%) | 6.9% / 6.9% | 0.6% / 0.6% |
| zemmour | 7.1% | 14.4% (9.0%–20.3%) | 23.3% / 23.3% | 1.5% / 1.5% |
| pecresse | 4.8% | 15.5% (10.9%–20.6%) | 28.8% / 28.8% | 5.6% / 5.6% |
| jadot | 4.6% | 5.7% (3.1%–8.8%) | 0.4% / 0.4% | 0.1% / 0.1% |
| lassalle | 3.1% | 1.2% (0.7%–1.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 4.1% (2.1%–6.5%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 1.7% (1.0%–2.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.7% (1.4%–4.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.3%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 49 days out (2022-02-20)

Actual matchup: current 30.6% (winner given matchup 95.1%), fitted 30.6% (winner given matchup 95.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 24.0% (18.1%–30.2%) | 92.3% / 92.3% ✓ | 88.9% / 88.9% ✓ |
| le_pen | 23.1% | 15.7% (10.3%–21.4%) | 35.6% / 35.6% ✓ | 3.0% / 3.0% |
| melenchon | 21.9% | 11.7% (7.1%–16.8%) | 11.8% / 11.8% | 1.1% / 1.1% |
| zemmour | 7.1% | 15.0% (9.7%–20.9%) | 29.8% / 29.8% | 2.0% / 2.0% |
| pecresse | 4.8% | 15.3% (10.8%–20.3%) | 29.8% / 29.8% | 4.9% / 4.9% |
| jadot | 4.6% | 5.9% (3.2%–9.1%) | 0.5% / 0.5% | 0.1% / 0.1% |
| lassalle | 3.1% | 1.5% (0.8%–2.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 4.7% (2.5%–7.1%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 1.9% (1.2%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.7% (1.4%–4.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.0% (0.6%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.3%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 42 days out (2022-02-27)

Actual matchup: current 43.2% (winner given matchup 95.1%), fitted 43.2% (winner given matchup 95.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 25.9% (19.9%–32.2%) | 96.1% / 96.1% ✓ | 93.2% / 93.2% ✓ |
| le_pen | 23.1% | 16.5% (11.2%–22.2%) | 46.3% / 46.3% ✓ | 3.3% / 3.3% |
| melenchon | 21.9% | 12.4% (7.9%–17.3%) | 13.9% / 13.9% | 0.8% / 0.8% |
| zemmour | 7.1% | 15.1% (9.9%–20.6%) | 30.8% / 30.8% | 1.3% / 1.3% |
| pecresse | 4.8% | 13.0% (9.1%–17.2%) | 12.5% / 12.5% | 1.3% / 1.3% |
| jadot | 4.6% | 5.2% (2.9%–7.9%) | 0.2% / 0.2% | 0.0% / 0.0% |
| lassalle | 3.1% | 1.4% (0.8%–2.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 4.5% (2.5%–6.9%) | 0.1% / 0.1% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.0% (1.2%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.4% (1.3%–3.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.0% (0.6%–1.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 35 days out (2022-03-06)

Actual matchup: current 57.3% (winner given matchup 97.8%), fitted 57.3% (winner given matchup 97.8%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 30.3% (24.0%–36.7%) | 99.2% / 99.2% ✓ | 97.7% / 97.7% ✓ |
| le_pen | 23.1% | 16.2% (11.3%–21.3%) | 58.0% / 58.0% ✓ | 1.6% / 1.6% |
| melenchon | 21.9% | 10.9% (7.2%–15.0%) | 9.1% / 9.1% | 0.1% / 0.1% |
| zemmour | 7.1% | 13.5% (9.0%–18.4%) | 23.9% / 23.9% | 0.3% / 0.3% |
| pecresse | 4.8% | 11.8% (8.5%–15.4%) | 9.4% / 9.4% | 0.2% / 0.2% |
| jadot | 4.6% | 6.1% (3.6%–8.9%) | 0.4% / 0.4% | 0.0% / 0.0% |
| lassalle | 3.1% | 1.6% (1.0%–2.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.6% (2.0%–5.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.1% (1.3%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.2% (1.3%–3.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.0% (0.7%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.4%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 28 days out (2022-03-13)

Actual matchup: current 62.3% (winner given matchup 98.6%), fitted 62.3% (winner given matchup 98.6%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 30.6% (24.6%–36.7%) | 99.4% / 99.4% ✓ | 98.4% / 98.4% ✓ |
| le_pen | 23.1% | 16.2% (11.7%–20.9%) | 62.8% / 62.8% ✓ | 1.1% / 1.1% |
| melenchon | 21.9% | 11.6% (8.0%–15.4%) | 11.3% / 11.3% | 0.1% / 0.1% |
| zemmour | 7.1% | 12.6% (8.5%–17.1%) | 16.7% / 16.7% | 0.2% / 0.2% |
| pecresse | 4.8% | 11.8% (8.6%–15.3%) | 9.6% / 9.6% | 0.1% / 0.1% |
| jadot | 4.6% | 5.4% (3.3%–7.6%) | 0.1% / 0.1% | 0.0% / 0.0% |
| lassalle | 3.1% | 2.1% (1.4%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.5% (2.1%–5.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.1% (1.4%–2.9%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.1% (1.3%–3.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.3% (0.8%–1.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.4%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 21 days out (2022-03-20)

Actual matchup: current 77.6% (winner given matchup 99.1%), fitted 77.6% (winner given matchup 99.1%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 30.1% (24.5%–35.9%) | 99.4% / 99.4% ✓ | 98.7% / 98.7% ✓ |
| le_pen | 23.1% | 17.6% (13.0%–22.3%) | 78.1% / 78.1% ✓ | 1.0% / 1.0% |
| melenchon | 21.9% | 12.3% (8.8%–16.0%) | 11.7% / 11.7% | 0.1% / 0.1% |
| zemmour | 7.1% | 11.6% (8.0%–15.6%) | 8.3% / 8.3% | 0.1% / 0.1% |
| pecresse | 4.8% | 10.2% (7.6%–13.0%) | 2.2% / 2.2% | 0.1% / 0.1% |
| jadot | 4.6% | 5.6% (3.6%–7.8%) | 0.1% / 0.1% | 0.0% / 0.0% |
| lassalle | 3.1% | 2.4% (1.6%–3.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.7% (2.3%–5.2%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.2% (1.5%–3.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.6% (1.6%–3.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.0% (0.7%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.6% (0.4%–0.9%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 14 days out (2022-03-27)

Actual matchup: current 74.9% (winner given matchup 97.9%), fitted 74.9% (winner given matchup 97.9%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 27.0% (22.0%–32.2%) | 98.6% / 98.6% ✓ | 96.9% / 96.9% ✓ |
| le_pen | 23.1% | 18.7% (14.3%–23.2%) | 76.2% / 76.2% ✓ | 2.2% / 2.2% |
| melenchon | 21.9% | 14.5% (10.8%–18.3%) | 18.4% / 18.4% | 0.5% / 0.5% |
| zemmour | 7.1% | 11.3% (7.9%–14.9%) | 4.0% / 4.0% | 0.1% / 0.1% |
| pecresse | 4.8% | 11.1% (8.2%–14.2%) | 2.6% / 2.6% | 0.1% / 0.1% |
| jadot | 4.6% | 5.3% (3.5%–7.2%) | 0.1% / 0.1% | 0.0% / 0.0% |
| lassalle | 3.1% | 2.5% (1.7%–3.3%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.6% (2.4%–5.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.0% (1.4%–2.7%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.2% (1.4%–3.1%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.1% (0.8%–1.4%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.5%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

### 2022, 7 days out (2022-04-03)

Actual matchup: current 87.7% (winner given matchup 93.7%), fitted 87.7% (winner given matchup 93.7%).

| Candidate | Result | Fitted share (80%) | Qualify: current / fitted | Win: current / fitted |
|---|---|---|---|---|
| macron | 27.9% | 26.1% (21.5%–30.8%) | 98.1% / 98.1% ✓ | 92.5% / 92.5% ✓ |
| le_pen | 23.1% | 21.5% (17.1%–26.1%) | 89.5% / 89.5% ✓ | 6.5% / 6.5% |
| melenchon | 21.9% | 15.0% (11.6%–18.5%) | 10.3% / 10.3% | 0.9% / 0.9% |
| zemmour | 7.1% | 10.9% (7.9%–14.2%) | 1.8% / 1.8% | 0.1% / 0.1% |
| pecresse | 4.8% | 8.9% (6.8%–11.1%) | 0.2% / 0.2% | 0.0% / 0.0% |
| jadot | 4.6% | 5.1% (3.5%–6.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| lassalle | 3.1% | 2.7% (1.9%–3.5%) | 0.0% / 0.0% | 0.0% / 0.0% |
| roussel | 2.3% | 3.5% (2.3%–4.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| dupont_aignan | 2.1% | 2.3% (1.6%–3.0%) | 0.0% / 0.0% | 0.0% / 0.0% |
| hidalgo | 1.8% | 2.1% (1.4%–2.8%) | 0.0% / 0.0% | 0.0% / 0.0% |
| poutou | 0.8% | 1.2% (0.9%–1.6%) | 0.0% / 0.0% | 0.0% / 0.0% |
| arthaud | 0.6% | 0.7% (0.5%–1.0%) | 0.0% / 0.0% | 0.0% / 0.0% |

## Signal layers

Every replay fitted again with news and attention spikes allowed to add variance (model/src/signals.rs), `shock_sd` chosen by marginal likelihood from [0.0, 0.1, 0.2, 0.3, 0.5], and simulated at the current election-day errors. Contenders are those of the polls-only replays. The first row is the published settings, the signal weights in config/model.yaml, which the owner sets; the others are alternatives. "No worse" marks a row that changes something (a `shock_sd` above zero, or a momentum tilt, in at least one replay) and scores at least as well as the polls only on both elections, on the share log score, the runoff log score and the qualification and win log-loss.

| | Qualify Brier | Qualify log-loss | Win Brier | Win log-loss | Share MAE (pts) | 80% coverage | Share log score | Round 2 log score | P(winner \| matchup) |
|---|---|---|---|---|---|---|---|---|---|
| Polls only | 0.048 | 0.178 | 0.023 | 0.078 | 2.30 | 68.3% | -110.1 | -43.91 | 84.3% |
| published settings (config/model.yaml) | 0.048 | 0.179 | 0.023 | 0.079 | 2.31 | 67.1% | -116.4 | -44.61 | 84.2% |
| page views and news | 0.048 | 0.179 | 0.023 | 0.079 | 2.30 | 68.5% | -110.2 | -43.92 | 84.3% |
| page views | 0.048 | 0.179 | 0.023 | 0.079 | 2.30 | 68.3% | -109.1 | -43.91 | 84.3% |
| news | 0.048 | 0.178 | 0.023 | 0.078 | 2.30 | 67.8% | -113.7 | -44.04 | 84.3% |
| both, spike at 2× | 0.048 | 0.180 | 0.023 | 0.079 | 2.30 | 68.3% | -109.5 | -43.98 | 84.3% |
| both, spike at 5× | 0.048 | 0.177 | 0.023 | 0.077 | 2.31 | 67.4% | -119.0 | -43.97 | 84.3% |
| both, half-life 5 days | 0.048 | 0.179 | 0.023 | 0.079 | 2.30 | 68.3% | -111.3 | -43.91 | 84.3% |
| both, half-life 20 days | 0.048 | 0.179 | 0.023 | 0.079 | 2.30 | 68.5% | -108.8 | -43.94 | 84.3% |
| both, higher floors | 0.048 | 0.177 | 0.023 | 0.077 | 2.30 | 66.9% | -117.4 | -43.86 | 84.3% |
| momentum +0.05 | 0.048 | 0.178 | 0.023 | 0.078 | 2.30 | 68.2% | -109.8 | -43.93 | 84.3% |
| momentum +0.15 | 0.048 | 0.178 | 0.023 | 0.077 | 2.30 | 68.4% | -108.9 | -43.96 | 84.3% |
| momentum −0.05 | 0.048 | 0.178 | 0.023 | 0.078 | 2.31 | 68.3% | -110.9 | -43.91 | 84.3% |
| momentum −0.15 | 0.049 | 0.180 | 0.023 | 0.079 | 2.31 | 68.0% | -114.8 | -43.93 | 84.3% |

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
| 2017, polls only | 0.057 | 0.178 | 0.053 | 0.168 | 2.17 | 68.9% | -17.1 | -9.65 | 85.8% |
| 2017, published settings (config/model.yaml) | 0.061 | 0.188 | 0.056 | 0.178 | 2.18 | 67.6% | -19.1 | -10.34 | 85.3% |
| 2017, page views and news | 0.059 | 0.184 | 0.055 | 0.174 | 2.16 | 70.3% | -16.3 | -9.65 | 85.8% |
| 2017, page views | 0.059 | 0.183 | 0.055 | 0.175 | 2.15 | 68.9% | -16.2 | -9.65 | 85.8% |
| 2017, news | 0.058 | 0.182 | 0.054 | 0.172 | 2.16 | 69.8% | -17.3 | -9.79 | 85.6% |
| 2017, both, spike at 2× | 0.059 | 0.185 | 0.055 | 0.177 | 2.16 | 68.5% | -16.5 | -9.71 | 85.8% |
| 2017, both, spike at 5× | 0.057 | 0.177 | 0.054 | 0.170 | 2.18 | 68.9% | -19.4 | -9.75 | 85.8% |
| 2017, both, half-life 5 days | 0.058 | 0.180 | 0.055 | 0.173 | 2.17 | 68.9% | -17.0 | -9.64 | 85.8% |
| 2017, both, half-life 20 days | 0.059 | 0.184 | 0.055 | 0.174 | 2.15 | 69.8% | -15.9 | -9.67 | 85.8% |
| 2017, both, higher floors | 0.057 | 0.177 | 0.054 | 0.171 | 2.15 | 68.5% | -17.9 | -9.61 | 85.8% |
| 2017, momentum +0.05 | 0.056 | 0.176 | 0.053 | 0.167 | 2.16 | 68.9% | -16.5 | -9.64 | 85.8% |
| 2017, momentum +0.15 | 0.057 | 0.175 | 0.052 | 0.165 | 2.17 | 70.3% | -16.0 | -9.63 | 85.9% |
| 2017, momentum −0.05 | 0.057 | 0.179 | 0.053 | 0.169 | 2.18 | 68.9% | -18.1 | -9.67 | 85.8% |
| 2017, momentum −0.15 | 0.059 | 0.185 | 0.055 | 0.173 | 2.22 | 68.9% | -20.8 | -9.74 | 85.6% |
| 2022, polls only | 0.026 | 0.099 | 0.004 | 0.029 | 3.30 | 64.1% | -132.4 | 14.18 | 92.8% |
| 2022, published settings (config/model.yaml) | 0.025 | 0.094 | 0.003 | 0.026 | 3.34 | 59.4% | -136.7 | 14.17 | 92.8% |
| 2022, page views and news | 0.026 | 0.099 | 0.004 | 0.029 | 3.31 | 63.7% | -133.3 | 14.17 | 92.8% |
| 2022, page views | 0.026 | 0.099 | 0.004 | 0.029 | 3.30 | 64.1% | -132.4 | 14.18 | 92.8% |
| 2022, news | 0.025 | 0.097 | 0.003 | 0.027 | 3.31 | 60.6% | -135.8 | 14.19 | 92.8% |
| 2022, both, spike at 2× | 0.026 | 0.099 | 0.004 | 0.029 | 3.30 | 64.1% | -132.4 | 14.18 | 92.8% |
| 2022, both, spike at 5× | 0.025 | 0.094 | 0.003 | 0.025 | 3.31 | 59.4% | -139.0 | 14.22 | 92.9% |
| 2022, both, half-life 5 days | 0.026 | 0.099 | 0.004 | 0.029 | 3.31 | 63.7% | -133.8 | 14.17 | 92.8% |
| 2022, both, half-life 20 days | 0.026 | 0.099 | 0.004 | 0.029 | 3.30 | 64.1% | -132.4 | 14.18 | 92.8% |
| 2022, both, higher floors | 0.025 | 0.092 | 0.003 | 0.024 | 3.32 | 57.8% | -138.9 | 14.19 | 92.9% |
| 2022, momentum +0.05 | 0.026 | 0.099 | 0.004 | 0.029 | 3.31 | 63.3% | -132.7 | 14.15 | 92.8% |
| 2022, momentum +0.15 | 0.026 | 0.099 | 0.004 | 0.030 | 3.31 | 63.3% | -132.4 | 14.11 | 92.8% |
| 2022, momentum −0.05 | 0.026 | 0.100 | 0.004 | 0.029 | 3.31 | 64.1% | -132.2 | 14.20 | 92.9% |
| 2022, momentum −0.15 | 0.027 | 0.101 | 0.004 | 0.028 | 3.32 | 62.5% | -133.4 | 14.25 | 92.9% |

Spikes detected and `shock_sd` chosen, per replay (year, days before round 1):

| Replay | published settings (config/model.yaml) | page views and news | page views | news | both, spike at 2× | both, spike at 5× | both, half-life 5 days | both, half-life 20 days | both, higher floors | momentum +0.05 | momentum +0.15 | momentum −0.05 | momentum −0.15 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 2002 334 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 304 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 273 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 243 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 212 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 182 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 152 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 122 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 91 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 84 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 77 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 70 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 63 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 56 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 49 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 42 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 35 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 28 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 21 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 14 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2002 7 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 334 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 304 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 273 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 243 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 212 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 182 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 152 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 122 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 91 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 84 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 77 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 70 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 63 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 56 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 49 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 42 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 35 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 28 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 21 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 14 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2007 7 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 334 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 304 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 273 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 243 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 212 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 182 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 152 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 122 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 91 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 84 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 77 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 70 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 63 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 56 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 49 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 42 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 35 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 28 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 21 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 14 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2012 7 | 0 spikes, 0.1 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 | 0 spikes, 0 |
| 2017 334 | 9 spikes, 0.1 | 9 spikes, 0 | 22 spikes, 0 | 2 spikes, 0 | 19 spikes, 0 | 3 spikes, 0 | 9 spikes, 0 | 9 spikes, 0 | 4 spikes, 0 | 9 spikes, 0 | 9 spikes, 0 | 9 spikes, 0 | 9 spikes, 0 |
| 2017 304 | 14 spikes, 0.1 | 14 spikes, 0 | 37 spikes, 0 | 5 spikes, 0 | 31 spikes, 0 | 4 spikes, 0 | 14 spikes, 0 | 14 spikes, 0 | 6 spikes, 0 | 14 spikes, 0 | 14 spikes, 0 | 14 spikes, 0 | 14 spikes, 0 |
| 2017 273 | 22 spikes, 0.1 | 22 spikes, 0 | 53 spikes, 0 | 11 spikes, 0 | 45 spikes, 0 | 7 spikes, 0 | 22 spikes, 0 | 22 spikes, 0 | 8 spikes, 0 | 22 spikes, 0 | 22 spikes, 0 | 22 spikes, 0 | 22 spikes, 0 |
| 2017 243 | 28 spikes, 0.1 | 28 spikes, 0 | 65 spikes, 0 | 16 spikes, 0 | 56 spikes, 0 | 10 spikes, 0 | 28 spikes, 0 | 28 spikes, 0 | 11 spikes, 0 | 28 spikes, 0 | 28 spikes, 0 | 28 spikes, 0 | 28 spikes, 0 |
| 2017 212 | 39 spikes, 0.1 | 39 spikes, 0 | 92 spikes, 0 | 18 spikes, 0.1 | 78 spikes, 0 | 14 spikes, 0 | 39 spikes, 0 | 39 spikes, 0 | 13 spikes, 0 | 39 spikes, 0 | 39 spikes, 0 | 39 spikes, 0 | 39 spikes, 0 |
| 2017 182 | 49 spikes, 0.1 | 49 spikes, 0 | 112 spikes, 0 | 21 spikes, 0 | 96 spikes, 0 | 15 spikes, 0 | 49 spikes, 0 | 49 spikes, 0 | 17 spikes, 0 | 49 spikes, 0 | 49 spikes, 0 | 49 spikes, 0 | 49 spikes, 0 |
| 2017 152 | 60 spikes, 0.1 | 60 spikes, 0 | 136 spikes, 0 | 24 spikes, 0.1 | 119 spikes, 0 | 18 spikes, 0 | 60 spikes, 0 | 60 spikes, 0 | 23 spikes, 0 | 60 spikes, 0 | 60 spikes, 0 | 60 spikes, 0 | 60 spikes, 0 |
| 2017 122 | 70 spikes, 0.1 | 70 spikes, 0.1 | 158 spikes, 0.1 | 30 spikes, 0.2 | 133 spikes, 0.1 | 24 spikes, 0.2 | 70 spikes, 0.1 | 70 spikes, 0.1 | 28 spikes, 0.2 | 70 spikes, 0 | 70 spikes, 0 | 70 spikes, 0 | 70 spikes, 0 |
| 2017 91 | 78 spikes, 0.1 | 78 spikes, 0.1 | 176 spikes, 0.1 | 36 spikes, 0.2 | 149 spikes, 0.1 | 29 spikes, 0.2 | 78 spikes, 0.1 | 78 spikes, 0.1 | 32 spikes, 0.1 | 78 spikes, 0 | 78 spikes, 0 | 78 spikes, 0 | 78 spikes, 0 |
| 2017 84 | 79 spikes, 0.1 | 79 spikes, 0.1 | 182 spikes, 0.1 | 36 spikes, 0.1 | 153 spikes, 0.1 | 31 spikes, 0.2 | 79 spikes, 0.1 | 79 spikes, 0.1 | 34 spikes, 0.2 | 79 spikes, 0 | 79 spikes, 0 | 79 spikes, 0 | 79 spikes, 0 |
| 2017 77 | 83 spikes, 0.1 | 83 spikes, 0.1 | 189 spikes, 0.1 | 37 spikes, 0.1 | 162 spikes, 0.1 | 31 spikes, 0.3 | 83 spikes, 0.1 | 83 spikes, 0.1 | 34 spikes, 0.2 | 83 spikes, 0 | 83 spikes, 0 | 83 spikes, 0 | 83 spikes, 0 |
| 2017 70 | 85 spikes, 0.1 | 85 spikes, 0.1 | 192 spikes, 0.1 | 38 spikes, 0.1 | 165 spikes, 0.1 | 31 spikes, 0.3 | 85 spikes, 0.1 | 85 spikes, 0.1 | 34 spikes, 0.2 | 85 spikes, 0 | 85 spikes, 0 | 85 spikes, 0 | 85 spikes, 0 |
| 2017 63 | 86 spikes, 0.1 | 86 spikes, 0.1 | 195 spikes, 0 | 38 spikes, 0.1 | 166 spikes, 0.1 | 31 spikes, 0.2 | 86 spikes, 0.1 | 86 spikes, 0.1 | 34 spikes, 0.2 | 86 spikes, 0 | 86 spikes, 0 | 86 spikes, 0 | 86 spikes, 0 |
| 2017 56 | 87 spikes, 0.1 | 87 spikes, 0.1 | 202 spikes, 0 | 39 spikes, 0.1 | 170 spikes, 0.1 | 31 spikes, 0.2 | 87 spikes, 0.1 | 87 spikes, 0.1 | 35 spikes, 0.2 | 87 spikes, 0 | 87 spikes, 0 | 87 spikes, 0 | 87 spikes, 0 |
| 2017 49 | 91 spikes, 0.1 | 91 spikes, 0.1 | 209 spikes, 0 | 43 spikes, 0.1 | 177 spikes, 0.1 | 31 spikes, 0.3 | 91 spikes, 0.1 | 91 spikes, 0.1 | 36 spikes, 0.2 | 91 spikes, 0 | 91 spikes, 0 | 91 spikes, 0 | 91 spikes, 0 |
| 2017 42 | 94 spikes, 0.1 | 94 spikes, 0.1 | 215 spikes, 0 | 45 spikes, 0.1 | 185 spikes, 0.1 | 31 spikes, 0.2 | 94 spikes, 0.1 | 94 spikes, 0.1 | 37 spikes, 0.2 | 94 spikes, 0 | 94 spikes, 0 | 94 spikes, 0 | 94 spikes, 0 |
| 2017 35 | 98 spikes, 0.1 | 98 spikes, 0.1 | 222 spikes, 0 | 50 spikes, 0 | 190 spikes, 0 | 32 spikes, 0.2 | 98 spikes, 0.1 | 98 spikes, 0 | 39 spikes, 0.2 | 98 spikes, 0 | 98 spikes, 0 | 98 spikes, 0 | 98 spikes, 0 |
| 2017 28 | 101 spikes, 0.1 | 101 spikes, 0.1 | 228 spikes, 0 | 50 spikes, 0 | 197 spikes, 0 | 33 spikes, 0.2 | 101 spikes, 0.1 | 101 spikes, 0.1 | 41 spikes, 0.2 | 101 spikes, 0 | 101 spikes, 0 | 101 spikes, 0 | 101 spikes, 0 |
| 2017 21 | 102 spikes, 0.1 | 102 spikes, 0.1 | 232 spikes, 0 | 50 spikes, 0 | 199 spikes, 0 | 33 spikes, 0.2 | 102 spikes, 0.1 | 102 spikes, 0 | 41 spikes, 0.2 | 102 spikes, 0 | 102 spikes, 0 | 102 spikes, 0 | 102 spikes, 0 |
| 2017 14 | 108 spikes, 0.1 | 108 spikes, 0.1 | 239 spikes, 0 | 53 spikes, 0 | 207 spikes, 0 | 38 spikes, 0.2 | 108 spikes, 0.1 | 108 spikes, 0 | 46 spikes, 0.2 | 108 spikes, 0 | 108 spikes, 0 | 108 spikes, 0 | 108 spikes, 0 |
| 2017 7 | 108 spikes, 0.1 | 108 spikes, 0.1 | 240 spikes, 0 | 54 spikes, 0.1 | 207 spikes, 0 | 38 spikes, 0.2 | 108 spikes, 0.1 | 108 spikes, 0.1 | 46 spikes, 0.2 | 108 spikes, 0 | 108 spikes, 0 | 108 spikes, 0 | 108 spikes, 0 |
| 2022 334 | 17 spikes, 0.1 | 17 spikes, 0 | 30 spikes, 0 | 7 spikes, 0 | 31 spikes, 0 | 5 spikes, 0 | 17 spikes, 0 | 17 spikes, 0 | 3 spikes, 0 | 17 spikes, 0 | 17 spikes, 0 | 17 spikes, 0 | 17 spikes, 0 |
| 2022 304 | 22 spikes, 0.1 | 22 spikes, 0 | 45 spikes, 0 | 10 spikes, 0 | 45 spikes, 0 | 7 spikes, 0 | 22 spikes, 0 | 22 spikes, 0 | 4 spikes, 0 | 22 spikes, 0 | 22 spikes, 0 | 22 spikes, 0 | 22 spikes, 0 |
| 2022 273 | 35 spikes, 0.1 | 35 spikes, 0 | 69 spikes, 0 | 17 spikes, 0 | 67 spikes, 0 | 13 spikes, 0.1 | 35 spikes, 0 | 35 spikes, 0 | 8 spikes, 0.1 | 35 spikes, 0 | 35 spikes, 0 | 35 spikes, 0 | 35 spikes, 0 |
| 2022 243 | 44 spikes, 0.1 | 44 spikes, 0 | 81 spikes, 0 | 21 spikes, 0 | 81 spikes, 0 | 16 spikes, 0.1 | 44 spikes, 0 | 44 spikes, 0 | 11 spikes, 0.1 | 44 spikes, 0 | 44 spikes, 0 | 44 spikes, 0 | 44 spikes, 0 |
| 2022 212 | 64 spikes, 0.1 | 64 spikes, 0 | 103 spikes, 0 | 34 spikes, 0.1 | 106 spikes, 0 | 24 spikes, 0.1 | 64 spikes, 0 | 64 spikes, 0 | 15 spikes, 0.1 | 64 spikes, 0 | 64 spikes, 0 | 64 spikes, 0 | 64 spikes, 0 |
| 2022 182 | 72 spikes, 0.1 | 72 spikes, 0 | 123 spikes, 0 | 41 spikes, 0 | 123 spikes, 0 | 30 spikes, 0.1 | 72 spikes, 0 | 72 spikes, 0 | 20 spikes, 0.2 | 72 spikes, 0 | 72 spikes, 0 | 72 spikes, 0 | 72 spikes, 0 |
| 2022 152 | 87 spikes, 0.1 | 87 spikes, 0 | 147 spikes, 0 | 44 spikes, 0.1 | 152 spikes, 0 | 35 spikes, 0.1 | 87 spikes, 0 | 87 spikes, 0 | 23 spikes, 0.2 | 87 spikes, 0 | 87 spikes, 0 | 87 spikes, 0 | 87 spikes, 0 |
| 2022 122 | 102 spikes, 0.1 | 102 spikes, 0.1 | 171 spikes, 0 | 58 spikes, 0.1 | 183 spikes, 0 | 44 spikes, 0.1 | 102 spikes, 0.1 | 102 spikes, 0 | 32 spikes, 0.2 | 102 spikes, 0 | 102 spikes, 0 | 102 spikes, 0 | 102 spikes, 0 |
| 2022 91 | 111 spikes, 0.1 | 111 spikes, 0 | 183 spikes, 0 | 64 spikes, 0.1 | 194 spikes, 0 | 46 spikes, 0.1 | 111 spikes, 0 | 111 spikes, 0 | 35 spikes, 0.1 | 111 spikes, 0 | 111 spikes, 0 | 111 spikes, 0 | 111 spikes, 0 |
| 2022 84 | 115 spikes, 0.1 | 115 spikes, 0 | 188 spikes, 0 | 69 spikes, 0.1 | 203 spikes, 0 | 47 spikes, 0.1 | 115 spikes, 0 | 115 spikes, 0 | 36 spikes, 0.1 | 115 spikes, 0 | 115 spikes, 0 | 115 spikes, 0 | 115 spikes, 0 |
| 2022 77 | 121 spikes, 0.1 | 121 spikes, 0 | 200 spikes, 0 | 74 spikes, 0.1 | 209 spikes, 0 | 51 spikes, 0.1 | 121 spikes, 0 | 121 spikes, 0 | 40 spikes, 0.1 | 121 spikes, 0 | 121 spikes, 0 | 121 spikes, 0 | 121 spikes, 0 |
| 2022 70 | 122 spikes, 0.1 | 122 spikes, 0 | 202 spikes, 0 | 76 spikes, 0.1 | 214 spikes, 0 | 51 spikes, 0.1 | 122 spikes, 0 | 122 spikes, 0 | 40 spikes, 0.1 | 122 spikes, 0 | 122 spikes, 0 | 122 spikes, 0 | 122 spikes, 0 |
| 2022 63 | 125 spikes, 0.1 | 125 spikes, 0 | 207 spikes, 0 | 78 spikes, 0.1 | 220 spikes, 0 | 52 spikes, 0.1 | 125 spikes, 0 | 125 spikes, 0 | 42 spikes, 0.1 | 125 spikes, 0 | 125 spikes, 0 | 125 spikes, 0 | 125 spikes, 0 |
| 2022 56 | 128 spikes, 0.1 | 128 spikes, 0 | 211 spikes, 0 | 80 spikes, 0 | 226 spikes, 0 | 52 spikes, 0 | 128 spikes, 0 | 128 spikes, 0 | 43 spikes, 0.1 | 128 spikes, 0 | 128 spikes, 0 | 128 spikes, 0 | 128 spikes, 0 |
| 2022 49 | 133 spikes, 0.1 | 133 spikes, 0 | 217 spikes, 0 | 85 spikes, 0 | 235 spikes, 0 | 52 spikes, 0 | 133 spikes, 0 | 133 spikes, 0 | 43 spikes, 0.1 | 133 spikes, 0 | 133 spikes, 0 | 133 spikes, 0 | 133 spikes, 0 |
| 2022 42 | 135 spikes, 0.1 | 135 spikes, 0 | 220 spikes, 0 | 86 spikes, 0 | 237 spikes, 0 | 52 spikes, 0 | 135 spikes, 0 | 135 spikes, 0 | 43 spikes, 0.1 | 135 spikes, 0 | 135 spikes, 0 | 135 spikes, 0 | 135 spikes, 0 |
| 2022 35 | 135 spikes, 0.1 | 135 spikes, 0 | 221 spikes, 0 | 87 spikes, 0 | 244 spikes, 0 | 52 spikes, 0 | 135 spikes, 0 | 135 spikes, 0 | 43 spikes, 0.1 | 135 spikes, 0 | 135 spikes, 0 | 135 spikes, 0 | 135 spikes, 0 |
| 2022 28 | 137 spikes, 0.1 | 137 spikes, 0 | 223 spikes, 0 | 88 spikes, 0 | 246 spikes, 0 | 52 spikes, 0 | 137 spikes, 0 | 137 spikes, 0 | 43 spikes, 0.1 | 137 spikes, 0 | 137 spikes, 0 | 137 spikes, 0 | 137 spikes, 0 |
| 2022 21 | 137 spikes, 0.1 | 137 spikes, 0 | 228 spikes, 0 | 89 spikes, 0 | 250 spikes, 0 | 52 spikes, 0 | 137 spikes, 0 | 137 spikes, 0 | 43 spikes, 0.1 | 137 spikes, 0 | 137 spikes, 0 | 137 spikes, 0 | 137 spikes, 0 |
| 2022 14 | 139 spikes, 0.1 | 139 spikes, 0 | 231 spikes, 0 | 90 spikes, 0 | 255 spikes, 0 | 52 spikes, 0 | 139 spikes, 0 | 139 spikes, 0 | 43 spikes, 0.1 | 139 spikes, 0 | 139 spikes, 0 | 139 spikes, 0 | 139 spikes, 0 |
| 2022 7 | 140 spikes, 0.1 | 140 spikes, 0 | 234 spikes, 0 | 91 spikes, 0 | 259 spikes, 0 | 52 spikes, 0 | 140 spikes, 0 | 140 spikes, 0 | 43 spikes, 0.1 | 140 spikes, 0 | 140 spikes, 0 | 140 spikes, 0 | 140 spikes, 0 |

## Market blend

The model at the current settings pooled with the last market prices published by each replay's date (model/src/blend.rs), at each weight on the model; mean log-loss of who qualifies and who wins. Like the model, the markets are conditioned on the field that ran: each venue's prices on the runners are scaled to sum to 1. Elections with market history: 2017 (17 replays), 2022 (21 replays). The best weight by the backtest's rule (among the weights at least as good as the model alone on every such election, the lowest total): **1**. The published weight is config/model.yaml's `market_blend.model_weight`, set by the owner.

| Weight on the model | 2017 qualify | 2017 win | 2022 qualify | 2022 win | Total |
|---|---|---|---|---|---|
| 0 | 0.181 | 0.257 | 0.094 | 0.057 | 0.589 |
| 0.1 | 0.181 | 0.247 | 0.094 | 0.052 | 0.574 |
| 0.2 | 0.180 | 0.235 | 0.094 | 0.048 | 0.558 |
| 0.3 | 0.181 | 0.225 | 0.094 | 0.044 | 0.543 |
| 0.4 | 0.181 | 0.212 | 0.094 | 0.040 | 0.527 |
| 0.5 | 0.181 | 0.200 | 0.094 | 0.037 | 0.513 |
| 0.6 | 0.182 | 0.189 | 0.094 | 0.035 | 0.500 |
| 0.7 | 0.182 | 0.179 | 0.094 | 0.032 | 0.488 |
| 0.8 | 0.183 | 0.171 | 0.094 | 0.030 | 0.479 |
| 0.9 | 0.183 | 0.165 | 0.094 | 0.028 | 0.471 |
| 1 | 0.184 | 0.160 | 0.094 | 0.026 | 0.465 |
