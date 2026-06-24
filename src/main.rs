mod db;
mod aircraft;

use std::io;

fn main() {

    db::initialize_database().unwrap();

    loop {

        println!("1. Add Aircraft");
        println!("2. View Aircraft");
        println!("3. Exit");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .unwrap();

        match choice.trim() {

            "1" => {

                let mut registration = String::new();
                let mut aircraft_type = String::new();
                let mut hours = String::new();

                println!("Registration:");
                io::stdin()
                    .read_line(&mut registration)
                    .unwrap();

                println!("Aircraft Type:");
                io::stdin()
                    .read_line(&mut aircraft_type)
                    .unwrap();

                println!("Total Hours:");
                io::stdin()
                    .read_line(&mut hours)
                    .unwrap();

                let total_hours: f64 =
                    hours.trim().parse().unwrap_or(0.0);

                db::add_aircraft(
                    registration.trim(),
                    aircraft_type.trim(),
                    total_hours,
                )
                .unwrap();

                println!("Aircraft added.");
            }
            "2" => {

            let aircraft = db::get_aircraft().unwrap();

            println!("\n--- Aircraft Fleet ---");

            for a in aircraft {

                println!(
                    "{} | {} | {} hrs",
                    a.registration,
                    a.aircraft_type,
                    a.total_hours
                );
            }
        }

            "3" => break,

            _ => println!("Invalid choice"),
        }
    }
}
