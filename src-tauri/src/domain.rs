use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryItem {
    pub url: String,
    pub title: Option<String>,
    pub last_visited_time: i64, // epoch seconds
    pub num_visits: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryListResponse {
    pub items: Vec<HistoryItem>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverviewStats {
    pub total_visits: i64,
    pub distinct_sites: i64,
    pub top_entities: Vec<String>,
}

#[derive(thiserror::Error, Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum AppError {
    #[error("Database error: {0}")]
    Db(String),
    #[error("Invalid arguments: {0}")]
    Invalid(String),
    #[error("Internal: {0}")]
    Internal(String),
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        Self::Db(e.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
