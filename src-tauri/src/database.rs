use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::path::PathBuf;

/// Sets up the SQLite database connection and runs migrations.
pub async fn init(db_path: &PathBuf) -> Result<SqlitePool, sqlx::Error> {
    // Create the database file and containing directories if they don't exist.
    if !db_path.exists() {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Sqlite::create_database(db_path.to_str().unwrap()).await?;
    }

    let db_url = db_path.to_str().unwrap();
    let pool = SqlitePool::connect(db_url).await?;

    // Run VACUUM on every startup to keep the database file small and optimized.
    sqlx::query("VACUUM;").execute(&pool).await?;

    // Run migrations to ensure the schema is up to date.
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
