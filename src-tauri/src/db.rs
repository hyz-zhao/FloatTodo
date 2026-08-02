use crate::models::DateSummary;
use crate::models::NewTodo;
use crate::models::Todo;
use crate::models::UpdateTodo;
use crate::models::WeekSummary;
use chrono::{Datelike, Local, NaiveDate};
use rusqlite::{params, Connection};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

/// 全局数据库状态（单连接 + Mutex 互斥）
pub struct DbState(pub Mutex<Connection>);

/// 获取项目目录下的 data/float-todo.db 路径
fn db_path() -> PathBuf {
    let dir = std::env::current_dir()
        .expect("无法获取当前工作目录")
        .join("data");
    if !dir.exists() {
        std::fs::create_dir_all(&dir).expect("创建数据目录失败");
    }
    dir.join("float-todo.db")
}

/// 初始化数据库（建表）
pub fn init_db() -> Connection {
    let path = db_path();
    let conn = Connection::open(&path).expect("打开数据库失败");
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            text        TEXT    NOT NULL,
            completed   INTEGER NOT NULL DEFAULT 0,
            range_start TEXT    NOT NULL,
            range_end   TEXT    NOT NULL,
            created_at  TEXT    NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_todos_range
            ON todos(range_start, range_end);
        "#,
    )
    .expect("初始化数据库表失败");
    conn
}

