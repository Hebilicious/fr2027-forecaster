// What the API returns, decoded at the boundary: the UI never trusts a payload's shape.
// These mirror schemas/forecast.schema.json, schemas/series.schema.json and the documents
// `fr2027 export` writes to api/<name>.json (and `fr2027 serve` answers at the same paths).
import { Schema } from "effect"

const Interval = Schema.Struct({
  mean: Schema.Finite,
  lo80: Schema.Finite,
  hi80: Schema.Finite,
})

const CandidateForecast = Schema.Struct({
  candidate_id: Schema.String,
  name: Schema.String,
  party: Schema.String,
  bloc: Schema.String,
  status: Schema.String,
  /** The config slot the candidate runs from; forecasts written before it was added lack it. */
  slot: Schema.optional(Schema.String),
  p_run: Schema.Finite,
  simulated: Schema.Boolean,
  p_qualify_r1: Schema.Finite,
  p_win: Schema.Finite,
  r1_share: Schema.NullOr(Interval),
})

const PairForecast = Schema.Struct({
  a: Schema.String,
  b: Schema.String,
  p_matchup: Schema.Finite,
  p_win_given_matchup: Schema.Record(Schema.String, Schema.Finite),
  polled: Schema.Boolean,
  anchor: Schema.optional(Schema.Struct({ a: Schema.String, b: Schema.String })),
})

const HouseEffect = Schema.Struct({
  firm: Schema.String,
  bloc: Schema.String,
  mean: Schema.Finite,
  sd: Schema.Finite,
})

// The settings the run simulated with (forecast files from before model 0.4.0 lack them).
const ErrorSettings = Schema.Struct({
  student_t_df: Schema.Finite,
  bloc_sd: Schema.Finite,
  candidate_sd: Schema.Finite,
  horizon_bloc_sd: Schema.Finite,
  horizon_candidate_sd: Schema.Finite,
})

const Round2Settings = Schema.Struct({
  random_walk_sd: Schema.Finite,
  design_effect: Schema.Finite,
  prior_sd: Schema.Finite,
  error_sd: Schema.Finite,
  horizon_error_sd: Schema.Finite,
  student_t_df: Schema.Finite,
  transfer_sd: Schema.Finite,
  anchor_gap_mean: Schema.Finite,
  anchor_gap_sd: Schema.Finite,
  loyalty: Schema.Finite,
  transfers: Schema.Record(Schema.String, Schema.Record(Schema.String, Schema.Finite)),
})

const Parameters = Schema.Struct({ round1_error: ErrorSettings, round2: Round2Settings })
export type Parameters = typeof Parameters.Type

const SignalSource = Schema.Literals(["attention", "news"])

const Spike = Schema.Struct({
  candidate_id: Schema.String,
  date: Schema.String,
  score: Schema.Finite,
  ratios: Schema.Struct({
    attention: Schema.optionalKey(Schema.Finite),
    news: Schema.optionalKey(Schema.Finite),
  }),
})

/** Event detection and momentum (model/src/signals.rs). Forecasts written before it lack it. */
const ForecastSignals = Schema.Struct({
  sources: Schema.Array(SignalSource),
  shock_sd: Schema.Finite,
  half_life_days: Schema.Finite,
  momentum_weight: Schema.Finite,
  x_momentum_weight: Schema.optionalKey(Schema.Finite),
  tone_momentum_weight: Schema.optionalKey(Schema.Finite),
  spikes: Schema.Array(Spike),
  momentum: Schema.optionalKey(Schema.Array(Schema.Struct({ candidate_id: Schema.String, tilt: Schema.Finite }))),
})

/** Who runs: the hand-set chances and the ballot prices they were pooled with. */
const Ballot = Schema.Struct({
  hand_weight: Schema.Finite,
  market_fetched_at: Schema.NullOr(Schema.String),
  candidates: Schema.Array(
    Schema.Struct({ candidate_id: Schema.String, p_run_hand: Schema.Finite, p_market: Schema.Finite }),
  ),
})

