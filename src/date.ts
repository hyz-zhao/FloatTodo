// 日期工具：格式化、今天/本周/下周快捷、范围判断
import type { RangePreset } from "./types";

/** yyyy-MM-dd 格式 */
export function fmtDate(d: Date): string {
  const y = d.getFullYear();
  const m = String(d.getMonth() + 1).padStart(2, "0");
  const day = String(d.getDate()).padStart(2, "0");
  return `${y}-${m}-${day}`;
}

/** 解析 yyyy-MM-dd 字符串为本地 Date（00:00:00） */
export function parseDate(s: string): Date {
  const [y, m, d] = s.split("-").map(Number);
  return new Date(y, (m || 1) - 1, d || 1);
}

/** 一周从周一开始；返回给定日期所在周的周一 */
export function startOfWeek(d: Date): Date {
  const day = d.getDay(); // 0=周日, 1=周一
  const diff = day === 0 ? -6 : 1 - day;
  const out = new Date(d);
  out.setDate(d.getDate() + diff);
  out.setHours(0, 0, 0, 0);
  return out;
}

/** 加 N 天，返回新对象 */
export function addDays(d: Date, n: number): Date {
  const out = new Date(d);
  out.setDate(d.getDate() + n);
  return out;
}

/** 人类可读日期（中文）：今天/昨天/M月D日/yyyy年M月D日 */
export function humanDate(s: string): string {
  const d = parseDate(s);
  const today = new Date();
  today.setHours(0, 0, 0, 0);
  const diffDays = Math.round((d.getTime() - today.getTime()) / 86400000);
  if (diffDays === 0) return "今天";
  if (diffDays === 1) return "明天";
  if (diffDays === -1) return "昨天";
  if (d.getFullYear() === today.getFullYear()) {
    return `${d.getMonth() + 1}月${d.getDate()}日`;
  }
  return `${d.getFullYear()}年${d.getMonth() + 1}月${d.getDate()}日`;
}

/** 范围描述：今天 / 明天 / M月D日 - M月D日 / yyyy-MM-dd - yyyy-MM-dd */
export function describeRange(start: string, end: string): string {
  if (start === end) {
    return humanDate(start);
  }
  return `${humanDate(start)} - ${humanDate(end)}`;
}

/** 根据预设获取起止日期（包含两端） */
export function rangeForPreset(preset: RangePreset, customStart?: string, customEnd?: string): { start: string; end: string } {
  const today = new Date();
  today.setHours(0, 0, 0, 0);
  switch (preset) {
    case "today":
      return { start: fmtDate(today), end: fmtDate(today) };
    case "tomorrow": {
      const t = addDays(today, 1);
      return { start: fmtDate(t), end: fmtDate(t) };
    }
    case "this-week": {
      const s = startOfWeek(today);
      const e = addDays(s, 6);
      return { start: fmtDate(s), end: fmtDate(e) };
    }
    case "next-week": {
      const s = addDays(startOfWeek(today), 7);
      const e = addDays(s, 6);
      return { start: fmtDate(s), end: fmtDate(e) };
    }
    case "custom":
      return { start: customStart || fmtDate(today), end: customEnd || fmtDate(today) };
  }
}
