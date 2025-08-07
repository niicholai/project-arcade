use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: i64,
    pub title: String,
    pub igdb_id: Option<i64>,
    pub source_path: String,
    pub install_path: Option<String>,
    pub status: String, // e.g., 'Ready to Install', 'Installed', 'Updating'
    
    // Metadata fields
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub banner_url: Option<String>,
    pub release_date: Option<String>, // Using String for simplicity
    pub developer: Option<String>,
    pub publisher: Option<String>,
    pub genre: Option<String>,
    pub themes: Option<String>, // Comma-separated
    pub game_modes: Option<String>, // Comma-separated
    pub tags: Option<String>, // Comma-separated
    pub metacritic_score: Option<i32>,
    pub steam_rating_percent: Option<i32>,
    pub steam_rating_text: Option<String>,
    pub age_rating: Option<String>, // ESRB rating like "E10+", "T", "M"
    pub screenshots: Option<String>, // JSON array of screenshot URLs
    pub videos: Option<String>, // JSON array of video IDs
    pub time_to_beat: Option<i32>, // In hours
    pub install_size: Option<i64>, // In bytes
}
