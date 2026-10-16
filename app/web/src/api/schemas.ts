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
})

const HouseEffect = Schema.Struct({
  firm: Schema.String,
  bloc: Schema.String,
  mean: Schema.Finite,
  sd: Schema.Finite,
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
  changes: Schema.Struct({
    previous: Schema.NullOr(Schema.String),
    new_polls: Schema.Array(Schema.String),
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
  top_topics: Schema.NullOr(Schema.Array(Schema.String)),
  // Stance drops only (null before): posts drawn, those off topic, shares among relevant posts
  // with 95% intervals, and the drop's Cohen's kappa. The sentiment fields above mirror the shares.
  stance_posts_drawn: Schema.NullOr(Schema.Finite),
  stance_off_topic: Schema.NullOr(Schema.Finite),
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
  summary: Schema.String,
  source_url: Schema.String,
  source_title: Schema.NullOr(Schema.String),
  proposed_change: Schema.NullOr(Schema.String),
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
})

export const Signals = Schema.Struct({
  generated_at: Schema.String,
  markets: Markets,
  attention: Attention,
  news: News,
  x: Schema.Struct({ rows: Schema.Array(XRow) }),
  events: Schema.Array(CampaignEvent),
})
export type Signals = typeof Signals.Type
export type Venue = typeof Venue.Type
export type Question = typeof Question.Type
export type Markets = typeof Markets.Type
export type MarketClose = typeof MarketClose.Type
export type Attention = typeof Attention.Type
export type PageViews = typeof PageViews.Type
export type News = typeof News.Type
export type NewsDay = typeof NewsDay.Type
export type Headline = typeof Headline.Type
export type XRow = typeof XRow.Type
export type CampaignEvent = typeof CampaignEvent.Type