export const Forecast = Schema.Struct({
  schema_version: Schema.Literal("1.0"),
  model_version: Schema.String,
  generated_at: Schema.String,
  as_of: Schema.String,
  inputs_hash: Schema.String,
  seed: Schema.Finite,
  simulations: Schema.Finite,
  election: Schema.Struct({ round1: Schema.String, round2: Schema.String }),
  candidates: Schema.Array(CandidateForecast),
  pairs: Schema.Array(PairForecast),
  aggregation: Schema.Struct({
    random_walk_sd: Schema.Finite,
    design_effect: Schema.Finite,
    scenario_noise_share: Schema.Finite,
    log_likelihood: Schema.Finite,
    polls_used: Schema.Finite,
    scenarios_used: Schema.Finite,
    round2_pairs_polled: Schema.Finite,
    last_field_end: Schema.NullOr(Schema.String),
    poll_ids: Schema.Array(Schema.String),
    house_effects: Schema.Array(HouseEffect),
  }),
  parameters: Schema.optional(Parameters),
  signals: Schema.optionalKey(ForecastSignals),
  ballot: Schema.optionalKey(Ballot),
  /** The chances at several strengths of the news and social signals (0: polls alone, 1: published). */
  signal_levels: Schema.optionalKey(
    Schema.Array(
      Schema.Struct({
        scale: Schema.Finite,
        candidates: Schema.Array(
          Schema.Struct({ candidate_id: Schema.String, p_qualify_r1: Schema.Finite, p_win: Schema.Finite }),
        ),
      }),
    ),
  ),
  changes: Schema.Struct({
    previous: Schema.NullOr(Schema.String),
    new_polls: Schema.Array(Schema.String),
    /** Forecasts written before event detection lack it. */
    new_spikes: Schema.optionalKey(Schema.Array(Schema.Struct({ candidate_id: Schema.String, date: Schema.String }))),
    p_win_delta: Schema.Array(Schema.Struct({ candidate_id: Schema.String, delta: Schema.Finite })),
  }),
  warnings: Schema.Array(Schema.String),
})
export type Forecast = typeof Forecast.Type
export type CandidateForecast = typeof CandidateForecast.Type
export type PairForecast = typeof PairForecast.Type

const SeriesPoint = Schema.Struct({
  date: Schema.String,
  candidate_id: Schema.String,
  mean: Schema.Finite,
  lo80: Schema.Finite,
  hi80: Schema.Finite,
})

export const Series = Schema.Struct({
  schema_version: Schema.Literal("1.0"),
  forecast: Schema.String,
  points: Schema.Array(SeriesPoint),
})
export type Series = typeof Series.Type
export type SeriesPoint = typeof SeriesPoint.Type

const PollScenario = Schema.Struct({
  round: Schema.Finite,
  scenario_id: Schema.String,
  /** Percent, by candidate id. */
  shares: Schema.Record(Schema.String, Schema.Finite),
})

// One poll with its scenarios. The full rows, with provenance, are in data/clean/polls.csv.
const Poll = Schema.Struct({
  poll_id: Schema.String,
  firm: Schema.String,
  sponsor: Schema.NullOr(Schema.String),
  field_start: Schema.String,
  field_end: Schema.String,
  published_at: Schema.String,
  sample_size: Schema.Finite,
  entry: Schema.String,
  source_url: Schema.String,
  notice_url: Schema.NullOr(Schema.String),
  scenarios: Schema.Array(PollScenario),
})

export const Polls = Schema.Struct({ polls: Schema.Array(Poll) })
export type Poll = typeof Poll.Type

/** How often a source is expected to update; the UI translates it. */
const Cadence = Schema.Literals(["on_publication", "hourly", "12h", "daily"])

const Collector = Schema.Struct({
  name: Schema.String,
  /** A timestamp, or a calendar date for sources dated by publication (polls). */
  updated: Schema.NullOr(Schema.String),
  cadence: Cadence,
  stale: Schema.Boolean,
})

