use crate::game::keywar_game_random_number::keywar_game_random_number;
use crate::game::keywar_game_challenging::keywar_game_challenging;

//seleciona o modo de jogo
pub fn game_mode() {
    use std::io;

    println!("Escolha o modo de jogo:");
    println!("1 - Número aleatório");
    println!("2 - Desafiado");
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
            println!("Você escolheu o modo Desafio!");
            keywar_game_challenging();
        }
        "0" => {
            println!("Você escolheu sair do jogo.");
        }
        _ => {
            println!("Opção inválida. Por favor, escolha novamente.");
            game_mode();
        }
    }
}
