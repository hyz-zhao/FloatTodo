// Windows 下使用 Ctrl + C 关闭控制台；release 模式下不显示控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    float_todo_lib::run()
}
