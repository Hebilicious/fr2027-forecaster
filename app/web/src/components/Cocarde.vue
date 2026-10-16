<template>
  <!-- The cocarde tricolore: blue at the heart, then white, then red. -->
  <svg aria-hidden="true" class="cocarde" viewBox="0 0 32 32">
    <circle class="rouge" cx="16" cy="16" r="15" />
    <circle class="blanc" cx="16" cy="16" r="10.2" />
    <circle class="bleu" cx="16" cy="16" r="5.6" />
    <circle class="rim" cx="16" cy="16" r="15" />
  </svg>
</template>

<style scoped>
.cocarde {
  display: block;
  width: 26px;
  height: 26px;
  flex: none;
  filter: drop-shadow(0 2px 8px rgba(0, 0, 0, 0.5));
}

.rouge {
  fill: var(--flag-rouge);
}

.blanc {
  fill: var(--flag-blanc);
}

.bleu {
  fill: var(--flag-bleu);
}

.rim {
  fill: none;
  stroke: rgba(255, 255, 255, 0.35);
  stroke-width: 0.6;
}
</style>
