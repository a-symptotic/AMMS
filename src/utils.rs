use std::io;

/// Read a line of text from the user.
pub fn read_input(prompt: &str) -> String {
    let mut input = String::new();

    println!("{}", prompt);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

/// Read a floating-point number.
pub fn read_f64(prompt: &str) -> f64 {
    loop {
        let input = read_input(prompt);

        match input.parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid number. Try again."),
        }
    }
}

/// Read an integer.
pub fn read_i32(prompt: &str) -> i32 {
    loop {
        let input = read_input(prompt);

        match input.parse::<i32>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid number. Try again."),
        }
    }
}
