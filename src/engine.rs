
use dioxus::{html::{canvas::height, u}, logger::tracing::info, prelude::*};

// The database is only available to server code
#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        // Open the database from the persisted "projects.db" file
        let conn = rusqlite::Connection::open("projects.db").expect("Failed to open database");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );",
        ).unwrap();

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS sheets (
                id INTEGER PRIMARY KEY,
                project INTEGER NOT NULL,
                name TEXT NOT NULL,
                width REAL,
                height REAL,
                quantity INTEGER DEFAULT 1
            );",
        ).unwrap();

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS sheets (
                id INTEGER PRIMARY KEY,
                project INTEGER NOT NULL,
                name TEXT NOT NULL,
                width REAL,
                height REAL,
                quantity INTEGER DEFAULT 1,
                filename TEXT NOT NULL,
                preview BLOB
            );",
        ).unwrap();

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
    let projects = DB.with(|f| {
        f.prepare("SELECT id, name, description, created_at FROM [projects] ORDER BY id DESC LIMIT 100")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    Ok(projects)
}

#[server(endpoint = "count_projects")]
pub async fn count_projects() -> Result<usize, ServerFnError> {
    let projects = DB.with(|f| {
        f.prepare("SELECT COUNT(*) FROM [projects]")
            .unwrap()
            .query_row([], |row| row.get(0))
            .unwrap()
    });
    Ok(projects)
}

// SHEETS

#[server(endpoint = "create_sheet")]
pub async fn create_sheet(project: usize, name: String, w: f64, h: f64, quantity: usize) -> Result<(), ServerFnError> {
    DB.with(|db| {
        db.execute(
            "INSERT INTO sheets (project, name, width, height, quantity) VALUES (?1, ?2, ?3, ?4, ?5)",
            [project.to_string(), name, w.to_string(), h.to_string(), quantity.to_string()],
        )
    })?;
    Ok(())
}

#[server(endpoint = "update_sheet_quantity")]
pub async fn update_sheet_quantity(id: usize, quantity: usize) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("UPDATE sheets SET quantity = (?2) WHERE id = (?1) LIMIT 1", [&id, &quantity]))?;
    Ok(())
}

#[server(endpoint = "delete_sheet")]
pub async fn delete_sheet(id: usize) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("DELETE FROM sheets WHERE id = (?1)", [&id]))?;
    Ok(())
}

#[server(endpoint = "list_sheets")]
pub async fn list_sheets(project: usize) -> Result<Vec<(usize, String, f64, f64, usize)>, ServerFnError> {
    let project_id = if project == std::usize::MAX {
        0
    } else {
        project
    };
    let sheets = DB.with(|f| {
        f.prepare("SELECT id, name, width, height, quantity FROM [sheets] WHERE project = ?1 ORDER BY id DESC LIMIT 100")
            .unwrap()
            .query_map([project_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    Ok(sheets)
}

#[server(endpoint = "count_sheets")]
pub async fn count_sheets(project: usize) -> Result<usize, ServerFnError> {
    let project_id = if project == std::usize::MAX {
        0
    } else {
        project
    };
    info!("count_sheets: {}", project);
    let count = DB.with(|f| {
        f.prepare("SELECT COUNT(*) FROM [sheets] WHERE project = (?1)")
            .unwrap()
            .query_row([project_id], |row| row.get(0))
            .unwrap()
    });
    Ok(count)
}

// PARTS

#[server(endpoint = "create_part")]
pub async fn create_part(project: usize, name: String, w: f64, h: f64, quantity: usize) -> Result<(), ServerFnError> {
    DB.with(|db| {
        db.execute(
            "INSERT INTO sheets (project, name, width, height, quantity) VALUES (?1, ?2, ?3, ?4, ?5)",
            [project.to_string(), name, w.to_string(), h.to_string(), quantity.to_string()],
        )
    })?;
    Ok(())
}

#[server(endpoint = "update_part_quantity")]
pub async fn update_part_quantity(id: usize, quantity: usize) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("UPDATE sheets SET quantity = (?2) WHERE id = (?1) LIMIT 1", [&id, &quantity]))?;
    Ok(())
}

#[server(endpoint = "delete_part")]
pub async fn delete_part(id: usize) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("DELETE FROM sheets WHERE id = (?1)", [&id]))?;
    Ok(())
}

#[server(endpoint = "list_parts")]
pub async fn list_parts(project: usize) -> Result<Vec<(usize, String, f64, f64, usize)>, ServerFnError> {
    let project_id = if project == std::usize::MAX {
        0
    } else {
        project
    };
    let sheets = DB.with(|f| {
        f.prepare("SELECT id, name, width, height, quantity FROM [sheets] WHERE project = ?1 ORDER BY id DESC LIMIT 100")
            .unwrap()
            .query_map([project_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    Ok(sheets)
}

#[server(endpoint = "count_parts")]
pub async fn count_parts(project: usize) -> Result<usize, ServerFnError> {
    let project_id = if project == std::usize::MAX {
        0
    } else {
        project
    };
    info!("count_sheets: {}", project);
    let count = DB.with(|f| {
        f.prepare("SELECT COUNT(*) FROM [sheets] WHERE project = (?1)")
            .unwrap()
            .query_row([project_id], |row| row.get(0))
            .unwrap()
    });
    Ok(count)
}