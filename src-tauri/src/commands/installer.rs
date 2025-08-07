use crate::{config, models::Game, state::AppState, Result};
use std::{fs, path::{Path, PathBuf}};
use tauri::{api::shell, AppHandle, Manager};

#[derive(Clone, serde::Serialize)]
struct InstallProgress {
    id: i64,
    progress: u8,
    status: String,
}

#[tauri::command]
pub async fn install_game(
    app_handle: AppHandle,
    id: i64,
    state: tauri::State<'_, AppState>,
) -> Result<()> {
    let db = state.db.clone();
    let handle = app_handle.clone();

    // Run installation in a blocking thread since unrar is not Send
    let result = tokio::task::spawn_blocking(move || {
        // Block on the async runtime to run our async functions
        let rt = tokio::runtime::Handle::current();
        rt.block_on(install_game_task(handle, id, db))
    })
    .await
    .map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    result
}

async fn install_game_task(app_handle: AppHandle, id: i64, db: sqlx::SqlitePool) -> Result<()> {
    let game = sqlx::query_as::<_, Game>("SELECT * FROM games WHERE id = ?")
        .bind(id)
        .fetch_one(&db)
        .await?;

    let config = config::get_config(app_handle.clone()).await?;
    let install_directory = config.install_path.ok_or_else(|| {
        crate::Error::Config("Installation directory is not set.".to_string())
    })?;

    let source_path = Path::new(&game.source_path);
    let dest_path = Path::new(&install_directory);
    let temp_rar_path = dest_path.join(source_path.file_name().unwrap());

    app_handle
        .emit_all(
            "install_progress",
            InstallProgress {
                id,
                progress: 10,
                status: "Copying archive...".to_string(),
            },
        )
        .ok();

    fs::copy(source_path, &temp_rar_path)?;

    app_handle
        .emit_all(
            "install_progress",
            InstallProgress {
                id,
                progress: 50,
                status: "Extracting...".to_string(),
            },
        )
        .ok();

    let extraction_dest = dest_path.join(&game.title);
    fs::create_dir_all(&extraction_dest)?;
    
    // Correct unrar process: open -> read headers -> extract files
    let archive = unrar::Archive::new(&temp_rar_path);
    let mut opened_archive = archive.open_for_processing()
        .map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
    
    // Extract all files by iterating through them
    loop {
        match opened_archive.read_header() {
            Ok(Some(archive_with_header)) => {
                // Extract this file to the destination
                opened_archive = archive_with_header.extract_to(&extraction_dest)
                    .map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
            }
            Ok(None) => {
                // No more files in archive
                break;
            }
            Err(e) => {
                return Err(crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())));
            }
        }
    }


    app_handle
        .emit_all(
            "install_progress",
            InstallProgress {
                id,
                progress: 90,
                status: "Cleaning up...".to_string(),
            },
        )
        .ok();

    fs::remove_file(&temp_rar_path)?;

    let final_install_path = extraction_dest.to_str().unwrap().to_string();
    sqlx::query("UPDATE games SET status = 'Installed', install_path = ? WHERE id = ?")
        .bind(&final_install_path)
        .bind(id)
        .execute(&db)
        .await?;
    
    app_handle
        .emit_all(
            "install_progress",
            InstallProgress {
                id,
                progress: 100,
                status: "Installed".to_string(),
            },
        )
        .ok();
    
    println!("Game {} installed successfully at {}", game.title, final_install_path);

    Ok(())
}

/// A simple heuristic to find the most likely executable in a directory.
/// It finds all .exe files and returns the largest one.
fn find_executable_in_dir(dir: &Path) -> Result<PathBuf> {
    let mut largest_exe: Option<PathBuf> = None;
    let mut max_size = 0;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("exe") {
            let metadata = entry.metadata()?;
            if metadata.len() > max_size {
                max_size = metadata.len();
                largest_exe = Some(path);
            }
        }
    }

    largest_exe.ok_or_else(|| crate::Error::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound, "No executable found in installation directory"
    )))
}


#[tauri::command]
pub async fn launch_game(
    app_handle: AppHandle,
    id: i64,
    state: tauri::State<'_, AppState>,
) -> Result<()> {
    // 1. Get game from DB
    let game = sqlx::query_as::<_, Game>("SELECT * FROM games WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await?;
    
    // 2. Ensure the install path exists
    let install_path = game.install_path.ok_or_else(|| crate::Error::Io(
        std::io::Error::new(std::io::ErrorKind::NotFound, "Game is not installed.")
    ))?;
    
    // 3. Find the executable
    let executable_path = find_executable_in_dir(Path::new(&install_path))?;
    
    // 4. Launch the executable using Tauri's shell API
    // We pass `None` for the working directory to let the OS decide, which is usually correct.
    shell::open(&app_handle.shell_scope(), executable_path.to_str().unwrap(), None)?;

    Ok(())
}
