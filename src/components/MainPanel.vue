<script setup lang="ts">
// 主面板：编辑感杂志排版，衬线品牌 + 元信息 + 日期 + 列表 + 输入
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";
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
import HistoryView from "./HistoryView.vue";
import TodoItem from "./TodoItem.vue";
import { useToast } from "../composables/useToast";

const activeTab = ref<"editor" | "history">("editor");

const { show } = useToast();

const start = ref("");
const end = ref("");
const todos = ref<Todo[]>([]);
const draft = ref("");
const listEl = ref<HTMLDivElement | null>(null);
const hasAnyCompleted = computed(() => todos.value.some((t) => t.completed));
const pendingCount = computed(() => todos.value.filter((t) => !t.completed).length);

async function loadConfig() {
  try {
    const cfg = await apiGetConfig();
    if (cfg.last_range_start && cfg.last_range_end) {
      start.value = cfg.last_range_start;
      end.value = cfg.last_range_end;
    } else {
      const t = rangeForPreset("today");
      start.value = t.start;
      end.value = t.end;
    }
  } catch (e) {
    console.error("加载配置失败", e);
    show("加载配置失败");
  }
}

async function reload() {
  if (!start.value || !end.value) return;
  try {
    todos.value = await apiListTodos(start.value, end.value);
    const cfg = await apiGetConfig();
    await apiSaveWindowConfig({
      ...cfg,
      last_range_start: start.value,
      last_range_end: end.value,
    });
  } catch (e) {
    console.error("加载待办列表失败", e);
    show("加载待办列表失败");
  }
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
  try {
    await apiAddTodo(text, start.value, end.value);
    draft.value = "";
    await reload();
  } catch (e) {
    console.error("添加待办失败", e);
    show("添加待办失败");
  }
}

function onKeyEnter() {
  onAdd();
}

async function onToggle(todo: Todo, completed: boolean) {
  try {
    await apiUpdateTodo(todo.id, { completed });
    await reload();
  } catch (e) {
    console.error("更新待办状态失败", e);
    show("更新待办状态失败");
  }
}

async function onEdit(todo: Todo, text: string) {
  try {
    await apiUpdateTodo(todo.id, { text });
    await reload();
  } catch (e) {
    console.error("编辑待办失败", e);
    show("编辑待办失败");
  }
}

async function onRemove(todo: Todo) {
  try {
    await apiDeleteTodo(todo.id);
    await reload();
  } catch (e) {
    console.error("删除待办失败", e);
    show("删除待办失败");
  }
}

async function onClearCompleted() {
  if (!hasAnyCompleted.value) return;
  try {
    await apiClearCompleted(start.value, end.value);
    await reload();
  } catch (e) {
    console.error("清除已完成待办失败", e);
    show("清除已完成待办失败");
  }
}

async function onCollapse() {
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
    console.error("保存窗口配置失败", e);
  }
  try {
    await apiCollapseToBall();
  } catch (e) {
    console.error("收起为悬浮球失败", e);
    show("收起为悬浮球失败");
  }
}

async function onQuit() {
  if (confirm("确定要退出 FloatTodo 吗？")) {
    try {
      await apiQuit();
    } catch (e) {
      console.error("退出应用失败", e);
      show("退出应用失败");
    }
  }
}

const now = ref(new Date());
let timer: ReturnType<typeof setInterval> | null = null;

onMounted(async () => {
  await loadConfig();
  await reload();
  timer = setInterval(() => {
    now.value = new Date();
  }, 60_000);
});

onBeforeUnmount(() => {
  if (timer) {
    clearInterval(timer);
    timer = null;
  }
});

const issueNo = computed(() => {
  // 用 ISO 周数生成「期号」
  const d = now.value;
  const target = new Date(d.valueOf());
  const dayNr = (d.getDay() + 6) % 7;
  target.setDate(target.getDate() - dayNr + 3);
  const firstThursday = target.valueOf();
  target.setMonth(0, 1);
  if (target.getDay() !== 4) {
    target.setMonth(0, 1 + ((4 - target.getDay()) + 7) % 7);
  }
  const week = 1 + Math.ceil((firstThursday - target.valueOf()) / 604800000);
  return `Nº ${String(week).padStart(2, "0")}`;
});

