use crate::{models::Game, Result};
use reqwest::Client;
use serde::Deserialize;
use sqlx::{Pool, Sqlite};
use tauri::{AppHandle, Manager};

// --- Mock API Response Structs ---
// In a real app, these would be more complex and match the actual API responses.

#[derive(Deserialize)]
struct IgdbGameData {
    name: String,
    summary: Option<String>,
    first_release_date: Option<i64>,
}

#[derive(Deserialize)]
struct SteamGameData {
    metacritic: Option<MetacriticScore>,
}

#[derive(Deserialize)]
struct MetacriticScore {
    score: i32,
}

// --- Mock API Fetch Functions ---
// These simulate making a network request and returning data.

async fn fetch_igdb_data(_client: &Client, igdb_id: i64) -> Option<IgdbGameData> {
    println!("SIMULATING: Fetching from IGDB for id {}", igdb_id);
    // In a real app, you would build a real request:
    // client.post("https://api.igdb.com/v4/games")
    //     .header("Client-ID", "...")
    //     .header("Authorization", "Bearer ...")
    //     .body(format!("fields name,summary,first_release_date; where id = {};", igdb_id))
    //     .send().await.ok()?.json().await.ok()
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    // Let's pretend IGDB has a summary but not a release date for this game.
    Some(IgdbGameData {
        name: "Mock Game from IGDB".to_string(),
        summary: Some("This is a detailed game summary from IGDB.".to_string()),
        first_release_date: None, // We pretend this is missing to test the waterfall
    })
}

async fn fetch_steam_data(_client: &Client, app_id: &str) -> Option<SteamGameData> {
    println!("SIMULATING: Fetching from Steam for appid {}", app_id);
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    Some(SteamGameData {
        metacritic: Some(MetacriticScore { score: 95 }),
    })
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
        .await?;

    // 2. Create a single HTTP client to be reused for all requests.
    let client = Client::new();

    // 3. --- The Waterfall ---
    // For each field, we try providers in order until we get data.
    if let Some(igdb_id) = game.igdb_id {
        if let Some(igdb_data) = fetch_igdb_data(&client, igdb_id).await {
            if game.description.is_none() {
                game.description = igdb_data.summary;
            }
            if game.release_date.is_none() && igdb_data.first_release_date.is_some() {
                 game.release_date = Some("1999-12-31".to_string()); // Placeholder conversion
            }
        }
    }

    // Pretend we have a steam_id field to use for fetching from steam
    if game.metacritic_score.is_none() {
       if let Some(steam_data) = fetch_steam_data(&client, "730" /* placeholder app id */).await {
            game.metacritic_score = steam_data.metacritic.map(|m| m.score);
        }
    }

    println!("Metadata fetch simulation complete for game '{}'. Updating database.", game.title);

    // 4. Update the database with the new data.
    sqlx::query(
        "UPDATE games SET \
        description = ?, \
        release_date = ?, \
        metacritic_score = ? \
        WHERE id = ?",
    )
    .bind(&game.description)
    .bind(&game.release_date)
    .bind(game.metacritic_score)
    .bind(game.id)
    .execute(&db)
    .await?;

    println!("Database updated successfully for game id {}", game_id);

    // 5. Emit an event to the frontend to notify it of the update.
    app_handle.emit_all("metadata_updated", game.id).ok();

    Ok(())
}
