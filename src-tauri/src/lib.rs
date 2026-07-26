mod db;
mod models;

use db::{add_todo, clear_completed, delete_todo, init_db, list_todos, update_todo, DbState};
use models::WindowConfig;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewWindow};

/// 配置文件状态
pub struct ConfigState(pub Mutex<WindowConfig>);

/// 获取配置 json 的绝对路径
fn config_path(app: &AppHandle) -> PathBuf {
    let dir = app.path().app_data_dir().expect("无法获取 app_data_dir");
    if !dir.exists() {
        let _ = fs::create_dir_all(&dir);
    }
    dir.join("config.json")
}

/// 从磁盘加载配置（不存在则返回默认值）
fn load_config(app: &AppHandle) -> WindowConfig {
    let path = config_path(app);
    if !path.exists() {
        return WindowConfig::default();
    }
    let content = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

/// 把配置写回磁盘
fn save_config(app: &AppHandle, cfg: &WindowConfig) -> Result<(), String> {
    let path = config_path(app);
    let content = serde_json::to_string_pretty(cfg).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(())
}

/// 悬浮球的默认尺寸（逻辑像素，CSS 像素）
const BALL_SIZE: f64 = 60.0;
/// 主面板的默认尺寸（逻辑像素）
const PANEL_DEFAULT_W: f64 = 360.0;
const PANEL_DEFAULT_H: f64 = 520.0;

/// 读取前端需要的浮窗配置
#[tauri::command]
fn get_config(state: tauri::State<'_, ConfigState>) -> Result<WindowConfig, String> {
    let cfg = state.0.lock().map_err(|e| e.to_string())?;
    Ok(cfg.clone())
}

/// 保存浮窗配置（前端在拖动 / 尺寸变化后调用）
#[tauri::command]
fn save_window_config(
    app: AppHandle,
    state: tauri::State<'_, ConfigState>,
    config: WindowConfig,
) -> Result<(), String> {
    {
        let mut cfg = state.0.lock().map_err(|e| e.to_string())?;
        *cfg = config.clone();
    }
    save_config(&app, &config)
}

/// 切换为悬浮球模式：调整窗口尺寸到 BALL_SIZE
#[tauri::command]
fn collapse_to_ball(app: AppHandle) -> Result<(), String> {
    let win: WebviewWindow = app.get_webview_window("main").ok_or("找不到主窗口")?;

    // 取当前窗口中心点
    let scale = win.scale_factor().unwrap_or(1.0);
    let pos = win.outer_position().map_err(|e| e.to_string())?;
    let size = win.outer_size().map_err(|e| e.to_string())?;
    let cx = pos.x as f64 + (size.width as f64) / 2.0 / scale;
    let cy = pos.y as f64 + (size.height as f64) / 2.0 / scale;

    let new_physical = PhysicalSize::new(
        (BALL_SIZE * scale).round() as u32,
        (BALL_SIZE * scale).round() as u32,
    );
    win.set_size(new_physical).map_err(|e| e.to_string())?;

    let new_x = (cx * scale - (BALL_SIZE * scale) / 2.0).round() as i32;
    let new_y = (cy * scale - (BALL_SIZE * scale) / 2.0).round() as i32;
    win.set_position(PhysicalPosition::new(new_x, new_y))
        .map_err(|e| e.to_string())?;
    win.set_resizable(false).map_err(|e| e.to_string())?;

    // 通知前端切换 UI 形态
    win.emit("mode-change", "ball").map_err(|e| e.to_string())?;
    Ok(())
}

/// 切换为主面板模式：恢复默认或记忆的尺寸
#[tauri::command]
fn expand_to_panel(app: AppHandle) -> Result<(), String> {
    let win: WebviewWindow = app.get_webview_window("main").ok_or("找不到主窗口")?;
    let state: tauri::State<'_, ConfigState> = app.state();
    let cfg = state.0.lock().map_err(|e| e.to_string())?.clone();

    let scale = win.scale_factor().unwrap_or(1.0);
    // 计算展开位置：球中心 -> 面板左上角
    let pos = win.outer_position().map_err(|e| e.to_string())?;
    let size = win.outer_size().map_err(|e| e.to_string())?;
    let ball_cx = pos.x as f64 + (size.width as f64) / 2.0 / scale;
    let ball_cy = pos.y as f64 + (size.height as f64) / 2.0 / scale;

    let w = if cfg.panel_width == 0 {
        PANEL_DEFAULT_W as u32
    } else {
        cfg.panel_width
    };
    let h = if cfg.panel_height == 0 {
        PANEL_DEFAULT_H as u32
    } else {
        cfg.panel_height
    };

    let new_physical = PhysicalSize::new(
        (w as f64 * scale).round() as u32,
        (h as f64 * scale).round() as u32,
    );
    win.set_size(new_physical).map_err(|e| e.to_string())?;
    win.set_resizable(true).map_err(|e| e.to_string())?;

    // 优先用记忆位置，否则以悬浮球中心作为参考，居中展开
    if cfg.panel_x >= 0 && cfg.panel_y >= 0 {
        win.set_position(PhysicalPosition::new(cfg.panel_x, cfg.panel_y))
            .map_err(|e| e.to_string())?;
    } else {
        let new_x = (ball_cx * scale - (w as f64 * scale) / 2.0).round() as i32;
        let new_y = (ball_cy * scale - (h as f64 * scale) / 2.0).round() as i32;
        win.set_position(PhysicalPosition::new(new_x, new_y))
            .map_err(|e| e.to_string())?;
    }

    win.emit("mode-change", "panel")
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 完全退出应用
#[tauri::command]
fn quit_app(app: AppHandle) {
    app.exit(0);
}

/// Tauri 应用入口
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();

            // 初始化数据库
            let conn = init_db(&handle);
            app.manage(DbState(Mutex::new(conn)));

            // 加载配置
            let cfg = load_config(&handle);
            app.manage(ConfigState(Mutex::new(cfg.clone())));

            // 根据记忆 / 默认值初始化主窗口位置和大小
            if let Some(win) = app.get_webview_window("main") {
                let scale = win.scale_factor().unwrap_or(1.0);

                // 悬浮球位置
                let (bx, by) = if cfg.ball_x >= 0 && cfg.ball_y >= 0 {
                    (cfg.ball_x, cfg.ball_y)
                } else {
                    // 默认放在主屏右下角
                    if let Ok(monitor) = win.current_monitor() {
                        if let Some(m) = monitor {
                            let mon_size = m.size();
                            let mon_pos = m.position();
                            let x = mon_pos.x + mon_size.width as i32
                                - (BALL_SIZE * scale).round() as i32
                                - 24;
                            let y = mon_pos.y + mon_size.height as i32
                                - (BALL_SIZE * scale).round() as i32
                                - 24;
                            (x, y)
                        } else {
                            (100, 100)
                        }
                    } else {
                        (100, 100)
                    }
                };
                let _ = win.set_position(PhysicalPosition::new(bx, by));
                let _ = win.set_size(PhysicalSize::new(
                    (BALL_SIZE * scale).round() as u32,
                    (BALL_SIZE * scale).round() as u32,
                ));
                let _ = win.set_resizable(false);

                // 把初始模式通知前端
                let _ = win.emit("mode-change", "ball");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_todos,
            add_todo,
            update_todo,
            delete_todo,
            clear_completed,
            get_config,
            save_window_config,
            collapse_to_ball,
            expand_to_panel,
            quit_app,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