const dateStr = computed(() => {
  const d = now.value;
  return `${d.getFullYear()}.${String(d.getMonth() + 1).padStart(2, "0")}.${String(d.getDate()).padStart(2, "0")}`;
});
</script>

<template>
  <div class="panel">
    <!-- 标题栏 -->
    <header
      class="titlebar"
      @mousedown="async (e) => {
        if (e.button !== 0) return;
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        getCurrentWindow().startDragging().catch(() => {});
      }"
    >
      <div class="brand">
        <span class="logo">F</span>
        <span class="name">FloatTodo</span>
        <span class="tag">·</span>
        <span class="meta-issue">{{ issueNo }}</span>
      </div>
      <div class="actions">
        <button
          class="icon-btn"
          title="收起为悬浮球"
          @mousedown.stop
          @click.stop="onCollapse"
        >
          <svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true">
            <path d="M3 9h10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
          </svg>
        </button>
        <button
          class="icon-btn danger"
          title="退出"
          @mousedown.stop
          @click.stop="onQuit"
        >
          <svg viewBox="0 0 16 16" width="11" height="11" aria-hidden="true">
            <path
              d="M4 4l8 8M12 4l-8 8"
              stroke="currentColor"
              stroke-width="1.5"
              stroke-linecap="round"
            />
          </svg>
        </button>
      </div>
    </header>

    <!-- Tab 切换 -->
    <nav class="tab-bar">
      <button
        :class="{ active: activeTab === 'editor' }"
        @click="activeTab = 'editor'"
      >
        待办
      </button>
      <button
        :class="{ active: activeTab === 'history' }"
        @click="activeTab = 'history'"
      >
        记录
      </button>
    </nav>

    <template v-if="activeTab === 'editor'">
      <!-- 元信息条（编辑感：期号 · 日期 · 计数） -->
      <div class="meta-bar">
      <span class="meta-item">{{ dateStr }}</span>
      <span class="meta-sep">·</span>
      <span class="meta-item count">
        <em>{{ pendingCount }}</em> 项待办
      </span>
    </div>

    <DateRangePicker :start="start" :end="end" @update="onRangeChange" />

    <!-- 列表 -->
    <div class="list" ref="listEl">
      <div v-if="todos.length === 0" class="empty">
        <div class="empty-mark">— § —</div>
        <div class="empty-text">本日尚无安排</div>
        <div class="empty-hint">在下方添加第一项</div>
      </div>
      <ul v-else class="todo-list">
        <TodoItem
          v-for="(t, i) in todos"
          :key="t.id"
          :todo="t"
          :style="{ animationDelay: `${i * 40}ms` }"
          @toggle="onToggle"
          @edit="onEdit"
          @remove="onRemove"
        />
      </ul>
    </div>

    <!-- 工具条：清除已完成 -->
    <div class="toolbar" v-if="hasAnyCompleted">
      <button class="clear-btn" @click="onClearCompleted">
        <span>清除已完成</span>
        <span class="arrow">→</span>
      </button>
    </div>

    <!-- 输入区 -->
    <footer class="composer">
      <span class="composer-mark" aria-hidden="true">+</span>
      <input
        v-model="draft"
        class="composer-input"
        placeholder="写下这一项……"
        @keydown.enter="onKeyEnter"
      />
      <span class="composer-hint" v-if="!draft">↵</span>
      <span class="composer-hint" v-else>↵</span>
    </footer>
    </template>

    <HistoryView v-if="activeTab === 'history'" />
  </div>
</template>

<style scoped>
.panel {
  position: fixed;
  inset: 0;
  display: flex;
  flex-direction: column;
  background: var(--c-paper);
  border-radius: var(--r-xl);
  box-shadow: var(--c-shadow);
  overflow: hidden;
  border: 1px solid var(--c-line);
}

/* —— 标题栏 —— */
.titlebar {
  height: 42px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 10px 0 16px;
  background: var(--c-paper);
  border-bottom: 1px solid var(--c-line);
  cursor: grab;
  flex-shrink: 0;
}
.titlebar:active {
  cursor: grabbing;
}

