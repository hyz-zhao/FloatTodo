<script setup lang="ts">
// 根组件：根据后端推送的 mode-change 事件，在「悬浮球」和「主面板」之间切换
import { onBeforeUnmount, onMounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import FloatingBall from "./components/FloatingBall.vue";
import MainPanel from "./components/MainPanel.vue";
import type { UiMode } from "./types";

const mode = ref<UiMode>("ball");
let unlisten: (() => void) | null = null;

onMounted(async () => {
  unlisten = await listen<UiMode>("mode-change", (e) => {
    mode.value = e.payload;
  });
});
onBeforeUnmount(() => {
  if (unlisten) unlisten();
});
</script>

<template>
  <main class="root">
    <Transition name="fade" mode="out-in">
      <MainPanel v-if="mode === 'panel'" key="panel" />
      <FloatingBall v-else key="ball" />
    </Transition>
  </main>
</template>

<style scoped>
.root {
  position: fixed;
  inset: 0;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.18s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
