<script setup lang="ts">
defineProps<{
  text: string;
  hint: string;
}>();
</script>

<template>
  <div class="empty">
    <div class="empty-mark">— § —</div>
    <div class="empty-text">{{ text }}</div>
    <div class="empty-hint">{{ hint }}</div>
  </div>
</template>

<style scoped>
.empty {
  text-align: center;
  padding: 56px 24px 32px;
}
.empty-mark {
  font-family: var(--font-display);
  font-style: italic;
  font-size: 18px;
  color: var(--c-ink-dim);
  letter-spacing: 0.1em;
  margin-bottom: 16px;
}
.empty-text {
  font-family: var(--font-display);
  font-size: 18px;
  color: var(--c-ink-soft);
  margin-bottom: 6px;
  letter-spacing: 0.02em;
}
.empty-hint {
  font-size: 11px;
  color: var(--c-ink-dim);
  letter-spacing: 0.1em;
  text-transform: uppercase;
}
</style>