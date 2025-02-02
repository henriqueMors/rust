// main.rs
mod game;
mod db;
mod schema;
mod models;

use colored::*;
use game::game_mode::game_mode;
use game::continue_game::continue_game;
use game::keywar_game_random_number::keywar_game_random_number;
use game::keywar_game_challenging::keywar_game_challenging;
use game::keywar_multiplayer::keywar_multiplayer;

fn main() {
    println!("{}", "##############################".red().bold());
    println!("{}", "       @#=-KEY WAR-=#@".red().bold());
    println!("{}", "##############################".red().bold());

    // Loop principal do jogo
    loop {
        // Escolhe o modo de jogo
        let selected_mode = game_mode();

        // Loop para o modo de jogo atual
        loop {
            // Inicia o jogo baseado no modo escolhido
            match selected_mode.as_str() {
                "random" => keywar_game_random_number(),
                "challenging" => keywar_game_challenging(),
                "multiplayer" => keywar_multiplayer(),
                _ => {
                    println!("#ERR: Modo inválido!");
                    break;
                }
            }

            // Pergunta ao jogador o que ele deseja fazer
            match continue_game() {
                Some(action) if action == "A" => continue,  // Reinicia o mesmo modo
                Some(action) if action == "C" => break,    // Volta ao menu para mudar o modo
                None => {
                    println!("Até breve.");
                    return;
                }
                _ => unreachable!(),
            }
        }
    }
}