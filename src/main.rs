use std::{
    io::stdin,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};

fn seconds_since_epoch() -> Result<u64, SystemTimeError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
}

fn rand() -> u32 {
    match seconds_since_epoch() {
        Ok(seconds) => (seconds as u32 % 8) + 1,
        Err(_) => 1,
    }
}

fn main() {
    let number = rand().to_string();

    println!("Guess the number between 1 and 9 (or type `quit` to exit):");

    loop {
        let mut input = String::new();

        if let Err(_) = stdin().read_line(&mut input) {
            println!("Failed to read input. Please try again.");
            continue;
        };

        let guess = input.trim();

        match guess {
            "quit" => {
                println!("Goodbye!");
                return;
            }
            _ if guess == number => {
                println!("You guessed it right! The number was: {}", number);
                return;
            }
            _ => {
                println!("Incorrect guess. Try again!");
            }
        }
    }
}
