# Runoff anchors: held-out check

A one-off check of model 0.4.0's runoff anchors, run on 2026-10-17 (research/LOG.md, same date). In each past campaign (2002–2022) and at each replay date of `config/backtest.yaml` (182, 91, 30 and 7 days before round 1), with the polls the replay sees: every runoff between two candidates who ran that pollsters had tested is hidden in turn (all its round 2 scenarios removed), then predicted three ways and compared with what its hidden polls said (the polled estimate on the logit scale).

- **Anchor**: the tested runoff with a same-bloc candidate in place of one finalist, shifted by the gap (`Round2Fit::anchored`, `anchor_gap_mean` 0.11, `anchor_gap_sd` 0.23).
- **Transfer**: the bloc transfer table applied to the replay's fitted round 1 shares (runners only).
- **Blend**: the two combined by precision, as the simulation does (`transfer_sd` 0.35).

Runoffs with no possible anchor are left out. 34 held-out runoffs:

| | Root mean squared error (logit) |
|---|---|
| Transfer table alone (model 0.3.0) | 0.357 |
| Anchor alone | 0.291 |
| Blend (model 0.4.0) | 0.246 |

The anchor's stated uncertainty is about right: the mean of (anchor − polled)² / (anchor variance + polled variance) is 1.13, where 1 is exact.

Each held-out runoff (shares of the first-named candidate, %):

```
2002  91d jospin v chirac            polled 48.4  anchor 37.4 (sd 0.27)  transfer 51.6
2002  91d chevenement v chirac       polled 40.0  anchor 45.6 (sd 0.25)  transfer 48.6
2002  30d jospin v chirac            polled 50.7  anchor 37.4 (sd 0.27)  transfer 51.4
2002  30d chevenement v chirac       polled 40.0  anchor 47.9 (sd 0.24)  transfer 48.3
2002   7d jospin v chirac            polled 49.8  anchor 37.4 (sd 0.27)  transfer 50.4
2002   7d chevenement v chirac       polled 40.0  anchor 47.1 (sd 0.24)  transfer 47.4
2007 182d royal v sarkozy            polled 50.7  anchor 62.7 (sd 0.28)  transfer 47.9
2007  91d royal v sarkozy            polled 49.4  anchor 55.0 (sd 0.23)  transfer 48.4
2007  91d sarkozy v jm_le_pen        polled 84.5  anchor 84.3 (sd 0.24)  transfer 66.1
2007  30d royal v sarkozy            polled 47.6  anchor 53.1 (sd 0.21)  transfer 44.7
2007  30d sarkozy v jm_le_pen        polled 84.8  anchor 84.6 (sd 0.21)  transfer 68.2
2007   7d royal v sarkozy            polled 47.2  anchor 53.1 (sd 0.21)  transfer 44.7
2007   7d sarkozy v jm_le_pen        polled 84.8  anchor 84.6 (sd 0.21)  transfer 66.7
2012 182d hollande v sarkozy         polled 60.8  anchor 52.8 (sd 0.28)  transfer 56.3
2012  91d hollande v sarkozy         polled 57.0  anchor 52.8 (sd 0.28)  transfer 51.1
2012  30d hollande v sarkozy         polled 55.2  anchor 52.8 (sd 0.28)  transfer 49.1
2012   7d hollande v sarkozy         polled 54.9  anchor 52.8 (sd 0.28)  transfer 49.0
2017 182d fillon v le_pen            polled 61.8  anchor 55.0 (sd 0.28)  transfer 49.6
2017  91d fillon v le_pen            polled 64.7  anchor 55.0 (sd 0.28)  transfer 56.7
2017  30d fillon v le_pen            polled 56.8  anchor 55.0 (sd 0.28)  transfer 54.2
2017   7d melenchon v le_pen         polled 59.6  anchor 51.8 (sd 0.29)  transfer 56.8
2017   7d fillon v le_pen            polled 55.6  anchor 60.4 (sd 0.28)  transfer 56.1
2022 182d macron v pecresse          polled 55.5  anchor 54.1 (sd 0.28)  transfer 54.1
2022 182d macron v le_pen            polled 55.7  anchor 57.6 (sd 0.28)  transfer 55.2
2022 182d macron v zemmour           polled 59.9  anchor 58.6 (sd 0.27)  transfer 55.8
2022  91d macron v pecresse          polled 53.2  anchor 56.6 (sd 0.26)  transfer 52.2
2022  91d macron v le_pen            polled 56.8  anchor 65.2 (sd 0.26)  transfer 57.5
2022  91d macron v zemmour           polled 62.5  anchor 59.7 (sd 0.26)  transfer 58.1
2022  30d macron v pecresse          polled 59.9  anchor 56.6 (sd 0.26)  transfer 57.6
2022  30d macron v le_pen            polled 57.8  anchor 67.3 (sd 0.24)  transfer 59.4
2022  30d macron v zemmour           polled 64.5  anchor 60.6 (sd 0.24)  transfer 60.2
2022   7d macron v pecresse          polled 63.7  anchor 56.6 (sd 0.26)  transfer 56.8
2022   7d macron v le_pen            polled 54.6  anchor 68.6 (sd 0.24)  transfer 55.4
2022   7d macron v zemmour           polled 65.9  anchor 57.3 (sd 0.24)  transfer 57.3
```
