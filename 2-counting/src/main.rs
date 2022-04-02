use std::io;

fn main() {
    loop {
        println!("Input to count: ");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line.");

        let count = input.trim().chars().count();

        if count < 1 {
            println!("Please enter a word.");
        } else {
            println!("The string; {}, contains {} characters", input.trim(), count);
        }
    }
}
