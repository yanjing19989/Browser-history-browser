use crate::config::AppConfig;
use crate::db::with_conn;
use crate::domain::{AppError, AppResult, HistoryItem, HistoryListResponse, OverviewStats};

#[derive(Debug, serde::Deserialize)]
pub struct HistoryFilters {
    pub keyword: Option<String>,
    pub time_range: Option<String>, // 7d / 30d / 90d / all
    pub locale: Option<String>,
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

    let sql_items = format!("SELECT url, title, last_visited_time, num_visits FROM navigation_history {} ORDER BY last_visited_time DESC LIMIT ?{} OFFSET ?{}", where_sql, param_index, param_index + 1);
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

    let mut where_clauses = Vec::new();
    let mut params = Vec::new();

    if let Some(lower) = ts_lower {
        where_clauses.push("last_visited_time >= ?".to_string());
        params.push(lower);
    }
    if let Some(upper) = ts_upper {
        where_clauses.push("last_visited_time <= ?".to_string());
        params.push(upper);
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
        let distinct_sql = format!("SELECT COUNT(*) FROM (SELECT substr(url,1, instr(url,'/',9)-1) AS host FROM navigation_history {} GROUP BY host)", where_sql);

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

    // Top entities
    let top_entities: Vec<String> = with_conn(|conn| {
        let entity_where = if where_clauses.is_empty() {
            "WHERE product_entity_id IS NOT NULL".to_string()
        } else {
            format!("{} AND product_entity_id IS NOT NULL", where_sql)
        };

        let entity_sql = format!("SELECT product_entity_id, SUM(num_visits) v FROM navigation_history {} GROUP BY product_entity_id ORDER BY v DESC LIMIT 5", entity_where);

        let mut stmt = conn.prepare(&entity_sql)?;
        let mut rows = if params.is_empty() {
            stmt.query([])?
        } else {
            stmt.query(rusqlite::params_from_iter(params.iter()))?
        };

        let mut acc = Vec::new();
        while let Some(row) = rows.next()? {
            let id: Option<String> = row.get(0)?;
            if let Some(s) = id {
                acc.push(s);
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
