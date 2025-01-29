//main é o arquivo principal, aqui executa-se todo o software
mod game;
mod db;
mod schema;

use game::create_player::create_player;
use colored::*;
use db::establish_connection;
use game::game_mode::game_mode;
use game::continue_game::continue_game;

fn main() {
    println!("{}","@#=-KEY WAR-=#@".red().bold());

    let connection = establish_connection();
    println!("Conexão com o banco de dados estabelecida!");

    let mut connection = establish_connection();
    create_player (&mut connection, "Alice");
    println!("Jogador Alice criado!");

    let selected_mode = game_mode(); // Permite escolher um modo de jogo

    loop {
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
            Some(action) if action == "again" => continue,  // Reinicia o mesmo modo
            Some(action) if action == "change" => break,    // Volta ao menu para mudar o modo
            None => {
                println!("Jogo encerrado! Até a próxima.");
                return;
            }
            _ => unreachable!(),
        }
    }
}

