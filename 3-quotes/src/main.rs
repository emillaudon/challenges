use std::io;

fn main() {

    println!("What is the quote?");

    let mut input_quote = String::new();

    io::stdin().read_line(&mut input_quote)
        .expect("Failed to read line.");

    let input_quote_trimmed = input_quote.trim();

    println!("Who said it?");

    let mut input_quoted_person = String::new();

    io::stdin().read_line(&mut input_quoted_person)
        .expect("Failed to read line.");

    let input_quoted_trimmed = input_quoted_person.trim();

    println!("{} says, \"{}\"", input_quoted_trimmed, input_quote_trimmed);
    
}
