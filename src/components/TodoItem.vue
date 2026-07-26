<script setup lang="ts">
// 单条待办：圆形复选框 + SVG 笔触勾选动画，编辑感删除线
import { ref } from "vue";
import type { Todo } from "../types";

const props = defineProps<{ todo: Todo }>();
const emit = defineEmits<{
  (e: "toggle", todo: Todo, completed: boolean): void;
  (e: "remove", todo: Todo): void;
  (e: "edit", todo: Todo, text: string): void;
}>();

const editing = ref(false);
const draft = ref(props.todo.text);
const inputRef = ref<HTMLInputElement | null>(null);

function startEdit() {
  if (props.todo.completed) return;
  draft.value = props.todo.text;
  editing.value = true;
  setTimeout(() => inputRef.value?.focus(), 0);
}

function commit() {
  const v = draft.value.trim();
  if (!v) {
    emit("remove", props.todo);
    return;
  }
  if (v !== props.todo.text) {
    emit("edit", props.todo, v);
  }
  editing.value = false;
}

function cancel() {
  editing.value = false;
  draft.value = props.todo.text;
}

function onToggle(e: Event) {
  const checked = (e.target as HTMLInputElement).checked;
  emit("toggle", props.todo, checked);
}
</script>

<template>
  <li class="todo-item" :class="{ done: todo.completed }">
    <!-- 复选框（带 SVG 笔触动画） -->
    <label class="check">
      <input
        type="checkbox"
        :checked="todo.completed"
        @change="onToggle"
      />
      <span class="box" aria-hidden="true">
        <svg viewBox="0 0 16 16" width="11" height="11" class="tick">
          <path
            class="tick-path"
            d="M3 8.5l3 3 7-7"
            fill="none"
            stroke="currentColor"
            stroke-width="1.8"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </span>
    </label>

    <div class="text-wrap" @dblclick="startEdit">
      <input
        v-if="editing"
        ref="inputRef"
        v-model="draft"
        class="edit-input"
        @blur="commit"
        @keydown.enter="commit"
        @keydown.esc="cancel"
      />
      <span v-else class="text">{{ todo.text }}</span>
    </div>

    <button
      v-if="!editing"
      class="remove"
      title="删除"
      @click="emit('remove', todo)"
    >
      <svg viewBox="0 0 16 16" width="10" height="10" aria-hidden="true">
        <path
          d="M3 3l10 10M13 3L3 13"
          stroke="currentColor"
          stroke-width="1.5"
          stroke-linecap="round"
        />
      </svg>
    </button>
  </li>
</template>

<style scoped>
.todo-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 14px;
  position: relative;
  transition: background 0.2s var(--ease-out);
  animation: itemIn 0.4s var(--ease-out) backwards;
}

/* 列表项错落淡入 */
@keyframes itemIn {
  from {
    opacity: 0;
    transform: translateX(-4px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

/* 行间分隔：上下细线 */
.todo-item::before,
.todo-item::after {
  content: "";
  position: absolute;
  left: 14px;
  right: 14px;
  height: 1px;
  background: var(--c-line);
}
.todo-item::before {
  top: 0;
}
.todo-item::after {
  bottom: 0;
}
.todo-item:last-child::after {
  display: none;
}

/* —— 复选框 —— */
.check {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  flex-shrink: 0;
  cursor: pointer;
}
.check input {
  position: absolute;
  opacity: 0;
  width: 100%;
  height: 100%;
  margin: 0;
  cursor: pointer;
}
.box {
  width: 20px;
  height: 20px;
  border: 1.25px solid var(--c-ink-soft);
  border-radius: 50%;
  background: transparent;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: var(--c-paper);
  transition: all 0.25s var(--ease-out);
}
.check:hover .box {
  border-color: var(--c-ink);
  transform: scale(1.06);
}
.check input:checked + .box {
  background: var(--c-accent);
  border-color: var(--c-accent);
}

/* 勾选时笔触动画 */
.tick {
  overflow: visible;
}
.tick-path {
  stroke-dasharray: 18;
  stroke-dashoffset: 18;
  transition: stroke-dashoffset 0.32s var(--ease-out) 0.05s;
}
.check input:checked + .box .tick-path {
  stroke-dashoffset: 0;
}

/* —— 文字 —— */
.text-wrap {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
}
.text {
  font-size: 14px;
  color: var(--c-ink);
  line-height: 1.45;
  word-break: break-word;
  transition: color 0.3s ease;
  letter-spacing: 0.01em;
}

.todo-item.done .text {
  color: var(--c-ink-dim);
  /* 双重删除线 + 自定义偏移，模拟手绘感 */
  text-decoration: line-through;
  text-decoration-color: var(--c-accent);
  text-decoration-thickness: 1.2px;
  text-underline-offset: 2px;
}

.edit-input {
  flex: 1;
  width: 100%;
  font-size: 14px;
  padding: 2px 4px;
  border: none;
  border-bottom: 1.5px solid var(--c-accent);
  color: var(--c-ink);
  background: transparent;
  outline: none;
}

/* —— 删除按钮 —— */
.remove {
  width: 22px;
  height: 22px;
  border-radius: 50%;
  color: var(--c-ink-dim);
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: all 0.2s var(--ease-out);
  flex-shrink: 0;
  transform: scale(0.9);
}
.todo-item:hover .remove {
  opacity: 1;
  transform: scale(1);
}
.remove:hover {
  background: var(--c-ink);
  color: var(--c-paper);
}
</style>
