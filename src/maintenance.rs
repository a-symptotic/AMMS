use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct MaintenanceLog {
    pub registration: String,
    pub maintenance_date: String,
    pub engineer_name: String,
    pub aircraft_hours: f64,
    pub work_done: String,
}

pub fn add_maintenance_log(
    aircraft_id: i32,
    maintenance_date: &str,
    engineer_name: &str,
    aircraft_hours: f64,
    work_done: &str,
) -> Result<()> {
    let conn = Connection::open("amms.db")?;

    conn.execute(
        "
        INSERT INTO maintenance_logs
        (
            aircraft_id,
            maintenance_date,
            engineer_name,
            aircraft_hours,
            work_done
        )
        VALUES
        (?1, ?2, ?3, ?4, ?5)
        ",
        params![
            aircraft_id,
            maintenance_date,
            engineer_name,
            aircraft_hours,
            work_done
        ],
    )?;

    Ok(())
}

pub fn get_maintenance_logs() -> Result<Vec<MaintenanceLog>> {
    let conn = Connection::open("amms.db")?;

    let mut stmt = conn.prepare(
        "
        SELECT
            aircraft.registration,
            maintenance_date,
            engineer_name,
            aircraft_hours,
            work_done
        FROM maintenance_logs
        INNER JOIN aircraft
            ON maintenance_logs.aircraft_id = aircraft.id
        ORDER BY maintenance_logs.id DESC
        ",
    )?;

    let log_iter = stmt.query_map([], |row| {
        Ok(MaintenanceLog {
            registration: row.get(0)?,
            maintenance_date: row.get(1)?,
            engineer_name: row.get(2)?,
            aircraft_hours: row.get(3)?,
            work_done: row.get(4)?,
        })
    })?;

    let mut logs = Vec::new();

    for log in log_iter {
        logs.push(log?);
    }

    Ok(logs)
}

pub fn print_logs() -> Result<()> {
    let logs = get_maintenance_logs()?;

    if logs.is_empty() {
        println!("\nNo maintenance records found.");
        return Ok(());
    }

    println!();
    println!("==============================================================");
    println!("                 MAINTENANCE HISTORY");
    println!("==============================================================");
    for log in logs {
        println!("--------------------------------------------------------------");
        println!("Aircraft        : {}", log.registration);
        println!("Date            : {}", log.maintenance_date);
        println!("Engineer        : {}", log.engineer_name);
        println!("Aircraft Hours  : {:.1}", log.aircraft_hours);
        println!("Work Performed  : {}", log.work_done);
    }

    println!("--------------------------------------------------------------");

    Ok(())
}
