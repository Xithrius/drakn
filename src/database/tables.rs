use color_eyre::Result;
use rusqlite::Connection;

use super::connection::Database;

const TRACKS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS tracks (
    id TEXT PRIMARY KEY,
    path TEXT NOT NULL,
    hash TEXT UNIQUE,
    duration_secs REAL NOT NULL,
    valid BOOLEAN NOT NULL,

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
";

const PLAYLISTS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS playlists (
    id TEXT PRIMARY KEY,
    parent_id TEXT,

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (parent_id) REFERENCES playlists(id)
);
";

impl Database {
    pub(crate) fn create_tables(conn: &mut Connection) -> Result<()> {
        conn.execute_batch(&format!(
            "
            BEGIN;
            {}
            {}
            COMMIT;
            ",
            TRACKS_TABLE, PLAYLISTS_TABLE
        ))?;

        Ok(())
    }
}
