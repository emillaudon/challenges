use std::io;

fn main() {
    let input_noun = get_input("Noun");

    let input_verb = get_input("A verb: ");

    let input_adjective = get_input("An adjective: ");

    let input_adverb = get_input("Adverb: ");

    println!("Do you {} your {} {} {}? That's hilarious!", input_verb, input_adjective, input_noun, input_adverb);
}

fn get_input(message: &str) -> String {
    let mut input = String::new();

    println!("{}", message);

    io::stdin().read_line(&mut input)
        .expect("Failed to read line.");

    let input_trimmed = input.trim().to_string();

    input_trimmed
}