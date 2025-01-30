use crate::game::keywar_game_random_number::keywar_game_random_number;
use crate::game::keywar_game_challenging::keywar_game_challenging;
use crate::game::keywar_multiplayer::keywar_multiplayer;
use std::io;

pub fn game_mode() -> String {
    loop {
        println!("Escolha o modo de jogo:");
        println!("1 - Número aleatório");
        println!("2 - Desafiado");
        println!("3 - Multiplayer");
        println!("0 - Sair do jogo");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Erro ao ler a entrada");

        match choice.trim() {
            "1" => {
                println!("Você escolheu o modo Número Randômico!");
                keywar_game_random_number();
            }
            "2" => {
                println!("Você escolheu o modo Desafiado!");
                keywar_game_challenging();
            }
            "3" => {
                println!("Você escolheu o modo Multiplayer!");
                keywar_multiplayer();
            }
            "0" => {
                println!("Jogo encerrado. Até a próxima!");
                std::process::exit(0); // sai do programa imediatamente
            }
            _ => {
                println!("Opção inválida. Por favor, escolha novamente.");
            }
        }
    }
}
