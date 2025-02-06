mod game;
mod db;
mod schema;
mod models;

use colored::*;
use game::game_mode::game_mode;
use game::continue_game::continue_game;
use game::keywar_game_challenging::keywar_game_challenging;
use game::keywar_game_random_number::keywar_game_random_number;
use game::keywar_multiplayer::keywar_multiplayer;
use db::establish_connection;

fn main() {
    println!("{}", "#####################################".red().bold());
    println!("{}", "##        @#=-SPACE WAR-=#@        ##".red().bold());
    println!("{}", "#####################################".red().bold());

    let mut conn = establish_connection();

    loop {
        let selected_mode = game_mode();
        loop {
            match selected_mode.as_str() {
                "random" => keywar_game_random_number(&mut conn),
                "challenging" => keywar_game_challenging(&mut conn),
                "multiplayer" => keywar_multiplayer(),
                _ => {
                    println!("#ERR: Modo inválido!");
                    break;
                }
            }

            match continue_game() {
                Some(action) if action == "A" => continue,
                Some(action) if action == "C" => break,
                None => {
                    println!("Até breve.");
                    return;
                }
                _ => unreachable!(),
            }
        }
    }
}
