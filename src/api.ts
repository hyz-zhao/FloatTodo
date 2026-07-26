// 封装对 Tauri 后端命令的调用，统一在此处 import，便于维护
import { invoke } from "@tauri-apps/api/core";
import type { Todo, WindowConfig } from "./types";

/** 查询某日期范围内的事项列表 */
export function apiListTodos(rangeStart: string, rangeEnd: string) {
  return invoke<Todo[]>("list_todos", {
    rangeStart,
    rangeEnd,
  });
}

/** 新增一条事项 */
export function apiAddTodo(text: string, rangeStart: string, rangeEnd: string) {
  return invoke<Todo>("add_todo", {
    payload: {
      text,
      range_start: rangeStart,
      range_end: rangeEnd,
    },
  });
}

/** 更新事项（文本或完成状态） */
export function apiUpdateTodo(
  id: number,
  patch: { text?: string; completed?: boolean }
) {
  return invoke<void>("update_todo", {
    payload: {
      id,
      text: patch.text,
      completed: patch.completed,
    },
  });
}

/** 删除单条 */
export function apiDeleteTodo(id: number) {
  return invoke<void>("delete_todo", { id });
}

/** 清除范围内已完成事项 */
export function apiClearCompleted(rangeStart: string, rangeEnd: string) {
  return invoke<void>("clear_completed", {
    rangeStart,
    rangeEnd,
  });
}

/** 读取浮窗配置 */
export function apiGetConfig() {
  return invoke<WindowConfig>("get_config");
}

/** 保存浮窗配置 */
export function apiSaveWindowConfig(config: WindowConfig) {
  return invoke<void>("save_window_config", { config });
}

/** 切换为悬浮球模式 */
export function apiCollapseToBall() {
  return invoke<void>("collapse_to_ball");
}

/** 切换为主面板模式 */
export function apiExpandToPanel() {
  return invoke<void>("expand_to_panel");
}

/** 退出应用 */
export function apiQuit() {
  return invoke<void>("quit_app");
}
