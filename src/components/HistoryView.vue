<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import {
  apiHistoryDailySummary,
  apiHistoryWeeklySummary,
  apiListTodosByRange,
  apiListTodosForDate,
} from "../api";
import { fmtDate, humanDate, addDays, parseDate } from "../date";
import type { DateSummary, Todo, WeekSummary } from "../types";
import TodoItem from "./TodoItem.vue";
import { useToast } from "../composables/useToast";

const viewMode = ref<"daily" | "weekly">("daily");
const { show } = useToast();
const dailySummaries = ref<DateSummary[]>([]);
const weeklySummaries = ref<WeekSummary[]>([]);
const expandedDate = ref<string | null>(null);
const expandedTodos = ref<Todo[]>([]);
const expandedWeek = ref<string | null>(null);
const expandedWeekTodos = ref<Todo[]>([]);
const loading = ref(false);

const PAGE_DAYS = 30;
const today = new Date();
today.setHours(0, 0, 0, 0);
const rangeEnd = ref(fmtDate(today));
const rangeStart = ref(fmtDate(addDays(today, -(PAGE_DAYS - 1))));
const hasMore = ref(true);

function progressPercent(s: DateSummary): number {
  if (s.total === 0) return 0;
  return Math.round((s.completed / s.total) * 100);
}

async function loadSummaries() {
  try {
    dailySummaries.value = await apiHistoryDailySummary(
      rangeStart.value,
      rangeEnd.value,
    );
    weeklySummaries.value = await apiHistoryWeeklySummary(
      rangeStart.value,
      rangeEnd.value,
    );
    hasMore.value = dailySummaries.value.length >= PAGE_DAYS;
  } catch (e) {
    console.error("加载历史摘要失败", e);
    show("加载历史记录失败");
  }
}

async function toggleDate(date: string) {
  if (expandedDate.value === date) {
    expandedDate.value = null;
    expandedTodos.value = [];
    return;
  }
  expandedDate.value = date;
  expandedWeek.value = null;
  loading.value = true;
  try {
    expandedTodos.value = await apiListTodosForDate(date);
  } catch (e) {
    console.error("加载日期待办失败", e);
    show("加载待办详情失败");
  }
  loading.value = false;
}

async function toggleWeek(weekStart: string, weekEnd: string) {
  const key = weekStart;
  if (expandedWeek.value === key) {
    expandedWeek.value = null;
    expandedWeekTodos.value = [];
    return;
  }
  expandedWeek.value = key;
  expandedDate.value = null;
  loading.value = true;
  try {
    expandedWeekTodos.value = await apiListTodosByRange(weekStart, weekEnd);
  } catch (e) {
    console.error("加载周待办失败", e);
    show("加载待办详情失败");
  }
  loading.value = false;
}

async function loadMore() {
  const end = parseDate(rangeStart.value);
  const start = addDays(end, -(PAGE_DAYS - 1));
  rangeStart.value = fmtDate(start);
  await loadSummaries();
}

function describeWeekRange(ws: WeekSummary): string {
  const s = parseDate(ws.week_start);
  const e = parseDate(ws.week_end);
  const sm = s.getMonth() + 1;
  const sd = s.getDate();
  const em = e.getMonth() + 1;
  const ed = e.getDate();
  if (sm === em) return `${sm}月${sd}日 - ${ed}日`;
  return `${sm}月${sd}日 - ${em}月${ed}日`;
}

onMounted(() => {
  loadSummaries();
});
</script>

