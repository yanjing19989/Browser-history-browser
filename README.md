# 浏览历史可视化 (Tauri)

本项目旨在提供一个本地、私密且高性能的浏览历史可视化与分析桌面应用，基于 **Tauri (Rust)** + **原生 HTML/CSS/JavaScript** + **SQLite (FTS5)**。最初设计文档中示例为 Vue3；当前骨架实现采用无框架原生前端，后续如需可平滑迁移到任意框架。

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
- Node.js 18+
- Rust (stable)
- pnpm 或 npm

### 2. 安装依赖
```powershell
pnpm install
```
（首次会下载 `@tauri-apps/cli`）

### 3. 启动开发
```powershell
pnpm dev
```
Tauri 将加载 `src/` 下的 `index.html`。

### 4. 构建发布
```powershell
pnpm build
```
生成二进制在 `src-tauri/target/release/`。

## 目录结构
```
root
 ├─ src/                # 原生前端资源 (index.html, style.css, main.js)
 ├─ src-tauri/
 │   ├─ src/            # Rust 源码 (db.rs, commands.rs, domain.rs, main.rs)
 │   ├─ Cargo.toml
 │   └─ tauri.conf.json
 ├─ DESIGN.md
 ├─ README.md
 └─ package.json
```

## 已实现 IPC 命令
| 命令 | 描述 |
|------|------|
| list_history | 分页 + 过滤（keyword、timeRange、locale）获取历史记录，返回 items/total |
| stats_overview | KPI：总访问次数、唯一站点数、Top 实体占位 |

前端在 `main.js` 中通过：
```js
const { invoke } = window.__TAURI__.tauri;
invoke('list_history', { page:1, pageSize:20, filters:{ timeRange:'7d' } });
```

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
