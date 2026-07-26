use crate::models::NewTodo;
use crate::models::Todo;
use crate::models::UpdateTodo;
use chrono::Local;
use rusqlite::{params, Connection};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

/// 全局数据库状态（单连接 + Mutex 互斥）
pub struct DbState(pub Mutex<Connection>);

/// 获取应用数据目录下的 float-todo.db 路径
fn db_path(app: &AppHandle) -> PathBuf {
    let dir = app
        .path()
        .app_data_dir()
        .expect("无法获取 app_data_dir，请检查 Tauri 配置");
    if !dir.exists() {
        std::fs::create_dir_all(&dir).expect("创建应用数据目录失败");
    }
    dir.join("float-todo.db")
}

/// 初始化数据库（建表）
pub fn init_db(app: &AppHandle) -> Connection {
    let path = db_path(app);
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