<template>
  <div class="history-view">
    <!-- 视图切换 -->
    <div class="view-toggle">
      <button
        :class="{ active: viewMode === 'daily' }"
        @click="viewMode = 'daily'"
      >
        日视图
      </button>
      <button
        :class="{ active: viewMode === 'weekly' }"
        @click="viewMode = 'weekly'"
      >
        周视图
      </button>
    </div>

    <!-- 日视图 -->
    <div v-if="viewMode === 'daily'" class="history-list">
      <div v-if="dailySummaries.length === 0" class="empty">
        <div class="empty-mark">— § —</div>
        <div class="empty-text">暂无记录</div>
        <div class="empty-hint">添加待办后这里会出现历史记录</div>
      </div>
      <div
        v-for="s in dailySummaries"
        :key="s.date"
        class="day-card"
        :class="{ expanded: expandedDate === s.date }"
      >
        <div class="day-card-header" @click="toggleDate(s.date)">
          <div class="day-info">
            <span class="day-date">{{ humanDate(s.date) }}</span>
            <span class="day-stats">
              <em>{{ s.completed }}</em> / {{ s.total }} 已完成
            </span>
          </div>
          <div class="day-progress">
            <div class="progress-bar">
              <div
                class="progress-fill"
                :class="{ full: s.completed === s.total && s.total > 0 }"
                :style="{ width: progressPercent(s) + '%' }"
              ></div>
            </div>
          </div>
          <span class="expand-icon">{{
            expandedDate === s.date ? "▾" : "▸"
          }}</span>
        </div>
        <div v-if="expandedDate === s.date" class="day-card-body">
          <div v-if="loading" class="loading">加载中...</div>
          <div v-else-if="expandedTodos.length === 0" class="no-todos">
            该日无待办事项
          </div>
          <ul v-else class="todo-list">
            <TodoItem
              v-for="t in expandedTodos"
              :key="t.id"
              :todo="t"
              :readonly="true"
            />
          </ul>
        </div>
      </div>
    </div>

    <!-- 周视图 -->
    <div v-if="viewMode === 'weekly'" class="history-list">
      <div v-if="weeklySummaries.length === 0" class="empty">
        <div class="empty-mark">— § —</div>
        <div class="empty-text">暂无记录</div>
        <div class="empty-hint">添加待办后这里会出现历史记录</div>
      </div>
      <div
        v-for="s in weeklySummaries"
        :key="s.week_start"
        class="week-card"
        :class="{ expanded: expandedWeek === s.week_start }"
      >
        <div
          class="week-card-header"
          @click="toggleWeek(s.week_start, s.week_end)"
        >
          <div class="week-info">
            <span class="week-range">{{ describeWeekRange(s) }}</span>
            <span class="week-date-sub">{{ s.week_start }} - {{ s.week_end }}</span>
          </div>
          <div class="week-stats">
            <em>{{ s.completed }}</em> / {{ s.total }} 已完成
          </div>
          <span class="expand-icon">{{
            expandedWeek === s.week_start ? "▾" : "▸"
          }}</span>
        </div>
        <div class="week-progress" @click="toggleWeek(s.week_start, s.week_end)">
          <div class="progress-bar">
            <div
              class="progress-fill"
              :class="{ full: s.completed === s.total && s.total > 0 }"
              :style="{ width: (s.total === 0 ? 0 : Math.round((s.completed / s.total) * 100)) + '%' }"
            ></div>
          </div>
        </div>
        <div v-if="expandedWeek === s.week_start" class="day-card-body">
          <div v-if="loading" class="loading">加载中...</div>
          <div v-else-if="expandedWeekTodos.length === 0" class="no-todos">
            该周无待办事项
          </div>
          <ul v-else class="todo-list">
            <TodoItem
              v-for="t in expandedWeekTodos"
              :key="t.id"
              :todo="t"
              :readonly="true"
            />
          </ul>
        </div>
      </div>
    </div>

    <!-- 加载更多 -->
    <button
      v-if="hasMore"
      class="load-more"
      @click="loadMore"
    >
      加载更多...
    </button>
  </div>
</template>

<style scoped>
.history-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* —— 视图切换 —— */
.view-toggle {
  display: flex;
  gap: 4px;
  padding: 10px 18px;
  background: var(--c-paper-2);
  border-bottom: 1px solid var(--c-line);
  flex-shrink: 0;
}
.view-toggle button {
  font-family: var(--font-body);
  font-size: 11px;
  font-weight: 500;
  letter-spacing: 0.06em;
  padding: 5px 14px;
  border-radius: var(--r-pill);
  background: transparent;
  color: var(--c-ink-soft);
  transition: all 0.2s var(--ease-out);
}
.view-toggle button.active {
  background: var(--c-ink);
  color: var(--c-paper);
}
.view-toggle button:hover:not(.active) {
  color: var(--c-ink);
}

