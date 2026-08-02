use serde::{Deserialize, Serialize};

/// 待办事项数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    /// 自增主键
    pub id: i64,
    /// 事项文本
    pub text: String,
    /// 是否完成
    pub completed: bool,
    /// 所属日期范围起始日（包含），格式 yyyy-MM-dd
    pub range_start: String,
    /// 所属日期范围结束日（包含），格式 yyyy-MM-dd
    pub range_end: String,
    /// 创建时间（ISO 8601 字符串）
    pub created_at: String,
}

/// 新增待办时的入参
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTodo {
    pub text: String,
    pub range_start: String,
    pub range_end: String,
}

/// 更新待办时的入参
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTodo {
    pub id: i64,
    pub text: Option<String>,
    pub completed: Option<bool>,
}

/// 浮窗位置配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// 悬浮球 X 坐标
    pub ball_x: i32,
    /// 悬浮球 Y 坐标
    pub ball_y: i32,
    /// 主面板 X 坐标（-1 表示未记忆）
    pub panel_x: i32,
    /// 主面板 Y 坐标
    pub panel_y: i32,
    /// 主面板宽度
    pub panel_width: u32,
    /// 主面板高度
    pub panel_height: u32,
    /// 上次选中的日期范围起始
    pub last_range_start: String,
    /// 上次选中的日期范围结束
    pub last_range_end: String,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            ball_x: -1,
            ball_y: -1,
            panel_x: -1,
            panel_y: -1,
            panel_width: 360,
            panel_height: 520,
            last_range_start: String::new(),
            last_range_end: String::new(),
        }
    }
}

/// 单日统计摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateSummary {
    pub date: String,
    pub total: i64,
    pub completed: i64,
}

/// 单周统计摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekSummary {
    pub week_start: String,
    pub week_end: String,
    pub total: i64,
    pub completed: i64,
}
