use crate::config::AppConfig;
use crate::db::with_conn;
use crate::domain::{AppError, AppResult, HistoryItem, HistoryListResponse, OverviewStats};

#[derive(Debug, serde::Deserialize)]
pub struct HistoryFilters {
    pub keyword: Option<String>,
    pub time_range: Option<String>, // 7d / 30d / 90d / all
    pub locale: Option<String>,
    pub sort_by: Option<String>, // title, last_visited_time, num_visits
    pub sort_order: Option<String>, // asc, desc
}

fn compute_time_lower(bound: &Option<String>) -> Option<i64> {
    let now = chrono::Utc::now().timestamp();
    match bound.as_deref() {
        Some("7d") => Some(now - 7 * 86400),
        Some("30d") => Some(now - 30 * 86400),
        Some("90d") => Some(now - 90 * 86400),
        Some("all") | None => None,
        Some(range) => {
            // 处理自定义范围格式: "startTs-endTs"
            if let Some(dash_pos) = range.find('-') {
                if let Ok(start_ts) = range[..dash_pos].parse::<i64>() {
                    return Some(start_ts);
                }
            }
            None
        }
    }
}

fn compute_time_upper(bound: &Option<String>) -> Option<i64> {
    match bound.as_deref() {
        Some(range) => {
            // 处理自定义范围格式: "startTs-endTs"
            if let Some(dash_pos) = range.find('-') {
                if let Ok(end_ts) = range[dash_pos + 1..].parse::<i64>() {
                    return Some(end_ts);
                }
            }
            None
        }
        _ => None,
    }
}

fn build_order_clause(sort_by: &Option<String>, sort_order: &Option<String>) -> String {
    let sort_field = match sort_by.as_deref() {
        Some("title") => "title",
        Some("num_visits") => "num_visits",
        Some("last_visited_time") | None => "last_visited_time", // 默认按访问时间排序
        _ => "last_visited_time",
    };

    let order = match sort_order.as_deref() {
        Some("asc") => "ASC",
        Some("desc") | None => "DESC", // 默认降序
        _ => "DESC",
    };

    format!("ORDER BY {} {}", sort_field, order)
}

#[tauri::command]
pub fn list_history(
    page: u32,
    page_size: u32,
    filters: HistoryFilters,
) -> AppResult<HistoryListResponse> {
    if page_size == 0 || page_size > 500 {
        return Err(AppError::Invalid("page_size out of range".into()));
    }
    let offset = (page.saturating_sub(1) * page_size) as i64;

    let mut where_clauses: Vec<String> = Vec::new();
    let mut params_dyn: Vec<rusqlite::types::Value> = Vec::new();
    let mut param_index = 1;

    if let Some(ts_lower) = compute_time_lower(&filters.time_range) {
        where_clauses.push(format!("last_visited_time >= ?{}", param_index));
        params_dyn.push(ts_lower.into());
        param_index += 1;
    }
    if let Some(ts_upper) = compute_time_upper(&filters.time_range) {
        where_clauses.push(format!("last_visited_time <= ?{}", param_index));
        params_dyn.push(ts_upper.into());
        param_index += 1;
    }
    if let Some(locale) = &filters.locale {
        if !locale.is_empty() {
            where_clauses.push(format!("locale = ?{}", param_index));
            params_dyn.push(locale.clone().into());
            param_index += 1;
        }
    }
    if let Some(kw) = &filters.keyword {
        if !kw.is_empty() {
            where_clauses.push(format!(
                "(title LIKE ?{} OR url LIKE ?{})",
                param_index,
                param_index + 1
            ));
            params_dyn.push(format!("%{}%", kw).into());
            params_dyn.push(format!("%{}%", kw).into());
            param_index += 2;
        }
    }

    let where_sql = if where_clauses.is_empty() {
        "".into()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };

    let order_clause = build_order_clause(&filters.sort_by, &filters.sort_order);
    let sql_items = format!(
        "SELECT url, title, last_visited_time, num_visits FROM navigation_history {} {} LIMIT ?{} OFFSET ?{}",
        where_sql, order_clause, param_index, param_index + 1
    );
    let sql_count = format!("SELECT COUNT(*) FROM navigation_history {}", where_sql);

    let items = with_conn(|conn| {
        let mut stmt = conn.prepare(&sql_items)?;

        // 构建完整的参数向量：dynamic params + limit + offset
        let mut all_params: Vec<rusqlite::types::Value> = params_dyn.clone();
        all_params.push((page_size as i64).into());
        all_params.push(offset.into());

        let mut rows = stmt.query(rusqlite::params_from_iter(all_params.iter()))?;
        let mut acc = Vec::new();
        while let Some(row) = rows.next()? {
            acc.push(HistoryItem {
                url: row.get(0)?,
                title: row.get(1)?,
                last_visited_time: row.get(2)?,
                num_visits: row.get(3)?,
            });
        }
        Ok::<_, rusqlite::Error>(acc)
    })?;

    let total: i64 = with_conn(|conn| {
        let mut stmt = conn.prepare(&sql_count)?;
        let val: i64 =
            stmt.query_row(rusqlite::params_from_iter(params_dyn.iter()), |r| r.get(0))?;
        Ok(val)
    })?;

    Ok(HistoryListResponse { items, total })
}