/* —— 列表 —— */
.history-list {
  flex: 1;
  overflow-y: auto;
  padding: 0;
}

/* —— 日卡片 —— */
.day-card {
  border-bottom: 1px solid var(--c-line);
}
.day-card-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 18px;
  cursor: pointer;
  transition: background 0.15s var(--ease-out);
}
.day-card-header:hover {
  background: var(--c-paper-2);
}
.day-card.expanded .day-card-header {
  background: var(--c-paper-2);
}

.day-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.day-date {
  font-family: var(--font-display);
  font-size: 15px;
  font-weight: 500;
  color: var(--c-ink);
  letter-spacing: 0.01em;
}
.day-stats {
  font-size: 10px;
  color: var(--c-ink-soft);
  letter-spacing: 0.04em;
}
.day-stats em {
  font-family: var(--font-display);
  font-style: italic;
  font-weight: 600;
  font-size: 12px;
  color: var(--c-accent);
  margin-right: 1px;
}

.day-progress {
  width: 100px;
  display: none;
}
.day-card.expanded .day-progress,
.day-card-header:hover .day-progress {
  display: none;
}

.progress-bar {
  height: 3px;
  background: var(--c-line-2);
  border-radius: 3px;
  overflow: hidden;
}
.progress-fill {
  height: 100%;
  background: var(--c-ink-soft);
  border-radius: 3px;
  transition: width 0.5s var(--ease-out);
}
.progress-fill.full {
  background: var(--c-accent);
}

.expand-icon {
  font-size: 10px;
  color: var(--c-ink-dim);
  width: 16px;
  text-align: center;
  flex-shrink: 0;
}

/* 展开的待办列表 */
.day-card-body {
  padding: 0 0 8px;
  background: var(--c-paper-2);
}
.day-card-body .todo-list {
  list-style: none;
  margin: 0;
  padding: 0;
}
.loading,
.no-todos {
  padding: 16px 18px;
  font-size: 11px;
  color: var(--c-ink-soft);
  letter-spacing: 0.04em;
  text-align: center;
}

/* —— 周卡片 —— */
.week-card {
  border-bottom: 1px solid var(--c-line);
}
.week-card-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 18px 6px;
  cursor: pointer;
  transition: background 0.15s var(--ease-out);
}
.week-card-header:hover {
  background: var(--c-paper-2);
}
.week-card.expanded .week-card-header {
  background: var(--c-paper-2);
}
.week-progress {
  padding: 0 18px 12px;
  cursor: pointer;
}
.week-info {
  display: flex;
  flex-direction: column;
  gap: 1px;
}
.week-range {
  font-family: var(--font-display);
  font-size: 15px;
  font-weight: 500;
  color: var(--c-ink);
  letter-spacing: 0.01em;
}
.week-date-sub {
  font-size: 10px;
  color: var(--c-ink-dim);
  letter-spacing: 0.04em;
}
.week-stats {
  font-size: 10px;
  color: var(--c-ink-soft);
  letter-spacing: 0.04em;
}
.week-stats em {
  font-family: var(--font-display);
  font-style: italic;
  font-weight: 600;
  font-size: 12px;
  color: var(--c-accent);
  margin-right: 1px;
}
.week-progress .progress-bar {
  width: 100%;
}

/* —— 加载更多 —— */
.load-more {
  display: block;
  width: 100%;
  padding: 14px;
  text-align: center;
  font-family: var(--font-body);
  font-size: 11px;
  color: var(--c-ink-soft);
  letter-spacing: 0.06em;
  background: transparent;
  transition: color 0.2s;
  flex-shrink: 0;
}
.load-more:hover {
  color: var(--c-ink);
}

/* —— 空状态 —— */
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