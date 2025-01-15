mod game;

use game::game_mode::game_mode;
use game::continue_game::continue_game;

fn main() {
    println!("@#=-KEY WAR-=#@");

    loop {
        game_mode();
        if !continue_game() {
            break;
        }
    }

    println!("Jogo encerrado, até a próxima!");
}
