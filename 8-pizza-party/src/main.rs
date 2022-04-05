use std::io;

fn main() {
    let SLICES_PER_PIZZA: usize = 8;

    let people = get_input("How many people?");
    let pizzas = get_input("How many pizzas?");

    calculate_pizzas(people, pizzas, SLICES_PER_PIZZA);
    
}

fn calculate_pizzas(people: usize, pizzas: usize, slices_pp: usize) {
    let slices = pizzas * slices_pp;

    let slices_per_person = slices / people;

    println!("{} slices per person.", slices_per_person);
    println!("{} slices left.", slices % people);
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