#[tauri::command]
pub fn stats_overview(time_range: Option<String>) -> AppResult<OverviewStats> {
    let ts_lower = compute_time_lower(&time_range);
    let ts_upper = compute_time_upper(&time_range);

    let mut where_clauses: Vec<String> = Vec::new();
    let mut params: Vec<rusqlite::types::Value> = Vec::new();

    if let Some(lower) = ts_lower {
        where_clauses.push("last_visited_time >= ?".to_string());
        params.push(lower.into());
    }
    if let Some(upper) = ts_upper {
        where_clauses.push("last_visited_time <= ?".to_string());
        params.push(upper.into());
    }

    let where_sql = if where_clauses.is_empty() {
        "".to_string()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };

    let (total_visits, distinct_sites) = with_conn(|conn| {
        let total_sql = format!(
            "SELECT SUM(num_visits) FROM navigation_history {}",
            where_sql
        );
        let distinct_sql = format!("SELECT COUNT(*) FROM (SELECT CASE WHEN instr(substr(url, 9), '/') > 0 THEN substr(url, 1, instr(substr(url, 9), '/') + 7) ELSE url END AS host FROM navigation_history {} GROUP BY host)", where_sql);

        let total: i64 = if params.is_empty() {
            conn.query_row(&total_sql, [], |r| r.get::<_, Option<i64>>(0))?
                .unwrap_or(0)
        } else {
            conn.query_row(&total_sql, rusqlite::params_from_iter(params.iter()), |r| {
                r.get::<_, Option<i64>>(0)
            })?
            .unwrap_or(0)
        };

        let distinct: i64 = if params.is_empty() {
            conn.query_row(&distinct_sql, [], |r| r.get(0))?
        } else {
            conn.query_row(
                &distinct_sql,
                rusqlite::params_from_iter(params.iter()),
                |r| r.get(0),
            )?
        };

        Ok::<_, rusqlite::Error>((total, distinct))
    })?;

    // Top entities - 改为返回站点名称
    let top_entities: Vec<String> = with_conn(|conn| {
        // 提取站点域名并按访问次数排序
        let site_sql = format!(
            "SELECT 
                CASE 
                    WHEN url LIKE 'http://%' THEN 
                        CASE 
                            WHEN instr(substr(url, 8), '/') > 0 
                            THEN substr(url, 8, instr(substr(url, 8), '/') - 1)
                            ELSE substr(url, 8)
                        END
                    WHEN url LIKE 'https://%' THEN 
                        CASE 
                            WHEN instr(substr(url, 9), '/') > 0 
                            THEN substr(url, 9, instr(substr(url, 9), '/') - 1)
                            ELSE substr(url, 9)
                        END
                    ELSE url
                END as site_name,
                SUM(num_visits) as total_visits 
            FROM navigation_history 
            {} 
            GROUP BY site_name 
            ORDER BY total_visits DESC 
            LIMIT 6",
            where_sql
        );

        let mut stmt = conn.prepare(&site_sql)?;
        let mut rows = if params.is_empty() {
            stmt.query([])?
        } else {
            stmt.query(rusqlite::params_from_iter(params.iter()))?
        };

        let mut acc = Vec::new();
        while let Some(row) = rows.next()? {
            let site_name: String = row.get(0)?;
            if !site_name.is_empty() {
                acc.push(site_name);
            }
        }
        Ok::<_, rusqlite::Error>(acc)
    })?;

    Ok(OverviewStats {
        total_visits,
        distinct_sites,
        top_entities,
    })
}

