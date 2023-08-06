#![recursion_limit = "1000"]

mod commands;

mod game;
mod week1;
mod week2;
mod week3;
mod week4;

mod tests;

use commands::*;

use game::run_game;
use week1::week1_category;
use week2::week2_category;
use week3::week3_category;
use week4::week4_category;



fn main() {
    let category_handler = CommandHandler::new("category", [
        Command::new("game", run_game),
        Command::new("week 1", week1_category),
        Command::new("week 2", week2_category),
        Command::new("week 3", week3_category),
        Command::new("week_4", week4_category),
    ].into_iter());

    category_handler.handle();
}
