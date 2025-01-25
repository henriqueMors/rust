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

    let connection = establish_connection();
    create_player(&connection, "Alice");
    println!("Jogador Alice criado!");

    loop {
        game_mode();
        if !continue_game() {
            break;
        }
    }

    println!("Jogo encerrado, até a próxima!");
}
