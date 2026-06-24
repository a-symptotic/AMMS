use rusqlite::{params, Connection, Result};

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
