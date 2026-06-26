use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Aircraft {
    pub id: i32,
    pub registration: String,
    pub aircraft_type: String,
    pub total_hours: f64,
}

pub fn add_aircraft(registration: &str, aircraft_type: &str, total_hours: f64) -> Result<()> {
    let conn = Connection::open("amms.db")?;

    conn.execute(
        "
        INSERT INTO aircraft
        (registration, aircraft_type, total_hours)
        VALUES (?1, ?2, ?3)
        ",
        params![registration, aircraft_type, total_hours],
    )?;

    Ok(())
}

pub fn get_aircraft() -> Result<Vec<Aircraft>> {
    let conn = Connection::open("amms.db")?;

    let mut stmt = conn.prepare(
        "
        SELECT
            id,
            registration,
            aircraft_type,
            total_hours
        FROM aircraft
        ORDER BY registration
        ",
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

pub fn delete_aircraft(id: i32) -> Result<()> {
    let conn = Connection::open("amms.db")?;

    conn.execute("DELETE FROM aircraft WHERE id = ?1", params![id])?;

    Ok(())
}

pub fn insert_sample_aircraft() -> Result<()> {
    let conn = Connection::open("amms.db")?;

    let count: i32 = conn.query_row("SELECT COUNT(*) FROM aircraft", [], |row| row.get(0))?;

    if count == 0 {
        conn.execute(
            "INSERT INTO aircraft (registration, aircraft_type, total_hours)
             VALUES ('VT-R66', 'Robinson R66', 845.2)",
            [],
        )?;

        conn.execute(
            "INSERT INTO aircraft (registration, aircraft_type, total_hours)
             VALUES ('VT-C295', 'Airbus C295', 3210.5)",
            [],
        )?;

        conn.execute(
            "INSERT INTO aircraft (registration, aircraft_type, total_hours)
             VALUES ('VT-C172', 'Cessna 172', 1250.0)",
            [],
        )?;
    }

    Ok(())
}
