<script setup lang="ts">
// 悬浮球：墨色底 + 朱砂光晕，长按拖动，单击展开
import { onBeforeUnmount, onMounted, ref } from "vue";
import { apiCollapseToBall, apiExpandToPanel, apiSaveWindowConfig, apiGetConfig } from "../api";

const longPressTimer = ref<number | null>(null);
const isDragging = ref(false);
const dragMoved = ref(false);
let startX = 0;
let startY = 0;
const ballRef = ref<HTMLDivElement | null>(null);

function onMouseDown(e: MouseEvent) {
  if (e.button !== 0) return;
  startX = e.clientX;
  startY = e.clientY;
  dragMoved.value = false;
  // 长按 350ms 进入拖动
  longPressTimer.value = window.setTimeout(() => {
    isDragging.value = true;
    import("@tauri-apps/api/window").then(({ getCurrentWindow }) => {
      getCurrentWindow().startDragging().catch(() => {});
    });
  }, 350);
  window.addEventListener("mousemove", onMouseMove);
  window.addEventListener("mouseup", onMouseUp, { once: true });
}

function onMouseMove(e: MouseEvent) {
  const dx = Math.abs(e.clientX - startX);
  const dy = Math.abs(e.clientY - startY);
  if (dx > 5 || dy > 5) {
    if (longPressTimer.value !== null) {
      clearTimeout(longPressTimer.value);
      longPressTimer.value = null;
    }
    dragMoved.value = true;
  }
}

async function onMouseUp(_e: MouseEvent) {
  window.removeEventListener("mousemove", onMouseMove);
  if (longPressTimer.value !== null) {
    clearTimeout(longPressTimer.value);
    longPressTimer.value = null;
  }
  // 没有拖动 = 单击
  if (!dragMoved.value && !isDragging.value) {
    await apiExpandToPanel();
  }
  // 拖动结束后保存新位置
  if (isDragging.value) {
    isDragging.value = false;
    try {
      const { getCurrentWindow } = await import("@tauri-apps/api/window");
      const pos = await getCurrentWindow().outerPosition();
      const scale = await getCurrentWindow().scaleFactor();
      const cfg = await apiGetConfig();
      await apiSaveWindowConfig({
        ...cfg,
        ball_x: Math.round(pos.x / scale),
        ball_y: Math.round(pos.y / scale),
      });
    } catch (err) {
      console.error("保存悬浮球位置失败", err);
    }
  }
}

onMounted(() => {
  window.addEventListener("contextmenu", preventContext);
});
onBeforeUnmount(() => {
  window.removeEventListener("contextmenu", preventContext);
  if (longPressTimer.value !== null) clearTimeout(longPressTimer.value);
});

function preventContext(e: Event) {
  e.preventDefault();
}
</script>

<template>
  <div
    class="ball"
    :class="{ dragging: isDragging }"
    @mousedown="onMouseDown"
    @contextmenu.prevent
    title="点击展开 · 长按拖动"
  >
    <div class="ball-glow" aria-hidden="true"></div>
    <div class="ball-ring" aria-hidden="true"></div>
    <div class="ball-inner" ref="ballRef">
      <span class="glyph">F</span>
      <span class="dot" aria-hidden="true"></span>
    </div>
  </div>
</template>

<style scoped>
.ball {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  isolation: isolate;
}

/* 朱砂色径向光晕 */
.ball-glow {
  position: absolute;
  width: 88px;
  height: 88px;
  border-radius: 50%;
  background: radial-gradient(
    circle at center,
    rgba(183, 71, 42, 0.45) 0%,
    rgba(183, 71, 42, 0.18) 38%,
    transparent 70%
  );
  z-index: -1;
  animation: breathe 3.2s var(--ease-in-out) infinite;
  filter: blur(2px);
}

/* 墨色圆环 */
.ball-ring {
  position: absolute;
  width: 50px;
  height: 50px;
  border-radius: 50%;
  border: 1.25px solid var(--c-ink);
  z-index: 0;
  transition: transform 0.3s var(--ease-out), border-color 0.3s ease;
}

/* 墨色实心圆 + 衬线 F */
.ball-inner {
  position: relative;
  z-index: 1;
  width: 44px;
  height: 44px;
  border-radius: 50%;
  background: var(--c-ink);
  color: var(--c-paper);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 6px 20px rgba(26, 24, 20, 0.32);
  transition: transform 0.25s var(--ease-out), box-shadow 0.25s ease;
}

.glyph {
  font-family: var(--font-display);
  font-style: italic;
  font-weight: 500;
  font-size: 24px;
  letter-spacing: -0.02em;
  line-height: 1;
  color: var(--c-paper);
  transform: translateY(-1px);
}

/* 右下角朱砂小点 */
.dot {
  position: absolute;
  right: -1px;
  bottom: -1px;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--c-accent);
  border: 1.5px solid var(--c-paper);
}

.ball:hover .ball-ring {
  transform: scale(1.08);
  border-color: var(--c-accent);
}
.ball:hover .ball-inner {
  transform: scale(1.04);
  box-shadow: 0 8px 24px rgba(26, 24, 20, 0.38);
}
.ball:active .ball-inner {
  transform: scale(0.94);
}
.ball.dragging {
  cursor: grabbing;
}
.ball.dragging .ball-inner {
  transform: scale(1.12) rotate(-3deg);
}
.ball.dragging .ball-glow {
  animation: none;
  opacity: 0.6;
}

@keyframes breathe {
  0%, 100% {
    transform: scale(1);
    opacity: 0.85;
  }
  50% {
    transform: scale(1.18);
    opacity: 1;
  }
}
</style>
