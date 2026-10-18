<script setup lang="ts">
import { computed, ref } from "vue"
import { useIntlayer, useLocale } from "vue-intlayer"
import type { Forecast } from "../../api/schemas"
import { pathsToVictory, type Opponent, type Path } from "../../explorer"
import { probability } from "../../format"
import CandidateAvatar from "../CandidateAvatar.vue"

const props = defineProps<{ forecast: Forecast }>()
const content = useIntlayer("paths-to-victory")
const common = useIntlayer("common")
const { locale } = useLocale()

/** Opponents shown as their own piece of the bar; the rest share one piece. */
const NAMED = 3

interface Piece {
  readonly key: string
  /** Null for the piece that groups the remaining opponents. */
  readonly opponent: Opponent | null
  readonly pMatchup: number
  readonly pWins: number
}

const paths = computed(() => pathsToVictory(props.forecast))
const scaleMax = computed(() => Math.max(0.05, ...paths.value.map((p) => p.pQualify)))

function pieces(path: Path): Piece[] {
  const named = path.opponents
    .slice(0, NAMED)
    .map((o) => ({ key: o.candidateId, opponent: o, pMatchup: o.pMatchup, pWins: o.pWins }))
  const rest = path.opponents.slice(NAMED)
  const restMatchup = rest.reduce((sum, o) => sum + o.pMatchup, 0)
  if (restMatchup <= 0) return named
  const restWins = rest.reduce((sum, o) => sum + o.pMatchup * o.pWins, 0) / restMatchup
  return [...named, { key: "others", opponent: null, pMatchup: restMatchup, pWins: restWins }]
}

// Width as a share of the track, which spans 0 to the largest chance of reaching round 2.
const width = (p: number) => `${(p / scaleMax.value) * 100}%`

const pointed = ref<{ candidate: string; piece: string } | null>(null)
const isPointed = (path: Path, piece: Piece) =>
  pointed.value?.candidate === path.candidate.candidate_id && pointed.value.piece === piece.key

function pieceText(piece: Piece): string {
  const values = {
    share: probability(piece.pMatchup, locale.value),
    wins: probability(piece.pWins, locale.value),
  }
  return piece.opponent
    ? content.segment({ name: piece.opponent.name, ...values }).value
    : content.othersSegment(values).value
}

function summary(path: Path): string {
  const pointedPiece = pieces(path).find((piece) => isPointed(path, piece))
  if (pointedPiece) return pieceText(pointedPiece)
  const likeliest = path.opponents[0]
  if (path.conversion === null || !likeliest) return ""
  return content.summary({ rate: probability(path.conversion, locale.value), name: likeliest.name }).value
}

const rowLabel = (path: Path) =>
  content.rowLabel({
    name: path.candidate.name,
    reach: probability(path.pQualify, locale.value),
    win: probability(path.pWin, locale.value),
  }).value
</script>

