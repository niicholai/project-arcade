use crate::{models::Game, config::get_api_config, Result, error::Error};
use reqwest::Client;
use serde::Deserialize;
use sqlx::{Pool, Sqlite};
use tauri::{AppHandle, Manager};
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Rate limiting structures
struct RateLimiter {
    requests: Mutex<HashMap<String, Vec<u64>>>,
}

impl RateLimiter {
    fn new() -> Self {
        Self {
            requests: Mutex::new(HashMap::new()),
        }
    }

    fn can_make_request(&self, endpoint: &str, max_requests: u32, window_seconds: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut requests = self.requests.lock().unwrap();
        let requests_for_endpoint = requests.entry(endpoint.to_string()).or_insert_with(Vec::new);
        
        // Remove old requests outside the window
        requests_for_endpoint.retain(|&timestamp| now - timestamp < window_seconds);
        
        if requests_for_endpoint.len() < max_requests as usize {
            requests_for_endpoint.push(now);
            true
        } else {
            false
        }
    }
}

static RATE_LIMITER: once_cell::sync::Lazy<RateLimiter> = once_cell::sync::Lazy::new(RateLimiter::new);

// --- IGDB API Response Structs ---
// These match the actual IGDB API response structure

#[derive(Deserialize, Debug)]
struct IgdbGameData {
    name: String,
    summary: Option<String>,
    first_release_date: Option<i64>,
    cover: Option<IgdbCover>,
    screenshots: Option<Vec<IgdbScreenshot>>,
    videos: Option<Vec<IgdbVideo>>,
    genres: Option<Vec<IgdbGenre>>,
    themes: Option<Vec<IgdbTheme>>,
    involved_companies: Option<Vec<IgdbCompany>>,
    artworks: Option<Vec<IgdbArtwork>>,
}

#[derive(Deserialize, Debug)]
struct IgdbCover {
    url: String,
}

#[derive(Deserialize, Debug)]
struct IgdbScreenshot {
    url: String,
}

#[derive(Deserialize, Debug)]
struct IgdbArtwork {
    url: String,
}

#[derive(Deserialize, Debug)]
struct IgdbVideo {
    video_id: String,
    name: Option<String>,
}

#[derive(Deserialize, Debug)]
struct IgdbGenre {
    name: String,
}

#[derive(Deserialize, Debug)]
struct IgdbTheme {
    name: String,
}

#[derive(Deserialize, Debug)]
struct IgdbCompany {
    company: IgdbCompanyInfo,
    developer: bool,
    publisher: bool,
}

#[derive(Deserialize, Debug)]
struct IgdbCompanyInfo {
    name: String,
}

// --- Giant Bomb API Response Structs ---

#[derive(Deserialize, Debug)]
struct GiantBombGameData {
    results: Vec<GiantBombResult>,
}

#[derive(Deserialize, Debug)]
struct GiantBombResult {
    name: String,
    deck: Option<String>,
    original_release_date: Option<String>,
}

async fn get_igdb_token(client: &Client) -> Result<String> {
    let config = get_api_config();
    println!("Requesting IGDB token...");
    
    // Rate limiting: IGDB token requests - 4 requests per second (official limit)
    if !RATE_LIMITER.can_make_request("igdb_token", 4, 1) {
        return Err(Error::Config("Rate limit exceeded for IGDB token requests. Please wait before trying again.".to_string()));
    }
    
    let response = client
        .post("https://id.twitch.tv/oauth2/token")
        .query(&[
            ("client_id", &config.igdb_client_id),
            ("client_secret", &config.igdb_client_secret),
            ("grant_type", &"client_credentials".to_string()),
        ])
        .send()
        .await
        .map_err(|e| Error::Config(format!("Failed to get IGDB token: {}", e)))?;
    
    if !response.status().is_success() {
        return Err(Error::Config(format!("IGDB token request failed with status: {}", response.status())));
    }
    
    let token_data = response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| Error::Config(format!("Failed to parse IGDB token response: {}", e)))?;
    
    let access_token = token_data["access_token"]
        .as_str()
        .ok_or_else(|| Error::Config("No access_token in IGDB response".to_string()))?;
    
    println!("IGDB token obtained successfully");
    Ok(format!("Bearer {}", access_token))
}

