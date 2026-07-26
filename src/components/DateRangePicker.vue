<script setup lang="ts">
// 日期范围选择器：快捷预设 + 自定义起始/结束
import { computed, ref, watch } from "vue";
import { describeRange, fmtDate, humanDate, rangeForPreset } from "../date";
import type { RangePreset } from "../types";

const props = defineProps<{
  start: string;
  end: string;
}>();
const emit = defineEmits<{
  (e: "update", start: string, end: string): void;
}>();

// 反推当前是否匹配某个预设
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
  // 起始 > 结束则自动调整结束
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

const rangeLabel = computed(() => describeRange(props.start, props.end));

const presets: { key: RangePreset; label: string }[] = [
  { key: "today", label: "今天" },
  { key: "tomorrow", label: "明天" },
  { key: "this-week", label: "本周" },
  { key: "next-week", label: "下周" },
];
</script>

<template>
  <div class="date-picker">
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
    <div class="custom">
      <label class="field">
        <span>从</span>
        <input type="date" :value="customStart" @input="(e) => onCustomStart((e.target as HTMLInputElement).value)" />
      </label>
      <label class="field">
        <span>至</span>
        <input type="date" :value="customEnd" @input="(e) => onCustomEnd((e.target as HTMLInputElement).value)" />
      </label>
    </div>
    <div class="hint">{{ rangeLabel }}</div>
  </div>
</template>

<style scoped>
.date-picker {
  padding: 10px 12px 8px;
  border-bottom: 1px solid var(--c-border);
  background: var(--c-bg-soft);
}

.presets {
  display: flex;
  gap: 6px;
  margin-bottom: 8px;
}

.preset {
  flex: 1;
  padding: 5px 0;
  font-size: 12px;
  border-radius: var(--r-pill);
  background: #fff;
  color: var(--c-text-soft);
  border: 1px solid var(--c-border);
  transition: all 0.15s ease;
}
.preset:hover {
  color: var(--c-primary);
  border-color: var(--c-primary);
}
.preset.active {
  background: var(--c-primary);
  color: #fff;
  border-color: var(--c-primary);
}

.custom {
  display: flex;
  gap: 8px;
}

.field {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--c-text-soft);
}

.field input {
  flex: 1;
  padding: 4px 6px;
  font-size: 12px;
  border: 1px solid var(--c-border);
  border-radius: var(--r-sm);
  background: #fff;
  color: var(--c-text);
}
.field input:focus {
  border-color: var(--c-primary);
}

.hint {
  margin-top: 6px;
  font-size: 11px;
  color: var(--c-text-dim);
}
</style>
