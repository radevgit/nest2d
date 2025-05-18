
use dioxus::prelude::*;

// The database is only available to server code
#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        // Open the database from the persisted "projects.db" file
        let conn = rusqlite::Connection::open("projects.db").expect("Failed to open database");

        // Create the "dogs" table if it doesn't already exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );",
        ).unwrap();

        // Return the connection
        conn
    };
}

#[server(endpoint = "create_project")]
pub async fn create_project(name: String, description: String) -> Result<(), ServerFnError> {
    DB.with(|db| {
        db.execute(
            "INSERT INTO projects (name, description) VALUES (?1, ?2)",
            &[&name, &description],
        )
    })?;
    Ok(())
}

#[server(endpoint = "delete_project")]
pub async fn delete_project(id: usize) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("DELETE FROM projects WHERE id = (?1)", [&id]))?;
    Ok(())
}

#[server(endpoint = "list_projects")]
pub async fn list_projects() -> Result<Vec<(usize, String, String, String)>, ServerFnError> {
    let dogs = DB.with(|f| {
        f.prepare("SELECT id, name, description, created_at FROM [projects] ORDER BY id DESC LIMIT 100")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    Ok(dogs)
}
