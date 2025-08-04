-- Add migration script here
CREATE TABLE IF NOT EXISTS games (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    igdb_id INTEGER,
    source_path TEXT NOT NULL UNIQUE,
    install_path TEXT,
    status TEXT NOT NULL DEFAULT 'Ready to Install',
    description TEXT,
    cover_url TEXT,
    banner_url TEXT,
    release_date TEXT,
    developer TEXT,
    publisher TEXT,
    genre TEXT,
    themes TEXT,
    game_modes TEXT,
    tags TEXT,
    metacritic_score INTEGER,
    steam_rating_percent INTEGER,
    steam_rating_text TEXT,
    time_to_beat INTEGER,
    install_size INTEGER
);
