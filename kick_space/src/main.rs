// main.rs
mod game;
mod db;
mod schema;

use colored::*;
use game::game_mode::game_mode;
use game::continue_game::continue_game;
use game::keywar_game_random_number::keywar_game_random_number;
use game::keywar_game_challenging::keywar_game_challenging;
use game::keywar_multiplayer::keywar_multiplayer;

fn main() {
    println!("{}", "@#=-KEY WAR-=#@".red().bold());

    loop {
        // Escolhe o modo de jogo
        let selected_mode = game_mode();

        // Inicia o jogo baseado no modo escolhido
        match selected_mode.as_str() {
            "random" => keywar_game_random_number(),
            "challenging" => keywar_game_challenging(),
            "multiplayer" => keywar_multiplayer(),
            _ => {
                println!("Modo inválido!");
                break;
            }
        }

        // Pergunta ao jogador o que ele deseja fazer
        match continue_game() {
            Some(action) if action == "A" => continue,  // Reinicia o mesmo modo
            Some(action) if action == "C" => continue, // Volta ao menu para mudar o modo
            None => {
                println!("Jogo encerrado! Até a próxima.");
                return;
            }
            _ => unreachable!(),
        }
    }
}