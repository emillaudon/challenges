use std::io;

fn main() {
    let square_feet = get_input("How many feet are you covering?");

    calculate_paint(square_feet);
    
}

fn calculate_paint(square_feet: usize) {
    //180
    let mut whole_gallons = square_feet / 180;
    let leftover = square_feet % 180;
    
    if leftover > 0 {
        whole_gallons += 1;
    }
    
    println!("You need to buy {} gallons of paint", whole_gallons);
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
