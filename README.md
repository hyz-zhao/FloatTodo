# FloatTodo · 悬浮待办

轻量级 Windows 桌面待办工具，悬浮球常驻桌面，一键呼出，随手记录。

## 特性

- **悬浮球入口** — 桌面右下角常驻，点击展开主面板，收起后不占屏幕
- **日期范围管理** — 支持今天、明天、本周、下周快捷切换，也可自定义日期范围
- **编辑感设计** — 纸白墨黑朱砂配色，衬线字体排版，简约美学
- **个人中心** — 按日/周查看历史记录，进度条可视化完成率
- **本地存储** — SQLite 数据库，数据存于项目 `data/` 目录，完全离线

## 技术栈

| 层 | 技术 |
|---|---|
| 桌面框架 | [Tauri 2](https://tauri.app/) |
| 前端 | Vue 3 + TypeScript + Vite |
| 后端 | Rust |
| 数据库 | SQLite (rusqlite) |
| UI 设计 | 编辑感简约美学（Fraunces + Manrope 字体） |

## 快速开始

```bash
# 安装依赖
npm install

# 启动开发模式
npm run tauri dev

# 或双击项目根目录的 start.bat
```

## 项目结构

```
FloatTodo/
├── src/                    # Vue 前端
│   ├── components/         # 组件
│   │   ├── FloatingBall.vue   # 悬浮球
│   │   ├── MainPanel.vue      # 主面板
│   │   ├── TodoItem.vue       # 待办事项条目
│   │   ├── DateRangePicker.vue # 日期范围选择器
│   │   └── HistoryView.vue    # 个人中心（历史记录）
│   ├── api.ts              # Tauri 后端 API 封装
│   ├── types.ts            # TypeScript 类型定义
│   └── date.ts             # 日期工具函数
├── src-tauri/              # Rust 后端
│   └── src/
│       ├── lib.rs          # 应用入口、窗口管理、命令注册
│       ├── db.rs           # SQLite 数据库操作
│       └── models.rs       # 数据结构定义
├── data/                   # 本地数据（已 gitignore）
│   ├── float-todo.db       # 数据库文件
│   └── config.json         # 窗口配置
├── start.bat               # 快捷启动脚本
└── package.json
```

## 许可

MIT