use std::io;

fn main() {
    let number_one = get_number("first");
    let number_two = get_number("second");

    print_calculations(number_one, number_two);
}

fn get_number(message: &str) -> usize{
    let mut input = String::new();

    println!("Enter {} number: ", message);

    io::stdin().read_line(&mut input)
        .expect("Failed to read line.");

    match input.trim().parse::<usize>() {
        Ok(number) => return number,
        Err(e) => panic!("Enter a number!"),
    }
}

fn print_calculations(one: usize, two: usize) {
    println!("{} + {} = {} \n{} - {} = {}\n{} * {} = {}\n{} / {} = {}\n", one, two, one + two, one, two, one - two, one, two, one * two, one, two, one / two);
}