export const Health = Schema.Struct({
  /** When the documents were built: the hourly rebuild changes it, so the UI polls it. */
  checked_at: Schema.String,
  latest_forecast: Schema.NullOr(
    Schema.Struct({
      file: Schema.String,
      generated_at: Schema.String,
      as_of: Schema.String,
      last_field_end: Schema.NullOr(Schema.String),
      polls_used: Schema.Finite,
    }),
  ),
  forecast_count: Schema.Finite,
  collectors: Schema.Array(Collector),
  quarantine: Schema.Record(Schema.String, Schema.Array(Schema.String)),
  warnings: Schema.Array(Schema.String),
})
export type Health = typeof Health.Type
export type Collector = typeof Collector.Type
export type Cadence = typeof Cadence.Type

// Signals: what is recorded beside the polls but not read by the model.
const Venue = Schema.Literals(["polymarket", "kalshi"])
const Question = Schema.Literals(["win", "qualify", "on_ballot"])

const MarketSource = Schema.Struct({
  id: Schema.String,
  venue: Venue,
  question: Question,
  url: Schema.String,
  status: Schema.Literals(["ok", "error"]),
  error: Schema.NullOr(Schema.String),
})

const MarketQuote = Schema.Struct({
  candidate_id: Schema.String,
  venue: Venue,
  question: Question,
  price: Schema.NullOr(Schema.Finite),
  bid: Schema.NullOr(Schema.Finite),
  ask: Schema.NullOr(Schema.Finite),
  volume: Schema.NullOr(Schema.Finite),
})

const UnmatchedQuote = Schema.Struct({
  label: Schema.String,
  venue: Venue,
  question: Question,
  price: Schema.NullOr(Schema.Finite),
})

/** The day's last price for one venue, question and candidate. */
const MarketClose = Schema.Struct({
  date: Schema.String,
  venue: Venue,
  question: Question,
  candidate_id: Schema.String,
  price: Schema.NullOr(Schema.Finite),
})

const PageViews = Schema.Struct({ date: Schema.String, candidate_id: Schema.String, views: Schema.Finite })

const Feed = Schema.Struct({
  id: Schema.String,
  outlet: Schema.NullOr(Schema.String),
  status: Schema.String,
  items: Schema.Finite,
  error: Schema.NullOr(Schema.String),
})

const NewsDay = Schema.Struct({ date: Schema.String, candidate_id: Schema.String, items: Schema.Finite })

/**
 * GDELT's French-language articles naming a candidate on a day, out of the day's total, with the mean tone of those
 * articles and of every French article that day (absent from older API builds, null before tone was kept).
 */
const NewsVolumeDay = Schema.Struct({
  date: Schema.String,
  candidate_id: Schema.String,
  articles: Schema.Finite,
  total_articles: Schema.Finite,
  tone: Schema.optionalKey(Schema.NullOr(Schema.Finite)),
  total_tone: Schema.optionalKey(Schema.NullOr(Schema.Finite)),
})

const Headline = Schema.Struct({
  published_at: Schema.String,
  outlet: Schema.String,
  title: Schema.String,
  url: Schema.String,
  candidate_ids: Schema.Array(Schema.String),
})

