use std::io;
use chrono::Datelike;

fn main() {
    let age = get_input("What is your current age?");
    let retirement_age = get_input("At what age would you like to retire?");

    if retirement_age < age {
        println!("You can already retire, congratulations!");
    } else {
        let years_left = years_to_retirement(age, retirement_age);

        let current_date = chrono::Utc::now();
        let year = current_date.year() as usize;
        let retirement_year = year + years_left;

        println!("You have {} years left until you can retire.\n It's {}, so you can retire in {}.", years_left, year, retirement_year);
    }
}

fn get_input(message: &str) -> usize {
    let mut input = String::new();

    println!("{}", message);

    io::stdin().read_line(&mut input)
        .expect("Could not read line");

    match input.trim().parse::<usize>() {
        Ok(number) => return number,
        Err(_e) => panic!("Enter a number!"),
    }
}

fn years_to_retirement(age: usize, retirement_age: usize) -> usize {
    let years_left = retirement_age - age;

    years_left
}