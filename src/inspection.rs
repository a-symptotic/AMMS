use crate::utils::read_f64;

/// Runs the inspection due calculator.
pub fn inspection_calculator() {
    println!("\n========== Inspection Due Calculator ==========\n");

    let current_hours = read_f64("Current Aircraft Hours: ");
    let inspection_interval = read_f64("Inspection Interval (e.g. 50 or 100): ");

    if inspection_interval <= 0.0 {
        println!("\nInspection interval must be greater than zero.");
        return;
    }

    let next_due = ((current_hours / inspection_interval).floor() + 1.0) * inspection_interval;

    let remaining = next_due - current_hours;

    println!("\n===============================================");
    println!("Current Hours      : {:.1}", current_hours);
    println!("Inspection Interval: {:.1} Hours", inspection_interval);
    println!("Next Inspection At : {:.1} Hours", next_due);
    println!("Hours Remaining    : {:.1}", remaining);
    println!("===============================================\n");
}
