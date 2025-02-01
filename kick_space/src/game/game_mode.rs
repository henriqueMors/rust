use crate::game::keywar_game_random_number::keywar_game_random_number;
use crate::game::keywar_game_challenging::keywar_game_challenging;
use crate::game::keywar_multiplayer::keywar_multiplayer;
use crate::game::best_players_screen::show_best_players;
use std::io;

pub fn game_mode() -> String {
    loop {
        println!("Escolha o modo de jogo:");
        println!("1 - Número aleatório");
        println!("2 - Desafiado");
        println!("3 - Multiplayer");
        println!("4 - Ver melhores jogadores");
        println!("0 - Sair do jogo");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Erro ao ler a entrada");

        match choice.trim() {
            "1" => return "random".to_string(),
            "2" => return "challenging".to_string(),
            "3" => return "multiplayer".to_string(),
            "4" => {
                show_best_players();
                continue;
            }
            "0" => {
                println!("Jogo encerrado. Até a próxima!");
                std::process::exit(0);
            }
            _ => {
                println!("Opção inválida. Por favor, escolha novamente.");
            }
        }
    }
}