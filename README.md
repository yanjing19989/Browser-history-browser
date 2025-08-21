# 浏览历史可视化 (Tauri)

本项目旨在提供一个本地、私密且高性能的浏览历史可视化与分析桌面应用，基于 **Tauri (Rust)** + **原生 HTML/CSS/JavaScript** + **SQLite (FTS5)**。当前实现采用无框架原生前端，提供简洁高效的用户界面。

## 功能亮点
- 时间维度：按天/周访问趋势（后端接口预留）
- 列表浏览 + 多维过滤（时间范围、关键字、locale）+ 排序功能
- **增强分页功能**：支持页码直接输入、快速跳转和页面状态同步
- 全文搜索（后续接入 FTS5）
- 实体 / 品类 分布分析（接口预留）
- 图表联动（待接入图表库）
- **主题切换**：支持自动/浅色/深色三种模式，跟随系统偏好
- **详细信息面板**：点击条目查看详情，支持复制标题/链接、在浏览器中打开
- **现代化UI设计**：毛玻璃效果、半透明背景、圆角设计等视觉增强
- **浏览器数据库同步**：支持导入Chrome、Edge、Firefox、Safari等浏览器历史数据
- **统计仪表板**：统一KPI卡片显示，包含热门站点排行和改进的指标计算
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
 │   ├─ theme.js               # 主题管理系统
 │   ├─ theme-init.js          # 主题初始化脚本
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
| list_history | 分页 + 过滤（keyword、timeRange、locale）+ 排序（sort_by、sort_order）获取历史记录，返回 items/total |
| stats_overview | KPI：总访问次数、唯一站点数、Top 实体占位，改进的站点域名提取逻辑 |
| get_config | 获取应用配置信息 |
| set_db_path | 设置数据库文件路径 |
| validate_db_path | 验证数据库路径是否有效 |
| browse_db_file | 打开文件对话框选择数据库文件 |
| browse_browser_db_file | 打开文件对话框选择浏览器数据库文件 |
| copy_browser_db_to_app | 将浏览器数据库复制到应用数据目录 |
| set_browser_db_path | 设置浏览器数据库为应用数据源 |
| open_db_directory | 在文件管理器中打开数据库目录 |
| cleanup_old_dbs | 自动清理旧的数据库文件 |

前端在 `main.js` 中通过：
```js
const { invoke } = window.__TAURI__.tauri;
// 基础查询
invoke('list_history', { page:1, pageSize:20, filters:{ timeRange:'7d' } });
// 带排序的查询
invoke('list_history', { 
  page:1, 
  pageSize:20, 
  filters:{ 
    timeRange:'7d',
    sort_by: 'last_visited_time',
    sort_order: 'desc'
  } 
});
```

## 功能特性
- **历史浏览**：支持分页浏览历史记录，按时间降序排列
- **多维过滤**：按关键字、时间范围、locale 过滤历史记录
- **排序功能**：支持按标题、最后访问时间、访问次数排序，可切换升序/降序
- **增强分页**：页码直接输入、快速跳转、页面状态同步和边界验证
- **统计概览**：显示总访问次数、唯一站点数和热门实体，统一KPI卡片设计
- **详细信息**：点击历史记录查看详情，支持复制标题/链接、在默认浏览器中打开
- **主题管理**：自动/浅色/深色三种主题模式，支持跟随系统偏好，防闪烁加载
- **现代化UI**：毛玻璃效果、半透明背景、圆角设计和视觉增强
- **浏览器数据库同步**：支持选择、复制和设置Chrome、Edge、Firefox、Safari等浏览器历史数据库
- **配置管理**：支持自定义数据库路径，通过设置页面进行配置
- **文件对话框**：提供友好的文件选择界面
- **数据库管理**：数据库目录打开、旧文件自动清理等实用功能
- **用户反馈**：操作完成后显示提示信息，支持成功和错误状态

## CI/CD
项目已配置 GitHub Actions 自动化工作流，支持：
- **自动构建**：在推送到 main 分支和提交 PR 时自动触发
- **代码质量检查**：
  - Rust 代码格式检查 (`cargo fmt --check`)
  - Rust 代码 linting (`cargo clippy`)
  - Rust 单元测试 (`cargo test`)
- **Windows 构建**：生成可执行文件和 MSI 安装包
- **构建产物上传**：将构建结果保存为 artifacts，保留 30 天
- **自动发布**：创建 release 时自动上传构建产物

工作流配置位于 `.github/workflows/ci.yml`。

## 后续待办（与 DESIGN 对应）
- 引入 FTS5 同步触发器
- 增加更多统计接口：趋势、热力、实体分布
- UI：时间轴、图表组件（可选 ECharts 静态版）
- 国际化与更多主题选项
- 安全：主密码与配置加密
- 键盘快捷键增强（当前支持 Ctrl+K 聚焦搜索）

## 许可证
MIT