async fn fetch_igdb_data(client: &Client, igdb_id: i64) -> Result<Option<IgdbGameData>> {
    let config = get_api_config();
    let token = get_igdb_token(client).await?;
    
    println!("Fetching IGDB data for game ID: {}", igdb_id);
    
    // Rate limiting: IGDB API requests - 4 requests per second (official limit)
    if !RATE_LIMITER.can_make_request("igdb_api", 4, 1) {
        return Err(Error::Config("Rate limit exceeded for IGDB API requests. Please wait before trying again.".to_string()));
    }
    
    // Build the IGDB query - using only valid IGDB API fields
    let query_body = format!(
        "fields name,summary,first_release_date,cover.url,screenshots.url,artworks.url,videos.video_id,videos.name,genres.name,themes.name,involved_companies.company.name,involved_companies.developer,involved_companies.publisher; where id = {};",
        igdb_id
    );
    
    let response = client
        .post("https://api.igdb.com/v4/games")
        .header("Client-ID", &config.igdb_client_id)
        .header("Authorization", &token)
        .header("Content-Type", "text/plain")
        .body(query_body)
        .send()
        .await
        .map_err(|e| Error::Config(format!("Failed to fetch IGDB data for game {}: {}", igdb_id, e)))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(Error::Config(format!("IGDB API request failed for game {}: {} - {}", igdb_id, status, error_text)));
    }
    
    let games: Vec<IgdbGameData> = response
        .json()
        .await
        .map_err(|e| Error::Config(format!("Failed to parse IGDB response for game {}: {}", igdb_id, e)))?;
    
    if games.is_empty() {
        println!("No IGDB data found for game ID: {}", igdb_id);
        Ok(None)
    } else {
        println!("Successfully fetched IGDB data for game: {}", games[0].name);
        Ok(games.into_iter().next())
    }
}

async fn fetch_giant_bomb_data(client: &Client, game_name: &str) -> Result<Option<GiantBombResult>> {
    let config = get_api_config();
    println!("Fetching Giant Bomb data for game: {}", game_name);
    
    // Rate limiting: Giant Bomb API requests - 200 requests per hour (official limit)
    if !RATE_LIMITER.can_make_request("giant_bomb_api", 200, 3600) {
        return Err(Error::Config("Rate limit exceeded for Giant Bomb API requests. Please wait before trying again.".to_string()));
    }
    
    let response = client
        .get("https://www.giantbomb.com/api/search")
        .query(&[
            ("api_key", &config.giant_bomb_api_key),
            ("format", &"json".to_string()),
            ("query", &game_name.to_string()),
            ("resources", &"game".to_string()),
        ])
        .send()
        .await
        .map_err(|e| Error::Config(format!("Failed to fetch Giant Bomb data for game '{}': {}", game_name, e)))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(Error::Config(format!("Giant Bomb API request failed for game '{}': {} - {}", game_name, status, error_text)));
    }
    
    let gb_data: GiantBombGameData = response
        .json()
        .await
        .map_err(|e| Error::Config(format!("Failed to parse Giant Bomb response for game '{}': {}", game_name, e)))?;
    
    if gb_data.results.is_empty() {
        println!("No Giant Bomb data found for game: {}", game_name);
        Ok(None)
    } else {
        println!("Successfully fetched Giant Bomb data for game: {}", gb_data.results[0].name);
        Ok(gb_data.results.into_iter().next())
    }
}

