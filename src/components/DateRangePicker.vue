<script setup lang="ts">
// 日期范围选择器：编辑感排版，衬线大数字 + 细描边 chip
import { computed, ref, watch } from "vue";
import { describeRange, fmtDate, parseDate, rangeForPreset } from "../date";
import type { RangePreset } from "../types";

const props = defineProps<{
  start: string;
  end: string;
}>();
const emit = defineEmits<{
  (e: "update", start: string, end: string): void;
}>();

function detectPreset(s: string, e: string): RangePreset {
  for (const p of ["today", "tomorrow", "this-week", "next-week"] as const) {
    const r = rangeForPreset(p);
    if (r.start === s && r.end === e) return p;
  }
  return "custom";
}

const preset = ref<RangePreset>(detectPreset(props.start, props.end));
const customStart = ref(props.start);
const customEnd = ref(props.end);

watch(
  () => [props.start, props.end] as const,
  ([s, e]) => {
    preset.value = detectPreset(s, e);
    customStart.value = s;
    customEnd.value = e;
  }
);

function applyPreset(p: RangePreset) {
  preset.value = p;
  const r = rangeForPreset(p, customStart.value, customEnd.value);
  customStart.value = r.start;
  customEnd.value = r.end;
  emit("update", r.start, r.end);
}

function onCustomStart(v: string) {
  customStart.value = v;
  preset.value = "custom";
  let end = customEnd.value;
  if (end < customStart.value) end = customStart.value;
  customEnd.value = end;
  emit("update", customStart.value, end);
}

function onCustomEnd(v: string) {
  customEnd.value = v;
  preset.value = "custom";
  let start = customStart.value;
  if (start > customEnd.value) start = customEnd.value;
  customStart.value = start;
  emit("update", start, customEnd.value);
}

const startObj = computed(() => parseDate(props.start) ?? new Date());
const endObj = computed(() => parseDate(props.end) ?? new Date());
const sameDay = computed(() => props.start === props.end);
const daySpan = computed(() => {
  const ms = endObj.value.getTime() - startObj.value.getTime();
  return Math.round(ms / 86400000) + 1;
});

const presets: { key: RangePreset; label: string }[] = [
  { key: "today", label: "今天" },
  { key: "tomorrow", label: "明天" },
  { key: "this-week", label: "本周" },
  { key: "next-week", label: "下周" },
];
</script>

<template>
  <div class="date-picker">
    <!-- 大数字日期头（编辑感） -->
    <div class="date-head">
      <div class="date-num">
        <span class="num">{{ startObj.getDate() }}</span>
        <div class="meta">
          <span class="month">{{ startObj.getMonth() + 1 }}月</span>
          <span class="year">{{ startObj.getFullYear() }}</span>
        </div>
      </div>
      <div v-if="!sameDay" class="date-arrow">
        <span class="dash"></span>
        <span class="span">共 {{ daySpan }} 天</span>
      </div>
      <div v-if="!sameDay" class="date-num small">
        <span class="num">{{ endObj.getDate() }}</span>
        <div class="meta">
          <span class="month">{{ endObj.getMonth() + 1 }}月</span>
        </div>
      </div>
    </div>

    <!-- 快捷选项 -->
    <div class="presets">
      <button
        v-for="p in presets"
        :key="p.key"
        class="preset"
        :class="{ active: preset === p.key }"
        @click="applyPreset(p.key)"
      >
        {{ p.label }}
      </button>
    </div>

    <!-- 自定义日期 -->
    <div class="custom">
      <label class="field">
        <span class="label">起</span>
        <input
          type="date"
          :value="customStart"
          @input="(e) => onCustomStart((e.target as HTMLInputElement).value)"
        />
      </label>
      <span class="divider">—</span>
      <label class="field">
        <span class="label">止</span>
        <input
          type="date"
          :value="customEnd"
          @input="(e) => onCustomEnd((e.target as HTMLInputElement).value)"
        />
      </label>
    </div>
  </div>
</template>

<style scoped>
.date-picker {
  padding: 18px 18px 14px;
  border-bottom: 1px solid var(--c-line);
  background: var(--c-paper);
  position: relative;
}

.date-picker::after {
  content: "";
  position: absolute;
  left: 18px;
  right: 18px;
  bottom: -1px;
  height: 1px;
  background: linear-gradient(
    to right,
    transparent,
    var(--c-line) 20%,
    var(--c-line) 80%,
    transparent
  );
}

/* —— 大数字日期 —— */
.date-head {
  display: flex;
  align-items: baseline;
  gap: 10px;
  margin-bottom: 14px;
}

.date-num {
  display: flex;
  align-items: baseline;
  gap: 6px;
}
.date-num.small .num {
  font-size: 24px;
}
.num {
  font-family: var(--font-display);
  font-weight: 500;
  font-size: 36px;
  color: var(--c-ink);
  line-height: 1;
  letter-spacing: -0.03em;
  font-feature-settings: "lnum", "tnum";
}
.meta {
  display: flex;
  flex-direction: column;
  gap: 1px;
}
.month {
  font-family: var(--font-body);
  font-size: 10px;
  font-weight: 500;
  color: var(--c-ink-soft);
  text-transform: uppercase;
  letter-spacing: 0.08em;
}
.year {
  font-family: var(--font-body);
  font-size: 9px;
  color: var(--c-ink-dim);
  letter-spacing: 0.1em;
}

.date-arrow {
  display: flex;
  align-items: center;
  gap: 6px;
  flex: 1;
  color: var(--c-ink-dim);
  font-size: 10px;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}
.dash {
  flex: 1;
  height: 1px;
  background: var(--c-line);
}
.span {
  white-space: nowrap;
}

/* —— 预设 chip —— */
.presets {
  display: flex;
  gap: 6px;
  margin-bottom: 12px;
}
.preset {
  flex: 1;
  padding: 6px 0;
  font-size: 11.5px;
  font-weight: 500;
  letter-spacing: 0.04em;
  border-radius: var(--r-pill);
  background: transparent;
  color: var(--c-ink-soft);
  border: 1px solid var(--c-line-2);
  transition: all 0.2s var(--ease-out);
}
.preset:hover {
  color: var(--c-ink);
  border-color: var(--c-ink);
}
.preset.active {
  background: var(--c-ink);
  color: var(--c-paper);
  border-color: var(--c-ink);
}

/* —— 自定义日期 —— */
.custom {
  display: flex;
  align-items: center;
  gap: 8px;
}
.field {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 6px;
}
.label {
  font-family: var(--font-display);
  font-style: italic;
  font-size: 12px;
  color: var(--c-ink-dim);
  width: 14px;
}
.field input {
  flex: 1;
  padding: 4px 6px;
  font-size: 12px;
  font-family: var(--font-mono);
  border: none;
  border-bottom: 1px dashed var(--c-line-2);
  color: var(--c-ink);
  background: transparent;
  letter-spacing: 0.02em;
}
.field input:focus {
  border-bottom-color: var(--c-accent);
  border-bottom-style: solid;
}
.divider {
  color: var(--c-ink-dim);
  font-size: 11px;
}
</style>
