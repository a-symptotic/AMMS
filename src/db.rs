use rusqlite::{Connection, Result};

pub fn initialize_database() -> Result<()> {
    let conn = Connection::open("amms.db")?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS aircraft (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            registration TEXT NOT NULL UNIQUE,
            aircraft_type TEXT NOT NULL,
            total_hours REAL NOT NULL
        )
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS maintenance_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            aircraft_id INTEGER NOT NULL,
            maintenance_date TEXT NOT NULL,
            engineer_name TEXT NOT NULL,
            aircraft_hours REAL NOT NULL,
            work_done TEXT NOT NULL,
            FOREIGN KEY (aircraft_id)
                REFERENCES aircraft(id)
                ON DELETE CASCADE
        )
        ",
        [],
    )?;

    Ok(())
}