use crate::{models::Game, state::AppState, Result};

#[tauri::command]
pub async fn get_games(state: tauri::State<'_, AppState>) -> Result<Vec<Game>> {
    sqlx::query_as::<_, Game>("SELECT * FROM games ORDER BY title")
        .fetch_all(&state.db)
        .await
        .map_err(Into::into)
}

/// A helper to decide if we need to trigger a background fetch.
fn needs_metadata_fetch(game: &Game) -> bool {
    // In a real app, this could be much more comprehensive.
    game.description.is_none() || game.release_date.is_none() || game.metacritic_score.is_none()
}

#[tauri::command]
pub async fn get_game_details(
    app_handle: tauri::AppHandle,
    id: i64,
    state: tauri::State<'_, AppState>,
) -> Result<Game> {
    let game = sqlx::query_as::<_, Game>("SELECT * FROM games WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await?;

    // If the game is missing key data and has an ID to search with,
    // trigger a non-blocking background task to fetch it.
    if needs_metadata_fetch(&game) && game.igdb_id.is_some() {
        println!(
            "Metadata for game '{}' is incomplete. Spawning background fetch.",
            game.title
        );
        let db_pool = state.db.clone();
        let handle = app_handle.clone();
        tokio::spawn(async move {
            if let Err(e) =
                crate::services::metadata::fetch_and_update_metadata(id, db_pool, handle).await
            {
                eprintln!("Failed to fetch metadata for game {}: {}", id, e);
            }
        });
    }

    Ok(game)
}

#[tauri::command]
pub async fn add_game_manually(
    app_handle: tauri::AppHandle,
    file_path: String,
    igdb_id: i64,
    state: tauri::State<'_, AppState>,
) -> Result<Game> {
    let title = std::path::Path::new(&file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown Title")
        .to_string();

    // Insert the game with the minimal info we have.
    let new_game_id =
        sqlx::query("INSERT INTO games (title, source_path, igdb_id, status) VALUES (?, ?, ?, ?)")
            .bind(&title)
            .bind(&file_path)
            .bind(igdb_id)
            .bind("Ready to Install")
            .execute(&state.db)
            .await?
            .last_insert_rowid();

    // Fetch the basic game data to return to the user immediately.
    let game = sqlx::query_as::<_, Game>("SELECT * FROM games WHERE id = ?")
        .bind(new_game_id)
        .fetch_one(&state.db)
        .await?;

    // Spawn the metadata fetch in the background. The user gets an immediate response
    // and the UI will update later once the fetch is complete.
    println!(
        "Game '{}' added. Spawning background metadata fetch.",
        game.title
    );
    let db_pool = state.db.clone();
    let handle = app_handle.clone();
    tokio::spawn(async move {
        if let Err(e) =
            crate::services::metadata::fetch_and_update_metadata(new_game_id, db_pool, handle)
                .await
        {
            eprintln!(
                "Failed to fetch metadata for game {}: {}",
                new_game_id, e
            );
        }
    });

    Ok(game)
}