<template>
  <div class="paths">
    <div class="intro">
      <h3>{{ content.title }}</h3>
      <p class="secondary small">{{ content.help }}</p>
      <ul class="legend small" :aria-label="common.legend.value">
        <li><span aria-hidden="true" class="key win" />{{ content.legendWin }}</li>
        <li><span aria-hidden="true" class="key lose" />{{ content.legendLose }}</li>
      </ul>
    </div>

    <div aria-hidden="true" class="head small muted">
      <span />
      <span />
      <span class="num-head">{{ content.headReach }}</span>
      <span class="num-head">{{ content.headWin }}</span>
    </div>
    <ol class="rows">
      <li v-for="(path, i) in paths" :key="path.candidate.candidate_id" class="row" :style="{ '--i': i }">
        <span class="who">
          <CandidateAvatar :id="path.candidate.candidate_id" :name="path.candidate.name" :size="34" />
          <span class="name">{{ path.candidate.name }}</span>
        </span>
        <div class="track-cell">
          <div class="track" role="group" :aria-label="rowLabel(path)">
            <span
              v-for="piece in pieces(path)"
              :key="piece.key"
              class="piece"
              :class="{ pointed: isPointed(path, piece), others: !piece.opponent }"
              :style="{ width: width(piece.pMatchup) }"
              tabindex="0"
              :aria-label="pieceText(piece)"
              @pointerenter="pointed = { candidate: path.candidate.candidate_id, piece: piece.key }"
              @pointerleave="pointed = null"
              @focus="pointed = { candidate: path.candidate.candidate_id, piece: piece.key }"
              @blur="pointed = null"
            >
              <span class="won" :style="{ width: `${piece.pWins * 100}%` }" />
              <span v-if="piece.pMatchup / scaleMax >= 0.14" aria-hidden="true" class="piece-label">{{
                  piece.opponent ? piece.opponent.name : content.others
                }}</span>
            </span>
          </div>
          <p aria-live="polite" class="summary small secondary">{{ summary(path) }}</p>
        </div>
        <span class="num value">{{ probability(path.pQualify, locale) }}</span>
        <span class="num value strong">{{ probability(path.pWin, locale) }}</span>
      </li>
    </ol>

    <details class="data">
      <summary class="small">{{ common.showData }}</summary>
      <div class="table-wrap">
        <table class="small">
          <thead>
            <tr>
              <th scope="col">{{ common.candidate }}</th>
              <th scope="col">{{ common.qualifies }}</th>
              <th scope="col">{{ common.wins }}</th>
              <th scope="col">{{ content.headConversion }}</th>
              <th scope="col">{{ content.headOpponent }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="path in paths" :key="path.candidate.candidate_id">
              <th scope="row">{{ path.candidate.name }}</th>
              <td class="num">{{ probability(path.pQualify, locale) }}</td>
              <td class="num">{{ probability(path.pWin, locale) }}</td>
              <td class="num">{{ path.conversion === null ? "—" : probability(path.conversion, locale) }}</td>
              <td>{{ path.opponents[0]?.name ?? "—" }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </details>
  </div>
</template>

<style scoped>
.paths {
  display: grid;
  /* One column as wide as the panel: the data table scrolls inside it instead of widening the page. */
  grid-template-columns: minmax(0, 1fr);
  gap: 18px;
}

h3 {
  font-family: var(--font-sans);
  font-size: 1.05rem;
  font-weight: 600;
  margin: 0 0 6px;
}

.intro p {
  max-width: 72ch;
}

.legend {
  display: flex;
  flex-wrap: wrap;
  gap: 18px;
  margin: 12px 0 0;
  padding: 0;
  list-style: none;
  color: var(--ink-2);
}

.legend li {
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.key {
  width: 14px;
  height: 10px;
  border-radius: 3px;
}

.key.win {
  background: var(--series);
}

.key.lose {
  background: var(--series-wash);
  box-shadow: inset 0 0 0 1px rgba(88, 124, 241, 0.45);
}

/* Candidate, the bar, then the two numbers it shows. */
.head, .row {
  display: grid;
  grid-template-columns: minmax(10rem, 13rem) minmax(0, 1fr) 4.5rem 4.5rem;
  align-items: start;
  column-gap: 16px;
}

.head {
  padding: 0 0 2px;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  font-size: 0.68rem;
}

.num-head {
  text-align: right;
}

.rows {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
}

.row {
  padding: 12px 0;
  border-bottom: 1px solid var(--grid);
  animation: rise 0.6s var(--ease-out) both;
  animation-delay: calc(var(--i) * 40ms);
}

.row:last-child {
  border-bottom: none;
}

.who {
  display: flex;
  align-items: center;
  gap: 12px;
  min-height: 34px;
}

.name {
  font-weight: 600;
  font-size: 0.95rem;
}

.track-cell {
  display: grid;
  gap: 8px;
  padding-top: 6px;
}

/* The chance of reaching round 2, one piece per opponent with a 2px gap between pieces. */
.track {
  display: flex;
  gap: 2px;
  height: 22px;
}

.piece {
  position: relative;
  min-width: 3px;
  height: 100%;
  border-radius: 4px;
  background: var(--series-wash);
  box-shadow: inset 0 0 0 1px rgba(88, 124, 241, 0.35);
  overflow: hidden;
  cursor: default;
  transition:
    box-shadow 0.2s,
    filter 0.2s;
}

.piece.others {
  background: rgba(243, 239, 230, 0.05);
  box-shadow: inset 0 0 0 1px rgba(243, 239, 230, 0.16);
}

.piece.pointed,
.piece:focus-visible {
  outline: none;
  filter: brightness(1.25);
  box-shadow: inset 0 0 0 1px var(--ink);
}

.won {
  position: absolute;
  inset: 0 auto 0 0;
  background: var(--series);
}

.piece.others .won {
  background: var(--dot);
}

.piece-label {
  position: absolute;
  left: 8px;
  top: 50%;
  transform: translateY(-50%);
  max-width: calc(100% - 12px);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 0.72rem;
  font-weight: 600;
  color: var(--ink);
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.6);
  pointer-events: none;
}

.summary {
  min-height: 1.3em;
  margin: 0;
}

.value {
  text-align: right;
  font-size: 0.95rem;
  color: var(--ink-2);
  padding-top: 7px;
}

.value.strong {
  font-weight: 650;
  color: var(--ink);
}

.data summary {
  cursor: pointer;
  color: var(--ink-2);
}

.data .table-wrap {
  margin-top: 12px;
}

@keyframes rise {
  from {
    opacity: 0;
    transform: translateY(6px);
  }
}

@media (max-width: 760px) {
  .head {
    display: none;
  }

  .row {
    grid-template-columns: 1fr auto auto;
    row-gap: 8px;
  }

  .track-cell {
    grid-column: 1 / -1;
    grid-row: 2;
  }
}

@media (prefers-reduced-motion: reduce) {
  .row {
    animation: none;
  }
}
</style>
