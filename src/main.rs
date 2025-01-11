use std::fmt::Error;

use guess_the_number::Game;

fn main() -> Result<(), Error> {
    let mut game = Game::new();

    while game.running() {
        game.update();
        game.draw();
    }

    Ok(())
}
