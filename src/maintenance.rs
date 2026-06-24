#[derive(Debug)]
pub struct MaintenanceLog {
    pub id: i32,
    pub aircraft_id: i32,
    pub maintenance_date: String,
    pub engineer_name: String,
    pub work_done: String,
}