const XRow = Schema.Struct({
  window_start: Schema.String,
  window_end: Schema.String,
  candidate_id: Schema.String,
  mentions: Schema.NullOr(Schema.Finite),
  mentions_method: Schema.NullOr(Schema.Literals(["x_counts_endpoint", "native_count", "sample_estimate"])),
  sample_size: Schema.NullOr(Schema.Finite),
  unique_authors: Schema.NullOr(Schema.Finite),
  engagement: Schema.NullOr(Schema.Finite),
  sentiment_pos: Schema.NullOr(Schema.Finite),
  sentiment_neg: Schema.NullOr(Schema.Finite),
  sentiment_neu: Schema.NullOr(Schema.Finite),
  bot_share_estimate: Schema.NullOr(Schema.Finite),
  // The stance rubric the drop follows; null for the first drops, read for sentiment. Absent from older API builds.
  rubric_version: Schema.optionalKey(Schema.NullOr(Schema.String)),
  top_topics: Schema.NullOr(Schema.Array(Schema.String)),
  // Stance drops only (null before): posts drawn, those off topic, shares among relevant posts
  // with 95% intervals, and the drop's Cohen's kappa. The sentiment fields above mirror the shares.
  stance_posts_drawn: Schema.NullOr(Schema.Finite),
  stance_off_topic: Schema.NullOr(Schema.Finite),
  // The counts behind the shares, so several windows can be pooled. Absent from older API builds.
  stance_support: Schema.optionalKey(Schema.NullOr(Schema.Finite)),
  stance_oppose: Schema.optionalKey(Schema.NullOr(Schema.Finite)),
  stance_neutral: Schema.optionalKey(Schema.NullOr(Schema.Finite)),
  support_share: Schema.NullOr(Schema.Finite),
  support_low: Schema.NullOr(Schema.Finite),
  support_high: Schema.NullOr(Schema.Finite),
  oppose_share: Schema.NullOr(Schema.Finite),
  oppose_low: Schema.NullOr(Schema.Finite),
  oppose_high: Schema.NullOr(Schema.Finite),
  neutral_share: Schema.NullOr(Schema.Finite),
  neutral_low: Schema.NullOr(Schema.Finite),
  neutral_high: Schema.NullOr(Schema.Finite),
  drop_kappa: Schema.NullOr(Schema.Finite),
})

const CampaignEvent = Schema.Struct({
  date: Schema.String,
  event_id: Schema.String,
  kind: Schema.String,
  candidate_ids: Schema.Array(Schema.String),
  /** French and English; a plain string from an older API build is shown as is. */
  summary: Schema.Union([Schema.String, Schema.Struct({ fr: Schema.String, en: Schema.String })]),
  source_url: Schema.String,
  source_title: Schema.NullOr(Schema.String),
  reported_by: Schema.String,
})

const Markets = Schema.Struct({
  fetched_at: Schema.NullOr(Schema.String),
  sources: Schema.Array(MarketSource),
  latest: Schema.Array(MarketQuote),
  unmatched: Schema.Array(UnmatchedQuote),
  history: Schema.Array(MarketClose),
})

const Attention = Schema.Struct({ fetched_at: Schema.NullOr(Schema.String), rows: Schema.Array(PageViews) })

const News = Schema.Struct({
  fetched_at: Schema.NullOr(Schema.String),
  feeds: Schema.Array(Feed),
  daily: Schema.Array(NewsDay),
  headlines: Schema.Array(Headline),
  // Absent from older API builds.
  volume: Schema.optionalKey(Schema.Array(NewsVolumeDay)),
})

const ForwardFlag = Schema.Struct({
  detected_at: Schema.String,
  kind: Schema.Literals(["spike", "x_shift"]),
  candidate_id: Schema.String,
  date: Schema.String,
  /** Spike: mean log ratio to the baseline. X shift: change in the support share. */
  score: Schema.Finite,
  detail: Schema.String,
  /** The candidate's latent share move over the next `horizon_days`; null while pending. */
  change: Schema.NullOr(Schema.Finite),
})

/** The forward log of signal flags, scored against the latent share history. */
const Forward = Schema.Struct({
  horizon_days: Schema.Finite,
  flags: Schema.Array(ForwardFlag),
  summary: Schema.Struct({
    logged: Schema.Finite,
    spikes_scored: Schema.Finite,
    spike_mean_move: Schema.NullOr(Schema.Finite),
    typical_mean_move: Schema.NullOr(Schema.Finite),
    x_shifts_scored: Schema.Finite,
    x_shifts_agreeing: Schema.Finite,
  }),
})

