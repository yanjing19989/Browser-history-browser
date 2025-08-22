use crate::config::AppConfig;
use once_cell::sync::Lazy;
use rusqlite::{params, Connection};
use std::sync::Mutex;

static CONN: Lazy<Mutex<Option<Connection>>> = Lazy::new(|| Mutex::new(None));

fn get_or_create_connection() -> rusqlite::Result<()> {
    let mut conn_guard = CONN.lock().unwrap();

    if conn_guard.is_none() {
        // 尝试从配置加载数据库路径
        let config = AppConfig::load().unwrap_or_default();
        let db_path = config.get_db_path().unwrap_or_else(|| {
            AppConfig::get_app_dir()
                .map(|app_dir| {
                    app_dir
                        .join("history_test.db")
                        .to_string_lossy()
                        .to_string()
                })
                .unwrap_or_else(|_| "history_test.db".to_string())
        });
        let conn = Connection::open(&db_path)?;
        // 基础性能设置
        conn.pragma_update(None, "journal_mode", "WAL").ok();
        conn.pragma_update(None, "synchronous", "NORMAL").ok();

        if db_path.ends_with("history_test.db") {
            init_schema(&conn)?;
        }

        *conn_guard = Some(conn);
    }

    Ok(())
}

pub fn reset_connection(new_path: &str) -> rusqlite::Result<()> {
    let mut conn_guard = CONN.lock().unwrap();

    // 关闭现有连接
    *conn_guard = None;

    // 创建新连接
    let conn = Connection::open(new_path)?;
    conn.pragma_update(None, "journal_mode", "WAL").ok();
    conn.pragma_update(None, "synchronous", "NORMAL").ok();

    *conn_guard = Some(conn);
    Ok(())
}

fn init_schema(conn: &Connection) -> rusqlite::Result<()> {
    // 仅创建演示表（若真实已有表，可删除此段或改为检测）
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS navigation_history (
            url TEXT PRIMARY KEY,
            id INTEGER,
            title TEXT,
            metadata TEXT,
            last_visited_time INTEGER NOT NULL DEFAULT 0,
            num_visits INTEGER NOT NULL DEFAULT 1,
            product_entity_id TEXT,
            locale TEXT,
            titledata TEXT,
            urldata TEXT,
            page_profile TEXT
        );
        CREATE INDEX IF NOT EXISTS idx_nav_last_time ON navigation_history(last_visited_time DESC);
        "#,
    )?;

    // 若表为空，插入一些样例数据
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM navigation_history", [], |r| r.get(0))?;
    if count == 0 {
        let now = chrono::Utc::now().timestamp();
        for i in 0..50 {
            // 简单样例
            conn.execute(
                "INSERT OR REPLACE INTO navigation_history(url, id, title, last_visited_time, num_visits, locale) VALUES (?,?,?,?,?,?)",
                params![
                    format!("https://example.com/page{}", i),
                    i as i64,
                    format!("示例页面 {}", i),
                    now - (i as i64 * 3600),
                    (i % 7 + 1) as i64,
                    if i % 2 == 0 { "en-us" } else { "zh-cn" }
                ],
            )?;
        }
    }
    Ok(())
}

pub fn with_conn<F, T>(f: F) -> Result<T, rusqlite::Error>
where
    F: FnOnce(&Connection) -> Result<T, rusqlite::Error>,
{
    get_or_create_connection()?;

    let guard = CONN.lock().unwrap();
    match guard.as_ref() {
        Some(conn) => f(conn),
        None => Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
            Some("数据库连接未初始化".to_string()),
        )),
    }
}
