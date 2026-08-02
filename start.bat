@echo off
chcp 65001 >nul
cd /d "%~dp0"

echo ================================
echo   FloatTodo - 悬浮待办
echo ================================
echo.

:: 关闭已运行的应用
echo [1/2] 检查并关闭旧进程...
taskkill /f /im float-todo.exe >nul 2>&1
echo        已完成

:: 启动开发模式
echo [2/2] 启动应用...
echo.
npm run tauri dev
pause