.brand {
  display: flex;
  align-items: center;
  gap: 8px;
}
.logo {
  font-family: var(--font-display);
  font-style: italic;
  font-weight: 600;
  font-size: 18px;
  color: var(--c-paper);
  background: var(--c-ink);
  width: 24px;
  height: 24px;
  border-radius: 50%;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  line-height: 1;
  letter-spacing: -0.02em;
}
.name {
  font-family: var(--font-display);
  font-style: italic;
  font-weight: 500;
  font-size: 16px;
  color: var(--c-ink);
  letter-spacing: -0.01em;
}
.tag {
  color: var(--c-ink-dim);
  font-size: 10px;
}
.meta-issue {
  font-family: var(--font-body);
  font-size: 10px;
  font-weight: 500;
  color: var(--c-ink-soft);
  letter-spacing: 0.1em;
  text-transform: uppercase;
}

.actions {
  display: flex;
  gap: 2px;
}
.icon-btn {
  width: 26px;
  height: 26px;
  border-radius: 50%;
  color: var(--c-ink-soft);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s var(--ease-out);
}
.icon-btn:hover {
  background: var(--c-ink);
  color: var(--c-paper);
}
.icon-btn.danger:hover {
  background: var(--c-accent);
}

/* —— Tab 切换栏 —— */
.tab-bar {
  display: flex;
  gap: 0;
  padding: 0 16px;
  background: var(--c-paper);
  border-bottom: 1px solid var(--c-line);
  flex-shrink: 0;
}
.tab-bar button {
  font-family: var(--font-body);
  font-size: 11px;
  font-weight: 500;
  letter-spacing: 0.08em;
  padding: 8px 16px 7px;
  background: transparent;
  color: var(--c-ink-soft);
  border-bottom: 2px solid transparent;
  transition: all 0.2s var(--ease-out);
  margin-bottom: -1px;
}
.tab-bar button.active {
  color: var(--c-ink);
  border-bottom-color: var(--c-accent);
}
.tab-bar button:hover:not(.active) {
  color: var(--c-ink);
}

/* —— 元信息条 —— */
.meta-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 18px;
  background: var(--c-paper-2);
  border-bottom: 1px solid var(--c-line);
  font-size: 10px;
  color: var(--c-ink-soft);
  letter-spacing: 0.1em;
  text-transform: uppercase;
  flex-shrink: 0;
}
.meta-sep {
  color: var(--c-ink-dim);
}
.meta-item.count em {
  font-family: var(--font-display);
  font-style: italic;
  font-weight: 600;
  font-size: 13px;
  color: var(--c-accent);
  font-style: normal;
  text-transform: none;
  letter-spacing: 0;
  margin-right: 2px;
}

/* —— 列表 —— */
.list {
  flex: 1;
  overflow-y: auto;
  padding: 0;
  position: relative;
}
.todo-list {
  list-style: none;
  margin: 0;
  padding: 0;
}

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

/* —— 工具条 —— */
.toolbar {
  padding: 0 18px;
  display: flex;
  justify-content: flex-end;
  flex-shrink: 0;
}
.clear-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  color: var(--c-ink-soft);
  padding: 4px 10px;
  border-radius: var(--r-pill);
  background: transparent;
  border: 1px solid var(--c-line-2);
  transition: all 0.2s var(--ease-out);
  letter-spacing: 0.04em;
}
.clear-btn .arrow {
  transition: transform 0.2s var(--ease-out);
}
.clear-btn:hover {
  color: var(--c-accent);
  border-color: var(--c-accent);
}
.clear-btn:hover .arrow {
  transform: translateX(2px);
}

/* —— 输入区 —— */
.composer {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 18px 16px;
  background: var(--c-paper);
  border-top: 1px solid var(--c-line);
  flex-shrink: 0;
}
.composer-mark {
  font-family: var(--font-display);
  font-size: 22px;
  font-weight: 400;
  color: var(--c-ink-soft);
  line-height: 1;
  width: 22px;
  text-align: center;
}
.composer-input {
  flex: 1;
  height: 32px;
  padding: 0;
  font-size: 14px;
  color: var(--c-ink);
  background: transparent;
  border: none;
  border-bottom: 1.5px solid var(--c-line-2);
  letter-spacing: 0.01em;
  transition: border-color 0.2s ease;
}
.composer-input:focus {
  border-bottom-color: var(--c-ink);
}
.composer-input::placeholder {
  color: var(--c-ink-dim);
  font-style: italic;
  font-family: var(--font-display);
}
.composer-hint {
  font-family: var(--font-display);
  font-style: italic;
  font-size: 11px;
  color: var(--c-ink-dim);
  width: 18px;
  text-align: center;
}
</style>
