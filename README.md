# 浏览历史可视化 (Tauri)

本项目旨在提供一个本地、私密且高性能的浏览历史可视化与分析桌面应用，基于 **Tauri (Rust)** + **原生 HTML/CSS/JavaScript** + **SQLite (FTS5)**。当前实现采用无框架原生前端，提供简洁高效的用户界面。

## 功能亮点
- 时间维度：按天/周访问趋势（后端接口预留）
- 列表浏览 + 简单过滤（时间范围、关键字、locale）
- 全文搜索（后续接入 FTS5）
- 实体 / 品类 分布分析（接口预留）
- 图表联动（待接入图表库）
- 国际化（预留）
- 本地隐私：数据不外发

## 快速开始
### 1. 环境准备
- **Node.js** 18+ (推荐使用 LTS 版本)
- **Rust** (stable 工具链)
- **包管理器**: npm 或 pnpm
- **系统依赖**: 
  - Windows: 无特殊要求
  - macOS: 无特殊要求  
  - Linux: 可能需要安装 GTK/WebKit 相关库

### 2. 安装依赖
```bash
npm install
# 或者使用 pnpm
pnpm install
```
（首次会下载 `@tauri-apps/cli`）

### 3. 启动开发
```bash
npm run dev
# 或者使用 pnpm
pnpm dev
```
Tauri 将加载 `src/` 下的 `index.html`。

### 4. 构建发布
```bash
npm run build
# 或者使用 pnpm
pnpm build
```
生成二进制在 `src-tauri/target/release/`。

## 目录结构
```
root
 ├─ src/                        # 原生前端资源
 │   ├─ index.html             # 主应用界面
 │   ├─ main.js                # 主应用逻辑
 │   ├─ settings.html          # 设置页面
 │   ├─ settings.js            # 设置页面逻辑
 │   ├─ style.css              # 应用样式
 │   └─ favicon.ico            # 应用图标
 ├─ src-tauri/
 │   ├─ src/                   # Rust 源码
 │   │   ├─ main.rs            # 应用入口点
 │   │   ├─ commands.rs        # IPC 命令处理
 │   │   ├─ db.rs              # 数据库连接管理
 │   │   ├─ domain.rs          # 数据结构定义
 │   │   └─ config.rs          # 配置管理
 │   ├─ Cargo.toml
 │   └─ tauri.conf.json
 ├─ DESIGN.md                  # 详细设计文档
 ├─ README.md
 └─ package.json
```

## 已实现 IPC 命令
| 命令 | 描述 |
|------|------|
| list_history | 分页 + 过滤（keyword、timeRange、locale）获取历史记录，返回 items/total |
| stats_overview | KPI：总访问次数、唯一站点数、Top 实体占位 |
| get_config | 获取应用配置信息 |
| set_db_path | 设置数据库文件路径 |
| validate_db_path | 验证数据库路径是否有效 |
| browse_db_file | 打开文件对话框选择数据库文件 |

前端在 `main.js` 中通过：
```js
const { invoke } = window.__TAURI__.tauri;
invoke('list_history', { page:1, pageSize:20, filters:{ timeRange:'7d' } });
```

## 功能特性
- **历史浏览**：支持分页浏览历史记录，按时间降序排列
- **多维过滤**：按关键字、时间范围、locale 过滤历史记录
- **统计概览**：显示总访问次数、唯一站点数和热门实体
- **配置管理**：支持自定义数据库路径，通过设置页面进行配置
- **文件对话框**：提供友好的文件选择界面

## 后续待办（与 DESIGN 对应）
- 引入 FTS5 同步触发器
- 增加更多统计接口：趋势、热力、实体分布
- UI：时间轴、图表组件（可选 ECharts 静态版）
- 国际化与主题切换
- 安全：主密码与配置加密

## 注意
当前 `db.rs` 会在本地生成 `history.db` 并写入 50 条示例数据。如需连接真实库，可修改路径或移除样例插入逻辑。

## 许可证
MIT（可调整）。
