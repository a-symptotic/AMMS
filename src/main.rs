mod aircraft;
mod db;
mod inspection;
mod maintenance;
mod utils;

use aircraft::*;
use maintenance::*;
use utils::*;

fn print_menu() {
    println!();
    println!("==================================================================");
    println!("        AIRCRAFT MAINTENANCE MANAGEMENT SYSTEM (AMMS)");
    println!("                          Version 1.0");
    println!("==================================================================");
    println!("Aircraft Management");
    println!("------------------------------------------------------------------");
    println!("1. Add Aircraft");
    println!("2. View Aircraft");
    println!("3. Delete Aircraft");
    println!();
    println!("Maintenance");
    println!("------------------------------------------------------------------");
    println!("4. Add Maintenance Log");
    println!("5. View Maintenance History");
    println!();
    println!("Inspection");
    println!("------------------------------------------------------------------");
    println!("6. Inspection Due Calculator");
    println!();
    println!("0. Exit");
    println!("==================================================================");
}
fn add_aircraft_menu() {
    println!();
    println!("================================================");
    println!("              ADD AIRCRAFT");
    println!("================================================");
    let registration = read_input("Registration:");
    let aircraft_type = read_input("Aircraft Type:");
    let hours = read_f64("Current Flight Hours:");

    match add_aircraft(&registration, &aircraft_type, hours) {
        Ok(_) => println!("\n✓ Aircraft added successfully."),
        Err(e) => println!("\nError: {}", e),
    }
}

fn view_aircraft_menu() {
    match get_aircraft() {
        Ok(list) => {
            if list.is_empty() {
                println!("\nNo aircraft found.");
                return;
            }

            println!();
            println!(
                "=============================================================================="
            );
            println!("{:^78}", "AIRCRAFT FLEET");
            println!(
                "=============================================================================="
            );
            println!(
                "{:<5} {:<15} {:<25} {:>10}",
                "ID", "Registration", "Aircraft Type", "Hours"
            );
            println!("=================================================================");

            for a in list {
                println!(
                    "{:<5} {:<15} {:<25} {:>10.1}",
                    a.id, a.registration, a.aircraft_type, a.total_hours
                );
            }
        }

        Err(e) => println!("Database Error: {}", e),
    }
}

fn delete_aircraft_menu() {
    println!();
    println!("================================================");
    println!("            DELETE AIRCRAFT");
    println!("================================================");
    view_aircraft_menu();

    let id = read_i32("\nAircraft ID to delete:");

    match delete_aircraft(id) {
        Ok(_) => println!("\n✓ Aircraft deleted."),
        Err(e) => println!("\nError: {}", e),
    }
}

fn add_maintenance_menu() {
    println!();
    println!("================================================");
    println!("          ADD MAINTENANCE RECORD");
    println!("================================================");

    let aircraft = match get_aircraft() {
        Ok(list) => list,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if aircraft.is_empty() {
        println!("\nNo aircraft available.");
        return;
    }

    println!();
    println!("Available Aircraft");
    println!("-----------------------------------------------------");

    for a in &aircraft {
        println!("{} : {} ({})", a.id, a.registration, a.aircraft_type);
    }

    println!("-----------------------------------------------------");

    let aircraft_id = read_i32("Aircraft ID:");

    let date = read_input("Maintenance Date (YYYY-MM-DD):");

    let engineer = read_input("Engineer Name:");

    let hours = read_f64("Aircraft Hours:");

    let work = read_input("Work Performed:");

    match add_maintenance_log(aircraft_id, &date, &engineer, hours, &work) {
        Ok(_) => println!("\n✓ Maintenance record saved."),
        Err(e) => println!("\nError: {}", e),
    }
}

fn view_maintenance_menu() {
    if let Err(e) = print_logs() {
        println!("Error: {}", e);
    }
}

fn main() {
    if let Err(e) = db::initialize_database() {
        println!("Failed to initialize database: {}", e);
        return;
    }

    if let Err(e) = insert_sample_aircraft() {
        println!("Sample data error: {}", e);
    }

    loop {
        print_menu();

        let choice = read_i32("\nChoose an option:");

        match choice {
            1 => add_aircraft_menu(),

            2 => view_aircraft_menu(),

            3 => delete_aircraft_menu(),

            4 => add_maintenance_menu(),

            5 => view_maintenance_menu(),

            6 => inspection::inspection_calculator(),

            0 => {
                println!();
                println!("==============================================================");
                println!("Thank you for using Aircraft Maintenance Management System.");
                println!("Goodbye!");
                println!("==============================================================");
                break;
            }

            _ => println!("\nInvalid option."),
        }
    }
}