// 配置相关命令
#[tauri::command]
pub fn get_config() -> AppResult<AppConfig> {
    AppConfig::load().map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn set_db_path(path: String) -> AppResult<String> {
    // 验证路径
    AppConfig::validate_db_path(&path)
        .map_err(|e| AppError::Invalid(format!("数据库路径验证失败: {}", e)))?;

    // 加载配置并设置新路径
    let mut config = AppConfig::load().map_err(|e| AppError::Internal(e.to_string()))?;

    config
        .set_db_path(path.clone())
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // 重新初始化数据库连接
    crate::db::reset_connection(&path).map_err(|e| AppError::Db(e.to_string()))?;

    Ok("数据库路径设置成功".to_string())
}

#[tauri::command]
pub fn validate_db_path(path: String) -> AppResult<bool> {
    AppConfig::validate_db_path(&path).map_err(|e| AppError::Invalid(e.to_string()))
}

#[tauri::command]
pub fn browse_db_file() -> AppResult<Option<String>> {
    use tauri::api::dialog::blocking::FileDialogBuilder;

    let file_path = FileDialogBuilder::new()
        .add_filter("SQLite数据库", &["db", "sqlite", "sqlite3"])
        .add_filter("所有文件", &["*"])
        .set_title("选择浏览历史数据库文件")
        .pick_file();

    Ok(file_path.map(|p| p.to_string_lossy().to_string()))
}

#[tauri::command]
pub fn browse_browser_db_file() -> AppResult<Option<String>> {
    use tauri::api::dialog::blocking::FileDialogBuilder;

    let file_path = FileDialogBuilder::new()
        .add_filter("SQLite数据库", &["db", "sqlite", "sqlite3"])
        .add_filter("Chrome/Edge数据库", &["*"])
        .add_filter("Firefox数据库", &["sqlite"])
        .add_filter("所有文件", &["*"])
        .set_title("选择浏览器历史数据库文件")
        .pick_file();

    Ok(file_path.map(|p| p.to_string_lossy().to_string()))
}

#[tauri::command]
pub fn copy_browser_db_to_app(source_path: String) -> AppResult<String> {
    use std::fs;
    use std::path::Path;

    if !Path::new(&source_path).exists() {
        return Err(AppError::Invalid("源数据库文件不存在".to_string()));
    }

    let app_dir = AppConfig::get_app_dir().map_err(|e| AppError::Internal(e.to_string()))?;
    fs::create_dir_all(&app_dir)
        .map_err(|e| AppError::Internal(format!("创建应用目录失败: {}", e)))?;

    // 生成目标文件名 (带时间戳的history.db)
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let target_filename = format!("history_{}.db", timestamp);
    let target_path = app_dir.join(&target_filename);

    fs::copy(&source_path, &target_path)
        .map_err(|e| AppError::Internal(format!("复制数据库文件失败: {}", e)))?;

    AppConfig::validate_db_path(target_path.to_string_lossy().as_ref())
        .map_err(|e| AppError::Invalid(format!("复制的数据库文件无效: {}", e)))?;

    Ok(target_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn set_browser_db_path(path: String) -> AppResult<String> {
    // 验证路径
    if !std::path::Path::new(&path).exists() {
        return Err(AppError::Invalid("浏览器数据库文件不存在".to_string()));
    }

    // 加载配置并设置浏览器数据库路径
    let mut config = AppConfig::load().map_err(|e| AppError::Internal(e.to_string()))?;

    config
        .set_browser_db_path(path)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok("浏览器数据库路径保存成功".to_string())
}

#[tauri::command]
pub fn open_db_directory() -> AppResult<String> {
    let config = AppConfig::load().map_err(|e| AppError::Internal(e.to_string()))?;

    let db_path = config
        .get_db_path()
        .ok_or_else(|| AppError::Invalid("未配置数据库路径".to_string()))?;

    let path = std::path::Path::new(&db_path);
    let parent_dir = path
        .parent()
        .ok_or_else(|| AppError::Invalid("无法获取数据库文件所在目录".to_string()))?;

    // 使用系统默认程序打开目录
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(parent_dir)
            .spawn()
            .map_err(|e| AppError::Internal(format!("打开目录失败: {}", e)))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(parent_dir)
            .spawn()
            .map_err(|e| AppError::Internal(format!("打开目录失败: {}", e)))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(parent_dir)
            .spawn()
            .map_err(|e| AppError::Internal(format!("打开目录失败: {}", e)))?;
    }

    Ok("目录已打开".to_string())
}

#[tauri::command]
pub fn cleanup_old_dbs() -> AppResult<String> {
    use std::fs;
    use std::path::Path;

    let config = AppConfig::load().map_err(|e| AppError::Internal(e.to_string()))?;

    let db_path = config
        .get_db_path()
        .ok_or_else(|| AppError::Invalid("未配置数据库路径".to_string()))?;

    let current_db = Path::new(&db_path);
    let parent_dir = current_db
        .parent()
        .ok_or_else(|| AppError::Invalid("无法获取数据库文件所在目录".to_string()))?;

    let current_filename = current_db
        .file_name()
        .ok_or_else(|| AppError::Invalid("无法获取当前数据库文件名".to_string()))?;

    // 读取目录中的所有文件
    let entries =
        fs::read_dir(parent_dir).map_err(|e| AppError::Internal(format!("读取目录失败: {}", e)))?;

    let mut deleted_count = 0;

    for entry in entries {
        let entry = entry.map_err(|e| AppError::Internal(format!("读取目录项失败: {}", e)))?;
        let entry_path = entry.path();

        // 检查是否是.db文件
        if let Some(extension) = entry_path.extension() {
            if extension == "db" {
                // 检查是否是当前使用的数据库文件
                if let Some(file_name) = entry_path.file_name() {
                    if file_name != current_filename {
                        // 删除其他.db文件
                        if let Err(e) = fs::remove_file(&entry_path) {
                            eprintln!("删除文件失败 {:?}: {}", entry_path, e);
                        } else {
                            deleted_count += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(format!("已清理 {} 个旧数据库文件", deleted_count))
}
