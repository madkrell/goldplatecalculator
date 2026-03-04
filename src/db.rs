use leptos::prelude::*;
use crate::models::shape::SavedObject;

/// Server function: load all saved object presets from the database.
#[server]
pub async fn get_saved_objects() -> Result<Vec<SavedObject>, ServerFnError> {
    let conn = open_db()?;
    let mut stmt = conn
        .prepare("SELECT name, shape_json FROM saved_objects ORDER BY id ASC")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let objects = stmt
        .query_map([], |row| {
            let name: String = row.get(0)?;
            let shape_json: String = row.get(1)?;
            Ok((name, shape_json))
        })
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .filter_map(|r| r.ok())
        .filter_map(|(name, shape_json)| {
            let shape = serde_json::from_str(&shape_json).ok()?;
            Some(SavedObject { name, shape })
        })
        .collect();

    Ok(objects)
}

/// Server function: save a new object preset to the database.
#[server]
pub async fn save_object_preset(name: String, shape_json: String) -> Result<(), ServerFnError> {
    use rusqlite::params;

    let conn = open_db()?;

    // Check for duplicate (same name and shape)
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM saved_objects WHERE name = ?1 AND shape_json = ?2",
            params![name, shape_json],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if !exists {
        conn.execute(
            "INSERT INTO saved_objects (name, shape_json) VALUES (?1, ?2)",
            params![name, shape_json],
        )
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    }

    Ok(())
}

/// Server function: delete a saved object preset from the database.
#[server]
pub async fn delete_saved_object(name: String, shape_json: String) -> Result<(), ServerFnError> {
    use rusqlite::params;

    let conn = open_db()?;
    conn.execute(
        "DELETE FROM saved_objects WHERE name = ?1 AND shape_json = ?2",
        params![name, shape_json],
    )
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

/// Open (or create) the SQLite database and ensure the table exists.
#[cfg(feature = "ssr")]
fn open_db() -> Result<rusqlite::Connection, ServerFnError> {
    use std::fs;

    // Ensure the data directory exists
    fs::create_dir_all("data").map_err(|e| ServerFnError::new(e.to_string()))?;

    let conn = rusqlite::Connection::open("data/saved_objects.db")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS saved_objects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            shape_json TEXT NOT NULL
        );"
    )
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(conn)
}