/// Fetches metadata from various sources using a waterfall approach and updates the database.
/// This function is designed to be run in a background task.
pub async fn fetch_and_update_metadata(
    game_id: i64,
    db: Pool<Sqlite>,
    app_handle: AppHandle,
) -> Result<()> {
    println!("Starting metadata fetch for game id {}", game_id);

    // 1. Get the current game state from the database.
    let mut game = sqlx::query_as::<_, Game>("SELECT * FROM games WHERE id = ?")
        .bind(game_id)
        .fetch_one(&db)
        .await
        .map_err(|e| Error::Config(format!("Failed to fetch game {} from database: {}", game_id, e)))?;

    println!("Fetched game from database: '{}' (IGDB ID: {:?})", game.title, game.igdb_id);

    // 2. Create a single HTTP client to be reused for all requests.
    let client = Client::new();

    // Track whether any metadata was actually updated
    let mut metadata_updated = false;

    // 3. --- The Waterfall ---
    // Try IGDB first, then fall back to Giant Bomb if needed
    if let Some(igdb_id) = game.igdb_id {
        println!("Attempting to fetch IGDB metadata for game ID: {}", igdb_id);
        println!("Current game title: '{}'", game.title);
        
        match fetch_igdb_data(&client, igdb_id).await {
            Ok(Some(igdb_data)) => {
                println!("Successfully fetched IGDB data for '{}'", igdb_data.name);
                println!("IGDB data debug: screenshots={:?}, videos={:?}, themes={:?}", 
                    igdb_data.screenshots.as_ref().map(|s| s.len()), 
                    igdb_data.videos.as_ref().map(|v| v.len()),
                    igdb_data.themes.as_ref().map(|t| t.len())
                );
                
                // Always update the game title with the real name from IGDB
                println!("Updating title from '{}' to '{}'", game.title, igdb_data.name);
                game.title = igdb_data.name;
                metadata_updated = true;
                
                // Update description
                if let Some(summary) = &igdb_data.summary {
                    println!("Updating description to: {}", summary);
                    game.description = igdb_data.summary.clone();
                    metadata_updated = true;
                }
                
                // Update release date
                if game.release_date.is_none() && igdb_data.first_release_date.is_some() {
                    let timestamp = igdb_data.first_release_date.unwrap();
                    let date = chrono::DateTime::from_timestamp(timestamp, 0)
                        .map(|dt| dt.format("%Y-%m-%d").to_string());
                    game.release_date = date;
                    metadata_updated = true;
                }
                
                // Update cover image URL
                if let Some(cover) = igdb_data.cover {
                    game.cover_url = Some(cover.url.replace("t_thumb", "t_cover_big"));
                    metadata_updated = true;
                }
                
                // Update banner URL (use first artwork as banner, fallback to first screenshot)
                if let Some(artworks) = &igdb_data.artworks {
                    if let Some(first_artwork) = artworks.first() {
                        game.banner_url = Some(first_artwork.url.replace("t_thumb", "t_1080p"));
                        metadata_updated = true;
                        println!("Set banner URL from artwork: {}", first_artwork.url);
                    }
                } else if let Some(screenshots) = &igdb_data.screenshots {
                    if let Some(first_screenshot) = screenshots.first() {
                        game.banner_url = Some(first_screenshot.url.replace("t_thumb", "t_screenshot_huge"));
                        metadata_updated = true;
                        println!("Set banner URL from screenshot (fallback): {}", first_screenshot.url);
                    }
                }
                
                // Update genres
                if let Some(genres) = igdb_data.genres {
                    game.genre = Some(genres.iter().map(|g| g.name.clone()).collect::<Vec<_>>().join(", "));
                    metadata_updated = true;
                }
                
                // Update themes
                if let Some(themes) = igdb_data.themes {
                    game.themes = Some(themes.iter().map(|t| t.name.clone()).collect::<Vec<_>>().join(", "));
                    metadata_updated = true;
                }
                
                // Update screenshots
                 if let Some(screenshots) = &igdb_data.screenshots {
                     println!("Found {} screenshots", screenshots.len());
                     for (i, screenshot) in screenshots.iter().enumerate() {
                         println!("Screenshot {}: {}", i, screenshot.url);
                     }
                     let screenshot_urls: Vec<String> = screenshots.iter()
                         .map(|s| s.url.replace("t_thumb", "t_screenshot_huge"))
                         .collect();
                     let screenshots_json = serde_json::to_string(&screenshot_urls).unwrap_or_default();
                     println!("Screenshots JSON: {}", screenshots_json);
                     game.screenshots = Some(screenshots_json);
                     metadata_updated = true;
                 } else {
                     println!("No screenshots found in IGDB data");
                 }
                
                                 // Update videos
                 if let Some(videos) = &igdb_data.videos {
                     println!("Found {} videos", videos.len());
                     for (i, video) in videos.iter().enumerate() {
                         println!("Video {}: {} - {}", i, video.video_id, video.name.as_deref().unwrap_or("No title"));
                     }
                     let video_data: Vec<serde_json::Value> = videos.iter()
                         .map(|v| {
                             let mut video_obj = serde_json::Map::new();
                             video_obj.insert("id".to_string(), serde_json::Value::String(v.video_id.clone()));
                             video_obj.insert("title".to_string(), serde_json::Value::String(v.name.clone().unwrap_or_else(|| format!("Video {}", v.video_id))));
                             serde_json::Value::Object(video_obj)
                         })
                         .collect();
                     let videos_json = serde_json::to_string(&video_data).unwrap_or_default();
                     println!("Videos JSON: {}", videos_json);
                     game.videos = Some(videos_json);
                     metadata_updated = true;
                 } else {
                     println!("No videos found in IGDB data");
                 }
                
                // Update developers and publishers
                if let Some(companies) = igdb_data.involved_companies {
                    let developers: Vec<_> = companies.iter()
                        .filter(|c| c.developer)
                        .map(|c| c.company.name.clone())
                        .collect();
                    let publishers: Vec<_> = companies.iter()
                        .filter(|c| c.publisher)
                        .map(|c| c.company.name.clone())
                        .collect();
                    
                    if !developers.is_empty() {
                        game.developer = Some(developers.join(", "));
                        metadata_updated = true;
                    }
                    if !publishers.is_empty() {
                        game.publisher = Some(publishers.join(", "));
                        metadata_updated = true;
                    }
                }
            }
            Ok(None) => {
                println!("No IGDB data found for game ID: {}", igdb_id);
            }
            Err(e) => {
                println!("Failed to fetch IGDB data for game {}: {}", igdb_id, e);
            }
        }
    } else {
        println!("No IGDB ID available for game '{}', skipping IGDB fetch", game.title);
    }
    
    // If we're still missing data, try Giant Bomb
    if game.description.is_none() || game.release_date.is_none() {
        println!("Attempting to fetch Giant Bomb data for game: {}", game.title);
        
        match fetch_giant_bomb_data(&client, &game.title).await {
            Ok(Some(gb_data)) => {
                println!("Successfully fetched Giant Bomb data for '{}'", gb_data.name);
                
                if game.description.is_none() {
                    game.description = gb_data.deck;
                    metadata_updated = true;
                }
                if game.release_date.is_none() {
                    game.release_date = gb_data.original_release_date;
                    metadata_updated = true;
                }
            }
            Ok(None) => {
                println!("No Giant Bomb data found for game: {}", game.title);
            }
            Err(e) => {
                println!("Failed to fetch Giant Bomb data for game '{}': {}", game.title, e);
            }
        }
    }

    println!("Metadata fetch complete for game '{}'. Updating database.", game.title);

    // 4. Update the database with the new data.
    sqlx::query(
        "UPDATE games SET \
        title = ?, \
        description = ?, \
        release_date = ?, \
        cover_url = ?, \
        banner_url = ?, \
        genre = ?, \
        themes = ?, \
        screenshots = ?, \
        videos = ?, \
        developer = ?, \
        publisher = ?, \
        time_to_beat = ? \
        WHERE id = ?",
    )
    .bind(&game.title)
    .bind(&game.description)
    .bind(&game.release_date)
    .bind(&game.cover_url)
    .bind(&game.banner_url)
    .bind(&game.genre)
    .bind(&game.themes)
    .bind(&game.screenshots)
    .bind(&game.videos)
    .bind(&game.developer)
    .bind(&game.publisher)
    .bind(game.time_to_beat)
    .bind(game.id)
    .execute(&db)
    .await
    .map_err(|e| Error::Config(format!("Failed to update game {} in database: {}", game_id, e)))?;

    println!("Database updated successfully for game id {}", game_id);

    // 5. Only emit an event to the frontend if metadata was actually updated
    if metadata_updated {
        println!("Emitting metadata_updated event for game id {}", game_id);
        app_handle.emit_all("metadata_updated", game.id).ok();
    } else {
        println!("No metadata was updated, skipping event emission");
    }

    Ok(())
}
