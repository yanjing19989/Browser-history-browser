use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use once_cell::sync::OnceCell;
use std::sync::Mutex;

// AppConfig的全局内存缓存，以避免频繁的磁盘读取
static GLOBAL_CONFIG: OnceCell<Mutex<AppConfig>> = OnceCell::new();

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub db_path: Option<String>,
    pub browser_db_path: Option<String>,
    #[serde(default = "default_top_sites_count")]
    pub top_sites_count: u32,
    pub last_updated: i64,
}

fn default_top_sites_count() -> u32 {
    6
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_path: None,
            browser_db_path: None,
            top_sites_count: 6,
            last_updated: chrono::Utc::now().timestamp(),
        }
    }
}

impl AppConfig {
    fn config_file_path() -> Result<PathBuf> {
        let mut app_dir = tauri::api::path::app_config_dir(&tauri::Config::default())
            .ok_or_else(|| anyhow::anyhow!("无法获取应用配置目录"))?;
        app_dir.push("BrowserHistoryBrowser");
        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)?;
        }

        Ok(app_dir.join("config.json"))
    }

    pub fn load() -> Result<Self> {
        if let Some(m) = GLOBAL_CONFIG.get() {
            let cfg = m.lock().unwrap();
            return Ok(cfg.clone());
        }

        let config_path = Self::config_file_path()?;

        let config = if !config_path.exists() {
            let default_config = Self::default();
            let _ = serde_json::to_string_pretty(&default_config)?;
            default_config.save()?;
            default_config
        } else {
            let content = fs::read_to_string(&config_path)?;
            let config: Self = serde_json::from_str(&content)?;
            config
        };
        let _ = GLOBAL_CONFIG.set(Mutex::new(config.clone()));
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        // 更新全局内存缓存（如果存在）
        if let Some(m) = GLOBAL_CONFIG.get() {
            let mut g = m.lock().unwrap();
            *g = self.clone();
        } else {
            let _ = GLOBAL_CONFIG.set(Mutex::new(self.clone()));
        }

        Ok(())
    }

    pub fn set_db_path(&mut self, path: String) -> Result<()> {
        // 验证路径是否存在
        if !Path::new(&path).exists() {
            return Err(anyhow::anyhow!("数据库文件不存在: {}", path));
        }

        self.db_path = Some(path);
        self.last_updated = chrono::Utc::now().timestamp();
        self.save()?;
        Ok(())
    }

    pub fn get_db_path(&self) -> Option<String> {
        self.db_path.clone()
    }

    pub fn set_browser_db_path(&mut self, path: String) -> Result<()> {
        self.browser_db_path = Some(path);
        self.last_updated = chrono::Utc::now().timestamp();
        self.save()?;
        Ok(())
    }

    pub fn set_top_sites_count(&mut self, count: u32) -> Result<()> {
        if count == 0 || count > 50 {
            return Err(anyhow::anyhow!("TOP站点数量必须在1-50之间"));
        }
        self.top_sites_count = count;
        self.last_updated = chrono::Utc::now().timestamp();
        self.save()?;
        Ok(())
    }

    pub fn get_app_dir() -> Result<PathBuf> {
        let mut app_dir = tauri::api::path::app_config_dir(&tauri::Config::default())
            .ok_or_else(|| anyhow::anyhow!("无法获取应用数据目录"))?;
        app_dir.push("BrowserHistoryBrowser");
        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)?;
        }

        Ok(app_dir)
    }

    pub fn validate_db_path(path: &str) -> Result<bool> {
        let db_path = Path::new(path);

        if !db_path.exists() {
            return Err(anyhow::anyhow!("文件不存在"));
        }

        if !db_path.is_file() {
            return Err(anyhow::anyhow!("路径不是文件"));
        }

        // 简单验证是否为SQLite文件（检查文件头）
        let file_content = fs::read(db_path)?;
        if file_content.len() < 16 {
            return Err(anyhow::anyhow!("文件太小，不是有效的数据库文件"));
        }

        let sqlite_header = b"SQLite format 3\0";
        if &file_content[..16] != sqlite_header {
            return Err(anyhow::anyhow!("不是有效的SQLite数据库文件"));
        }

        Ok(true)
    }
}
