# 浏览历史可视化应用设计文档

## 1. 概述
一个基于 Tauri（Rust + 前端 Web 技术）的本地浏览历史可视化与洞察工具。读取/管理本地 SQLite 历史数据库，提供高性能搜索、过滤、统计分析与可视化交互，帮助用户理解自己的浏览模式、内容主题与时间分布，同时支持产品实体(Product Entity)维度的语义聚合。

### 目标
- 直观：时间轴 / 图表 / 列表 三视图联动，快速定位信息。
- 高性能：万级 ~ 百万级记录可流畅查询（分页 + 索引 + 预聚合）。
- 语义增强：利用 `product_entities` 进行类别、实体、关键词聚合分析。
- 隐私安全：数据全程本地处理，可选加密，最小权限。
- 可扩展：模块化 IPC API，后续可接入标签、笔记、AI 总结等高级功能。

### 非目标（初版不做）
- 云同步 / 多设备协同
- 在线行为追踪或上传
- 大规模机器学习建模

## 2. 用户与使用场景
| 角色 | 需求 | 典型场景 |
|------|------|----------|
| 个人高级用户 | 自我时间管理、主题统计 | 复盘一周时间分配、找回某次技术文档 |
| 产品/运营分析者（本地数据） | 按实体/类别研究兴趣走向 | 观察一段时间对某竞品关注度 |
| 开发者 | 验证浏览器行为、调试 | 找特定请求页面访问序列 |

## 3. 现有 Schema 回顾
```
meta(key PK, value)
navigation_history(
  url PK, id, title, metadata, last_visited_time INTEGER, num_visits INTEGER,
  product_entity_id, locale, titledata, urldata, page_profile
)
product_entities(
  product_entity_id PK, category, entity, search_keywords
)
```
问题与改进点：
- `last_visited_time` 单值，不包含多次访问的时间序列。需要衍生访问时间线（可建扩展表或解析 metadata）。
- `num_visits` 聚合计数，无法区分日/周趋势，需派生统计。
- `metadata/titledata/urldata/page_profile` 结构未规范，建议 JSON 化解析。

## 4. 数据与索引设计
### 4.1 追加建议索引
```
CREATE INDEX IF NOT EXISTS idx_nav_last_time ON navigation_history(last_visited_time DESC);
CREATE INDEX IF NOT EXISTS idx_nav_entity ON navigation_history(product_entity_id);
CREATE INDEX IF NOT EXISTS idx_nav_locale ON navigation_history(locale);
CREATE INDEX IF NOT EXISTS idx_entity_category ON product_entities(category);
```
如需模糊搜索（title/url/keywords），启用 SQLite FTS5 虚表：
```
CREATE VIRTUAL TABLE IF NOT EXISTS nav_search USING fts5(
  url, title, titledata, urldata, content='navigation_history', content_rowid='rowid'
);
-- 触发器同步（插入/更新/删除）
```
### 4.2 物化视图 / 预聚合表（可延后）
```
CREATE TABLE IF NOT EXISTS daily_visits(
  day DATE PRIMARY KEY,
  total_visits INTEGER,
  distinct_sites INTEGER
);
```
更新策略：应用启动懒更新 + 增量（基于 last_visited_time > last_computed）。

### 4.3 数据访问模式
- 分页查询：`LIMIT ? OFFSET ?`（时间倒序 / 过滤后）。
- 时间范围过滤：`WHERE last_visited_time BETWEEN ? AND ?`。
- 按实体聚合：`SELECT product_entity_id, SUM(num_visits) ... GROUP BY`。
- 搜索：FTS5（优先）或 LIKE 回退。

## 5. 系统架构
```
+---------------------------+
|        Frontend          |
|  - 视图：Dashboard / ...  |
|  - 组件：表格/图表/时间轴  |
+-------------+-------------+
              | (IPC Commands Events)
+-------------v-------------+
|        Tauri (Rust)       |
|  Commands: query, stats   |
|  Services: DB, Search     |
|  Modules: repo, cache     |
+-------------+-------------+
              |
+-------------v-------------+
|        SQLite (Local)     |
+---------------------------+
```
模块划分（Rust）：
- `db`: 连接池(sqlite + rusqlite/sea-query)。
- `repo`: 具体查询实现（history_repo, entity_repo, stats_repo）。
- `search`: FTS5 封装。
- `service`: 聚合业务逻辑 + 缓存（eg. LRU for hot queries）。
- `ipc`: 命令暴露与参数校验。
- `domain`: 结构体 DTO / Error 类型。

## 6. IPC/API 定义（初版）
| 命令 | 参数 | 描述 | 返回 |
|------|------|------|------|
| `list_history` | `page, page_size, filters{keyword, entity_id, locale, time_range}` | 分页历史 | `items[], total` |
| `get_history_detail` | `url` | 取单条，解析扩展字段 | `HistoryDetail` |
| `search_suggest` | `q, limit` | 前缀/模糊建议 | `suggestions[]` |
| `stats_overview` | `time_range` | 总览：访问次数、不同站点、Top5实体 | `OverviewStats` |
| `trend_visits` | `granularity=day|week` | 访问趋势 | `[{ts, visits}]` |
| `entity_distribution` | `time_range` | 实体/类别分布 | `[{entity/category, visits}]` |
| `heatmap_hours` | `time_range` | 小时*星期热力 | `int[7][24]` |
| `reindex_search` | - | 重建 FTS | 状态 |

