use std::io::stdin;

fn main() {
    let number = "5";

    println!("Guess the number: ");

    loop {
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "quit" => return,
            guess if guess == number => return,
            _ => {
                println!("Guess again!");
            }
        }
    }
}
