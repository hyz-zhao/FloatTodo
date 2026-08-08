<script setup lang="ts">
defineProps<{
  visible: boolean;
  message: string;
}>();

const emit = defineEmits<{
  confirm: [];
  cancel: [];
}>();
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="visible" class="overlay" @click.self="emit('cancel')">
        <div class="dialog">
          <p class="msg">{{ message }}</p>
          <div class="actions">
            <button class="btn btn-cancel" @click="emit('cancel')">取消</button>
            <button class="btn btn-confirm" @click="emit('confirm')">确定</button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  z-index: 100;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.35);
  backdrop-filter: blur(4px);
}

.dialog {
  background: var(--c-paper);
  border-radius: var(--r-xl);
  padding: 28px 32px 20px;
  min-width: 240px;
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.18);
  border: 1px solid var(--c-line);
}

.msg {
  font-family: var(--font-body);
  font-size: 14px;
  color: var(--c-ink);
  letter-spacing: 0.02em;
  margin-bottom: 24px;
  text-align: center;
}

.actions {
  display: flex;
  gap: 10px;
  justify-content: center;
}

.btn {
  font-family: var(--font-body);
  font-size: 12px;
  letter-spacing: 0.06em;
  padding: 7px 24px;
  border-radius: var(--r-pill);
  transition: all 0.2s var(--ease-out);
}

.btn-cancel {
  background: transparent;
  color: var(--c-ink-soft);
  border: 1px solid var(--c-line-2);
}

.btn-cancel:hover {
  color: var(--c-ink);
  border-color: var(--c-ink-soft);
}

.btn-confirm {
  background: var(--c-ink);
  color: var(--c-paper);
}

.btn-confirm:hover {
  background: var(--c-accent);
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-active .dialog,
.modal-leave-active .dialog {
  transition: transform 0.2s var(--ease-out);
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .dialog {
  transform: scale(0.92);
}

.modal-leave-to .dialog {
  transform: scale(0.92);
}
</style>