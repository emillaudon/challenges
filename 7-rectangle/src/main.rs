use std::io;

fn main() {
    const CONVERSION_FACTOR: f64 = 0.09290304;

    let length = get_input("What is the length of the room in feet?");
    let width = get_input("What is the width of the room in feet?");
    
    calculate_dimensions(length, width, CONVERSION_FACTOR);
}

fn calculate_dimensions(length: f64, width: f64, conversion_factor: f64) {
    let square_foot = length * width;
    let square_meters = square_foot * conversion_factor;

    println!("You entered the dimensions of {} feet by {} feet\n The area is {} square feet\n {} square meters", length, width, square_foot, square_meters);
}

fn get_input(message: &str) -> f64 {
    let mut input = String::new();

    println!("{}", message);

    io::stdin().read_line(&mut input)
        .expect("Could not read line");

    match input.trim().parse::<f64>() {
        Ok(number) => return number,
        Err(_e) => panic!("Enter a number!"),
    }
}