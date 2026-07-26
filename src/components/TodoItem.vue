<script setup lang="ts">
// 单条待办：复选框 + 文本（勾选后画删除线）+ 悬停出现删除按钮
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
  // 已完成的不允许再次编辑（保持简洁）
  if (props.todo.completed) return;
  draft.value = props.todo.text;
  editing.value = true;
  setTimeout(() => inputRef.value?.focus(), 0);
}

function commit() {
  const v = draft.value.trim();
  if (!v) {
    // 空内容视为删除
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
    <label class="check">
      <input
        type="checkbox"
        :checked="todo.completed"
        @change="onToggle"
      />
      <span class="box" aria-hidden="true">
        <svg v-if="todo.completed" viewBox="0 0 16 16" width="10" height="10">
          <path
            d="M3 8.5l3 3 7-7"
            fill="none"
            stroke="white"
            stroke-width="2"
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
      ×
    </button>
  </li>
</template>

<style scoped>
.todo-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  border-radius: var(--r-sm);
  transition: background 0.15s ease;
  position: relative;
}
.todo-item:hover {
  background: var(--c-bg-soft);
}

.check {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
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
  width: 18px;
  height: 18px;
  border: 1.5px solid #c8d0d8;
  border-radius: 4px;
  background: #fff;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}
.check:hover .box {
  border-color: var(--c-primary);
}
.check input:checked + .box {
  background: var(--c-primary);
  border-color: var(--c-primary);
}

.text-wrap {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
}

.text {
  font-size: 14px;
  color: var(--c-text);
  line-height: 1.4;
  word-break: break-word;
  transition: color 0.15s ease, text-decoration-color 0.2s ease;
}

.todo-item.done .text {
  color: var(--c-text-dim);
  text-decoration: line-through;
  text-decoration-color: var(--c-text-dim);
}

.edit-input {
  flex: 1;
  width: 100%;
  font-size: 14px;
  padding: 2px 4px;
  border: 1px solid var(--c-primary);
  border-radius: 4px;
  background: #fff;
}

.remove {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  color: var(--c-text-dim);
  font-size: 18px;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: all 0.15s ease;
  flex-shrink: 0;
}
.todo-item:hover .remove {
  opacity: 1;
}
.remove:hover {
  background: var(--c-danger);
  color: #fff;
}
</style>
