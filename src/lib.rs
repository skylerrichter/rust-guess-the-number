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

#[derive(PartialEq)]
pub enum State {
    Quit,
    Win,
    Loose,
}

pub struct Game {
    state: Option<State>,
    number: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: None,
            number: rand(),
        }
    }

    pub fn running(&mut self) -> bool {
        self.state != Some(State::Quit)
    }

    pub fn update(&mut self) {
        let mut input = String::new();

        if let Err(_) = stdin().read_line(&mut input) {
            println!("Failed to read input. Please try again.");
        };

        match input.trim() {
            "quit" => self.state = Some(State::Quit),
            text if text == self.number.to_string() => self.state = Some(State::Win),
            _ => self.state = Some(State::Loose),
        }
    }

    pub fn draw(&mut self) {
        match self.state {
            Some(State::Quit) => println!("Quit!"),
            Some(State::Win) => println!("Win!"),
            Some(State::Loose) => println!("Loose!"),
            _ => println!("Whoops!"),
        }
    }
}
