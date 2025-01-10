use std::fmt::Error;

use guess_the_number::{Game, State};

fn main() -> Result<(), Error> {
    let mut game = Game::new();

    println!("Play!");

    while game.state != Some(State::Quit) {
        game.update();
        game.draw();
    }

    Ok({})
}