错误处理：统一 `Result<T, AppError>`，前端分类（网络/参数/内部）。

## 7. 前端信息架构与页面
1. Dashboard 仪表盘
   - KPI 卡片：总访问、站点数、Top 实体
   - 趋势折线 + 实体分布饼图 + 时间热力
2. History 列表 & 时间轴
   - 左：过滤面板（时间范围快捷、实体、多选 locale）
   - 中：虚拟滚动列表（按时间分组头：日期）
   - 右：详情抽屉（标题、URL、访问次数、首次/最后时间、关联实体、关键词 chips）
   - 时间轴模式：放大/缩小、拖动、范围刷选
3. 实体 Insights
   - 实体搜索/选择
   - 访问趋势、相关 URL 列表、关键词云（可后续）
4. 设置
   - 数据库路径校验 / 切换
   - FTS 重建
   - 隐私（加密开关、自动清理策略）
   - 语言切换

## 8. 组件与交互（UI 细节）
- 主色：#2563eb (Primary)，强调色 #f59e0b，背景浅灰 #f5f7fa，深色模式支持。
- 表格：虚拟滚动（Vue Virtual Scroller），行 hover 高亮，点击行 -> 详情抽屉。
- 图表：ECharts，联动：点击饼图过滤列表；热力图框选 -> 时间范围更新趋势。
- 搜索框：即时建议（防抖 250ms）。
- 时间范围快捷：今天 / 过去7天 / 过去30天 / 本月 / 自定义。
- 键盘快捷：`Ctrl+K` 聚焦搜索，`↑/↓` 选择建议。

## 9. 状态管理（Pinia）
Stores：
- `useHistoryStore`: 列表数据、分页、loading、当前过滤条件 hash。
- `useStatsStore`: 缓存概览与趋势（key: time_range+granularity）。
- `useEntityStore`: 实体元数据、id->entity 映射。
- `useUIStore`: 主题、侧栏状态、语言。
缓存策略：
- 最近 5 个过滤组合的列表缓存。
- 统计数据 10 分钟 TTL。

## 10. 性能策略
- 数据库：准备语句 + 连接复用；开启 WAL；`PRAGMA synchronous = NORMAL`。
- 分页：前端 page_size 默认 100，支持无限滚动动态加载。
- 列表：虚拟滚动，仅渲染视窗 ~ 20 行。
- 预聚合：启动懒更新；高频统计走内存缓存。
- FTS：最少字段，必要时分词算法 porter / unicode61。

## 11. 安全与隐私
- 数据仅本地；可选“启动需主密码”，利用派生密钥（Argon2）对敏感缓存/配置加密。
- 不上传使用统计，默认关闭更新检查（可手动）。
- 日志分级，生产默认 INFO，敏感字段（URL 查询参数中的 token）脱敏正则替换。

## 12. 国际化
- i18n JSON：`locales/en-US/*.json`, `locales/zh-CN/*.json`。
- 文案命名：`history.list.empty`, `stats.kpi.totalVisits`。
- 日期与数字格式：使用 dayjs + Intl API。


## 14. 构建与开发流程
目录建议：
```
root
 ├─ src-tauri
 │   ├─ src
 │   │   ├─ main.rs
 │   │   ├─ db/
 │   │   ├─ repo/
 │   │   ├─ service/
 │   │   ├─ ipc/
 │   │   └─ domain/
 │   └─ Cargo.toml
 ├─ src
 │   ├─ main.js
 │   ├─ index.html
 │   └─ style.css
 ├─ tests (frontend)
 ├─ package.json
 └─ DESIGN.md
```
CI：
- GitHub Actions：Rust fmt + clippy + cargo test；Node pnpm build；Playwright（可选 matrix OS）。

## 15. 路线图
阶段 1 (MVP 2 周)：基础查询、分页、仪表盘简版、FTS 搜索、实体分布、i18n。
阶段 2 (增强 4 周)：热力图、时间轴可视化、预聚合、缓存、快捷键、暗黑模式。
阶段 3 (扩展 4+ 周)：AI 总结、标签/笔记体系、导出报告、插件机制。

## 16. 未来扩展想法
- 接入浏览器实时监控（扩展）-> WebSocket 推送增量刷新。
- Embedding 相似页面聚类。
- 本地 LLM 摘要（用户选择记录批量总结）。

## 17. 开发起步 Checklist
- [ ] 初始化 Tauri + Vue3 + TypeScript
- [ ] Rust 引入 rusqlite / serde / anyhow
- [ ] 建立迁移与索引创建逻辑
- [ ] FTS5 可用性检测 + 虚表创建
- [ ] 定义 domain structs 与 IPC commands
- [ ] 前端路由 + 主题 + Pinia + ECharts
- [ ] 首个命令：list_history + UI 列表渲染
- [ ] 添加 stats_overview 图表

---
(完)
