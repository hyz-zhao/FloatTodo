<script setup lang="ts">
// 主面板：标题栏（可拖动）+ 日期范围 + 列表 + 新增输入
import { computed, onMounted, ref, watch } from "vue";
import {
  apiAddTodo,
  apiClearCompleted,
  apiCollapseToBall,
  apiDeleteTodo,
  apiListTodos,
  apiQuit,
  apiSaveWindowConfig,
  apiUpdateTodo,
  apiGetConfig,
} from "../api";
import { rangeForPreset } from "../date";
import type { Todo } from "../types";
import DateRangePicker from "./DateRangePicker.vue";
import TodoItem from "./TodoItem.vue";

const start = ref("");
const end = ref("");
const todos = ref<Todo[]>([]);
const draft = ref("");
const listEl = ref<HTMLDivElement | null>(null);
const hasAnyCompleted = computed(() => todos.value.some((t) => t.completed));

async function loadConfig() {
  const cfg = await apiGetConfig();
  if (cfg.last_range_start && cfg.last_range_end) {
    start.value = cfg.last_range_start;
    end.value = cfg.last_range_end;
  } else {
    const t = rangeForPreset("today");
    start.value = t.start;
    end.value = t.end;
  }
}

async function reload() {
  if (!start.value || !end.value) return;
  todos.value = await apiListTodos(start.value, end.value);
  // 记忆当前选择
  const cfg = await apiGetConfig();
  await apiSaveWindowConfig({
    ...cfg,
    last_range_start: start.value,
    last_range_end: end.value,
  });
}

function onRangeChange(s: string, e: string) {
  start.value = s;
  end.value = e;
}

watch([start, end], () => {
  reload();
});

async function onAdd() {
  const text = draft.value.trim();
  if (!text) return;
  await apiAddTodo(text, start.value, end.value);
  draft.value = "";
  await reload();
}

function onKeyEnter() {
  onAdd();
}

async function onToggle(todo: Todo, completed: boolean) {
  await apiUpdateTodo(todo.id, { completed });
  await reload();
}

async function onEdit(todo: Todo, text: string) {
  await apiUpdateTodo(todo.id, { text });
  await reload();
}

async function onRemove(todo: Todo) {
  await apiDeleteTodo(todo.id);
  await reload();
}

async function onClearCompleted() {
  if (!hasAnyCompleted.value) return;
  await apiClearCompleted(start.value, end.value);
  await reload();
}

async function onCollapse() {
  // 主面板大小 / 位置记到配置
  try {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const win = getCurrentWindow();
    const pos = await win.outerPosition();
    const size = await win.outerSize();
    const scale = await win.scaleFactor();
    const cfg = await apiGetConfig();
    await apiSaveWindowConfig({
      ...cfg,
      panel_x: Math.round(pos.x / scale),
      panel_y: Math.round(pos.y / scale),
      panel_width: size.width,
      panel_height: size.height,
    });
  } catch (e) {
    console.error(e);
  }
  await apiCollapseToBall();
}

async function onQuit() {
  if (confirm("确定要退出 FloatTodo 吗？")) {
    await apiQuit();
  }
}

onMounted(async () => {
  await loadConfig();
  await reload();
});
</script>

<template>
  <div class="panel">
    <!-- 标题栏：可拖动（通过 Tauri startDragging） -->
    <header
      class="titlebar"
      @mousedown="async (e) => {
        if (e.button !== 0) return;
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        getCurrentWindow().startDragging().catch(() => {});
      }"
    >
      <div class="title">FloatTodo · 待办</div>
      <div class="actions">
        <button class="icon-btn" title="收起为悬浮球" @click="onCollapse">—</button>
        <button class="icon-btn danger" title="退出" @click="onQuit">×</button>
      </div>
    </header>

    <DateRangePicker :start="start" :end="end" @update="onRangeChange" />

    <div class="list" ref="listEl">
      <div v-if="todos.length === 0" class="empty">
        还没有待办事项，在下方添加一个吧～
      </div>
      <ul v-else>
        <TodoItem
          v-for="t in todos"
          :key="t.id"
          :todo="t"
          @toggle="onToggle"
          @edit="onEdit"
          @remove="onRemove"
        />
      </ul>
    </div>

    <div class="toolbar" v-if="hasAnyCompleted">
      <button class="clear-btn" @click="onClearCompleted">清除已完成</button>
    </div>

    <footer class="composer">
      <input
        v-model="draft"
        class="composer-input"
        placeholder="+ 添加一项待办（回车保存）"
        @keydown.enter="onKeyEnter"
      />
      <button class="add-btn" :disabled="!draft.trim()" @click="onAdd">+</button>
    </footer>
  </div>
</template>

<style scoped>
.panel {
  position: fixed;
  inset: 0;
  display: flex;
  flex-direction: column;
  background: var(--c-bg);
  border-radius: var(--r-lg);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.18);
  overflow: hidden;
}

.titlebar {
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 8px 0 14px;
  background: var(--c-bg-soft);
  border-bottom: 1px solid var(--c-border);
  cursor: grab;
}
.titlebar:active {
  cursor: grabbing;
}
.title {
  font-size: 13px;
  font-weight: 600;
  color: var(--c-text);
}
.actions {
  display: flex;
  gap: 4px;
}
.icon-btn {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  font-size: 14px;
  line-height: 1;
  color: var(--c-text-soft);
}
.icon-btn:hover {
  background: rgba(0, 0, 0, 0.06);
  color: var(--c-text);
}
.icon-btn.danger:hover {
  background: var(--c-danger);
  color: #fff;
}

.list {
  flex: 1;
  overflow-y: auto;
  padding: 6px 6px;
}
.list ul {
  list-style: none;
}
.empty {
  text-align: center;
  color: var(--c-text-dim);
  font-size: 13px;
  padding: 36px 12px;
  line-height: 1.6;
}

.toolbar {
  padding: 4px 12px 0;
  display: flex;
  justify-content: flex-end;
}
.clear-btn {
  font-size: 12px;
  color: var(--c-text-soft);
  padding: 3px 8px;
  border-radius: var(--r-pill);
  background: var(--c-bg-soft);
}
.clear-btn:hover {
  color: var(--c-danger);
  background: #fef2f2;
}

.composer {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 12px 12px;
  border-top: 1px solid var(--c-border);
  background: var(--c-bg-soft);
}
.composer-input {
  flex: 1;
  height: 32px;
  padding: 0 10px;
  font-size: 13px;
  background: #fff;
  border: 1px solid var(--c-border);
  border-radius: var(--r-pill);
}
.composer-input:focus {
  border-color: var(--c-primary);
}
.add-btn {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: var(--c-primary);
  color: #fff;
  font-size: 18px;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}
.add-btn:disabled {
  background: #c8d6e5;
  cursor: not-allowed;
}
.add-btn:not(:disabled):hover {
  background: var(--c-primary-hover);
}
</style>