/// 查询某个日期范围内的事项，按未完成优先、再按创建时间倒序
#[tauri::command]
pub fn list_todos(
    state: State<'_, DbState>,
    range_start: String,
    range_end: String,
) -> Result<Vec<Todo>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, text, completed, range_start, range_end, created_at
             FROM todos
             WHERE range_start = ?1 AND range_end = ?2
             ORDER BY completed ASC, id DESC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(params![range_start, range_end], |row| {
            Ok(Todo {
                id: row.get(0)?,
                text: row.get(1)?,
                completed: row.get::<_, i64>(2)? != 0,
                range_start: row.get(3)?,
                range_end: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

/// 新增一条事项
#[tauri::command]
pub fn add_todo(state: State<'_, DbState>, payload: NewTodo) -> Result<Todo, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let now = Local::now().to_rfc3339();
    conn.execute(
        "INSERT INTO todos (text, completed, range_start, range_end, created_at)
         VALUES (?1, 0, ?2, ?3, ?4)",
        params![payload.text, payload.range_start, payload.range_end, now],
    )
    .map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(Todo {
        id,
        text: payload.text,
        completed: false,
        range_start: payload.range_start,
        range_end: payload.range_end,
        created_at: now,
    })
}

/// 更新事项（可更新文本或完成状态）
#[tauri::command]
pub fn update_todo(state: State<'_, DbState>, payload: UpdateTodo) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    match (&payload.text, &payload.completed) {
        (Some(t), Some(c)) => {
            conn.execute(
                "UPDATE todos SET text = ?1, completed = ?2 WHERE id = ?3",
                params![t, *c as i64, payload.id],
            )
            .map_err(|e| e.to_string())?;
        }
        (Some(t), None) => {
            conn.execute(
                "UPDATE todos SET text = ?1 WHERE id = ?2",
                params![t, payload.id],
            )
            .map_err(|e| e.to_string())?;
        }
        (None, Some(c)) => {
            conn.execute(
                "UPDATE todos SET completed = ?1 WHERE id = ?2",
                params![*c as i64, payload.id],
            )
            .map_err(|e| e.to_string())?;
        }
        (None, None) => return Ok(()),
    }
    Ok(())
}

/// 删除单条事项
#[tauri::command]
pub fn delete_todo(state: State<'_, DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM todos WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 清除某个范围内的全部已完成事项
#[tauri::command]
pub fn clear_completed(
    state: State<'_, DbState>,
    range_start: String,
    range_end: String,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "DELETE FROM todos WHERE completed = 1 AND range_start = ?1 AND range_end = ?2",
        params![range_start, range_end],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// 日统计摘要：按 range_start 分组汇总
#[tauri::command]
pub fn history_daily_summary(
    state: State<'_, DbState>,
    date_from: String,
    date_to: String,
) -> Result<Vec<DateSummary>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT range_start, COUNT(*) as total,
                    SUM(CASE WHEN completed = 1 THEN 1 ELSE 0 END) as completed
             FROM todos
             WHERE range_start >= ?1 AND range_start <= ?2
               AND range_start = range_end
             GROUP BY range_start
             ORDER BY range_start DESC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(params![date_from, date_to], |row| {
            Ok(DateSummary {
                date: row.get(0)?,
                total: row.get(1)?,
                completed: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

/// 周统计摘要：按 ISO 周分组汇总
#[tauri::command]
pub fn history_weekly_summary(
    state: State<'_, DbState>,
    date_from: String,
    date_to: String,
) -> Result<Vec<WeekSummary>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT range_start, completed FROM todos
             WHERE range_start >= ?1 AND range_start <= ?2
             ORDER BY range_start DESC",
        )
        .map_err(|e| e.to_string())?;
    let rows: Vec<(String, bool)> = stmt
        .query_map(params![date_from, date_to], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)? != 0))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    // 按 ISO 周分组
    let mut weeks: BTreeMap<(i32, u32), (i64, i64)> = BTreeMap::new();
    for (date_str, completed) in &rows {
        if let Ok(d) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            let iso = d.iso_week();
            let key = (iso.year(), iso.week());
            let entry = weeks.entry(key).or_insert((0, 0));
            entry.0 += 1;
            if *completed {
                entry.1 += 1;
            }
        }
    }

    let mut out = Vec::new();
    for ((year, week_num), (total, completed)) in weeks.iter().rev() {
        // 计算该 ISO 周的周一和周日
        // ISO 周定义：第 1 周 = 包含该年第一个周四的周
        let jan4 = NaiveDate::from_ymd_opt(*year, 1, 4).unwrap();
        let weekday = jan4.weekday().num_days_from_monday(); // 0=Mon
        let week1_monday = jan4 - chrono::Duration::days(weekday as i64);
        let monday = week1_monday + chrono::Duration::days(((*week_num as i64) - 1) * 7);
        let sunday = monday + chrono::Duration::days(6);

        out.push(WeekSummary {
            week_start: monday.format("%Y-%m-%d").to_string(),
            week_end: sunday.format("%Y-%m-%d").to_string(),
            total: *total,
            completed: *completed,
        });
    }

    Ok(out)
}

/// 查询某一天包含的所有待办（仅单日待办，用于日视图展开）
#[tauri::command]
pub fn list_todos_for_date(
    state: State<'_, DbState>,
    date: String,
) -> Result<Vec<Todo>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, text, completed, range_start, range_end, created_at
             FROM todos
             WHERE range_start = ?1 AND range_end = ?1
             ORDER BY completed ASC, id DESC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(params![date], |row| {
            Ok(Todo {
                id: row.get(0)?,
                text: row.get(1)?,
                completed: row.get::<_, i64>(2)? != 0,
                range_start: row.get(3)?,
                range_end: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

/// 按 range_start 范围查询待办（用于周视图展开）
#[tauri::command]
pub fn list_todos_by_range(
    state: State<'_, DbState>,
    range_start: String,
    range_end: String,
) -> Result<Vec<Todo>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, text, completed, range_start, range_end, created_at
             FROM todos
             WHERE range_start >= ?1 AND range_start <= ?2
             ORDER BY completed ASC, id DESC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(params![range_start, range_end], |row| {
            Ok(Todo {
                id: row.get(0)?,
                text: row.get(1)?,
                completed: row.get::<_, i64>(2)? != 0,
                range_start: row.get(3)?,
                range_end: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}
