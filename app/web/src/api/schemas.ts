// What the API returns, decoded at the boundary: the UI never trusts a payload's shape.
// These mirror schemas/forecast.schema.json, schemas/series.schema.json and the `fr2027 serve`
// endpoints.
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

const PollRow = Schema.Struct({
  poll_id: Schema.String,
  firm: Schema.String,
  sponsor: Schema.NullOr(Schema.String),
  field_start: Schema.String,
  field_end: Schema.String,
  published_at: Schema.String,
  sample_size: Schema.Finite,
  method: Schema.NullOr(Schema.String),
  population: Schema.NullOr(Schema.String),
  round: Schema.Finite,
  scenario_id: Schema.String,
  candidate_id: Schema.String,
  share: Schema.Finite,
  source_url: Schema.String,
  notice_url: Schema.NullOr(Schema.String),
  index_url: Schema.NullOr(Schema.String),
  entry: Schema.String,
  retrieved_at: Schema.String,
  content_hash: Schema.String,
})

export const Polls = Schema.Struct({ rows: Schema.Array(PollRow) })
export type PollRow = typeof PollRow.Type

export const Health = Schema.Struct({
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
  collectors: Schema.Array(
    Schema.Struct({ name: Schema.String, file: Schema.String, modified: Schema.NullOr(Schema.String) }),
  ),
  quarantine: Schema.Record(Schema.String, Schema.Array(Schema.String)),
  warnings: Schema.Array(Schema.String),
})
export type Health = typeof Health.Type
