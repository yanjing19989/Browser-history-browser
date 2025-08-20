use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub db_path: Option<String>,
    pub last_updated: i64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_path: None,
            last_updated: chrono::Utc::now().timestamp(),
        }
    }
}

impl AppConfig {
    fn config_file_path() -> Result<PathBuf> {
        let app_dir = tauri::api::path::app_config_dir(tauri::generate_context!().config())
            .ok_or_else(|| anyhow::anyhow!("无法获取应用配置目录"))?;

        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)?;
        }

        Ok(app_dir.join("config.json"))
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;

        if !config_path.exists() {
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(&config_path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
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
