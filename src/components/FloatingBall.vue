<script setup lang="ts">
// 悬浮球：单击展开主面板；长按进入拖动模式（拖动结束后自动保存位置）
import { onBeforeUnmount, onMounted, ref } from "vue";
import { apiCollapseToBall, apiExpandToPanel, apiSaveWindowConfig, apiGetConfig } from "../api";

const longPressTimer = ref<number | null>(null);
const isDragging = ref(false);
const dragMoved = ref(false);
const dragOffsetX = ref(0);
const dragOffsetY = ref(0);
let startX = 0;
let startY = 0;

function onMouseDown(e: MouseEvent) {
  // 仅响应左键
  if (e.button !== 0) return;
  startX = e.clientX;
  startY = e.clientY;
  dragMoved.value = false;
  // 长按 350ms 进入拖动模式
  longPressTimer.value = window.setTimeout(() => {
    isDragging.value = true;
    // 通过 Tauri 直接拖动窗口
    import("@tauri-apps/api/window").then(({ getCurrentWindow }) => {
      getCurrentWindow().startDragging().catch(() => {
        /* ignore */
      });
    });
  }, 350);

  window.addEventListener("mousemove", onMouseMove);
  window.addEventListener("mouseup", onMouseUp, { once: true });
}

function onMouseMove(e: MouseEvent) {
  const dx = Math.abs(e.clientX - startX);
  const dy = Math.abs(e.clientY - startY);
  // 移动超过 5 像素则取消长按（避免误触）
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
  // 没有拖动 = 单击，展开主面板
  if (!dragMoved.value && !isDragging.value) {
    await apiExpandToPanel();
  }
  // 拖动结束后保存新位置
  if (isDragging.value) {
    isDragging.value = false;
    try {
      const { getCurrentWindow, LogicalPosition } = await import("@tauri-apps/api/window");
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
  // 阻止悬浮球上的右键菜单
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
    <div class="ball-inner">F</div>
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
}

.ball-inner {
  width: 52px;
  height: 52px;
  border-radius: 50%;
  background: var(--c-ball);
  color: #fff;
  font-weight: 700;
  font-size: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 6px 18px var(--c-ball-shadow);
  transition: transform 0.15s ease, box-shadow 0.15s ease;
  user-select: none;
}

.ball:hover .ball-inner {
  transform: scale(1.05);
  box-shadow: 0 8px 22px var(--c-ball-shadow);
}

.ball:active .ball-inner {
  transform: scale(0.96);
}

.ball.dragging {
  cursor: grabbing;
}
.ball.dragging .ball-inner {
  transform: scale(1.1);
  box-shadow: 0 10px 26px var(--c-ball-shadow);
}
</style>
