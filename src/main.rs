
mod commands;

mod game;
mod week1;
mod week2;

use commands::*;

use game::run_game;
use week1::week1_category;
use week2::week2_category;

fn main() {
    let category_handler = CommandHandler::new("category", [
        Command::new("game", run_game),
        Command::new("week 1", week1_category),
        Command::new("week 2", week2_category),
    ].into_iter());

    category_handler.handle();
}
