use rusqlite::{params, Connection, Result};
use crate::aircraft::Aircraft;
pub fn initialize_database() -> Result<()> {
    let conn = Connection::open("amms.db")?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS aircraft (
            id INTEGER PRIMARY KEY,
            registration TEXT NOT NULL,
            aircraft_type TEXT NOT NULL,
            total_hours REAL NOT NULL
        )
        ",
        [],
    )?;
    conn.execute(
    "
    CREATE TABLE IF NOT EXISTS maintenance_logs (
        id INTEGER PRIMARY KEY,
        aircraft_id INTEGER NOT NULL,
        maintenance_date TEXT NOT NULL,
        engineer_name TEXT NOT NULL,
        work_done TEXT NOT NULL,
        FOREIGN KEY (aircraft_id) REFERENCES aircraft(id)
    )
    ",
    [],
)?;
    conn.execute(
    "
    CREATE TABLE IF NOT EXISTS maintenance_logs (
        id INTEGER PRIMARY KEY,
        aircraft_id INTEGER NOT NULL,
        maintenance_date TEXT NOT NULL,
        engineer_name TEXT NOT NULL,
        aircraft_hours REAL NOT NULL,
        work_done TEXT NOT NULL,
        FOREIGN KEY (aircraft_id) REFERENCES aircraft(id)
    )
    ",
    [],
)?;
    Ok(())
    
}
pub fn add_aircraft(
    registration: &str,
    aircraft_type: &str,
    total_hours: f64,
) -> Result<()> {

    let conn = Connection::open("amms.db")?;

    conn.execute(
        "INSERT INTO aircraft
        (registration, aircraft_type, total_hours)
        VALUES (?1, ?2, ?3)",
        params![registration, aircraft_type, total_hours],
    )?;

    Ok(())
}
pub fn get_aircraft() -> Result<Vec<Aircraft>> {

    let conn = Connection::open("amms.db")?;

    let mut stmt = conn.prepare(
        "SELECT id, registration, aircraft_type, total_hours
         FROM aircraft"
    )?;

    let aircraft_iter = stmt.query_map([], |row| {
        Ok(Aircraft {
            id: row.get(0)?,
            registration: row.get(1)?,
            aircraft_type: row.get(2)?,
            total_hours: row.get(3)?,
        })
    })?;

    let mut aircraft_list = Vec::new();

    for aircraft in aircraft_iter {
        aircraft_list.push(aircraft?);
    }

    Ok(aircraft_list)
}
