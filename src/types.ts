// 全局共享类型定义

/** 单条待办 */
export interface Todo {
  id: number;
  text: string;
  completed: boolean;
  range_start: string;
  range_end: string;
  created_at: string;
}

/** 浮窗位置 / 尺寸 / 上次选中日期范围 配置 */
export interface WindowConfig {
  ball_x: number;
  ball_y: number;
  panel_x: number;
  panel_y: number;
  panel_width: number;
  panel_height: number;
  last_range_start: string;
  last_range_end: string;
}

/** 快捷日期范围选项 */
export type RangePreset = "today" | "tomorrow" | "this-week" | "next-week" | "custom";

/** 前端 UI 当前形态 */
export type UiMode = "ball" | "panel";

/** 单日统计摘要 */
export interface DateSummary {
  date: string;
  total: number;
  completed: number;
}

/** 单周统计摘要 */
export interface WeekSummary {
  week_start: string;
  week_end: string;
  total: number;
  completed: number;
}