export const Signals = Schema.Struct({
  generated_at: Schema.String,
  markets: Markets,
  attention: Attention,
  news: News,
  x: Schema.Struct({ rows: Schema.Array(XRow) }),
  events: Schema.Array(CampaignEvent),
  forward: Forward,
})
export type Signals = typeof Signals.Type
export type Forward = typeof Forward.Type
export type ForwardFlag = typeof ForwardFlag.Type
export type ForecastSignals = typeof ForecastSignals.Type
export type Venue = typeof Venue.Type
export type Question = typeof Question.Type
export type Markets = typeof Markets.Type
export type MarketClose = typeof MarketClose.Type
export type Attention = typeof Attention.Type
export type PageViews = typeof PageViews.Type
export type News = typeof News.Type
export type NewsDay = typeof NewsDay.Type
export type NewsVolumeDay = typeof NewsVolumeDay.Type
export type Headline = typeof Headline.Type
export type XRow = typeof XRow.Type
export type CampaignEvent = typeof CampaignEvent.Type

// Every forecast run's chances, oldest first (`/api/history.json`).
const HistoryRun = Schema.Struct({
  file: Schema.String,
  as_of: Schema.String,
  generated_at: Schema.String,
  candidates: Schema.Array(
    Schema.Struct({
      candidate_id: Schema.String,
      p_win: Schema.NullOr(Schema.Finite),
      p_qualify_r1: Schema.NullOr(Schema.Finite),
    }),
  ),
})

export const History = Schema.Struct({ runs: Schema.Array(HistoryRun) })
export type History = typeof History.Type
export type HistoryRun = typeof HistoryRun.Type

// How the model did when replayed on past elections (`/api/track-record.json`, from the backtest).
const HorizonRecord = Schema.Struct({
  days: Schema.Finite,
  replays: Schema.Finite,
  /** Share of results that fell inside the model's 80% range. */
  coverage80: Schema.Finite,
  share_mae_points: Schema.Finite,
})

const ReliabilityBin = Schema.Struct({
  lo: Schema.Finite,
  hi: Schema.Finite,
  events: Schema.Finite,
  mean_forecast: Schema.NullOr(Schema.Finite),
  observed: Schema.NullOr(Schema.Finite),
})

export const TrackRecord = Schema.Struct({
  model_version: Schema.NullOr(Schema.String),
  elections: Schema.Array(Schema.Finite),
  by_horizon: Schema.Array(HorizonRecord),
  reliability: Schema.Array(ReliabilityBin),
})
export type TrackRecord = typeof TrackRecord.Type
export type ReliabilityBin = typeof ReliabilityBin.Type

const Questions = Schema.Struct({ win: Schema.Finite, qualify: Schema.Finite, run: Schema.Finite })
const MarketQuestions = Schema.Struct({
  win: Schema.NullOr(Schema.Finite),
  qualify: Schema.NullOr(Schema.Finite),
  run: Schema.NullOr(Schema.Finite),
})

const BlendCandidate = Schema.Struct({
  candidate_id: Schema.String,
  model: Questions,
  /** Venue-averaged prices; for `win`, each venue's prices scaled to sum to 1 first. */
  market: MarketQuestions,
  blend: Questions,
})

/** `api/blend.json`: the latest forecast pooled with the newest market snapshot (model/src/blend.rs). */
export const Blend = Schema.Struct({
  forecast: Schema.NullOr(Schema.String),
  /** The published weight on the model; 1 is the model alone. */
  model_weight: Schema.Finite,
  /** Probabilities are clipped to [floor, 1 − floor] before taking logs. */
  probability_floor: Schema.Finite,
  market_fetched_at: Schema.NullOr(Schema.String),
  /** False when the newest snapshot is too old to blend. */
  market_used: Schema.Boolean,
  /** The markets' share, for who wins, on people outside the field. */
  market_others_win: Schema.Finite,
  candidates: Schema.Array(BlendCandidate),
})
export type Blend = typeof Blend.Type
export type BlendCandidate = typeof BlendCandidate.Type
