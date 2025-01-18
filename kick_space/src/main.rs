//main é o arquivo principal, aqui executa-se todo o software
mod game;
use colored::*;

use game::game_mode::game_mode;
use game::continue_game::continue_game;

fn main() {
    println!("@#=-KEY WAR-=#@".green().bold());

    loop {
        game_mode();
        if !continue_game() {
            break;
        }
    }

    println!("Jogo encerrado, até a próxima!");
}
