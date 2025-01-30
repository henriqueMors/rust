use std::io; //biblioteca para insercao de caracteres pelo teclado

use crate::game::game_mode::game_mode;


pub fn continue_game() -> Option<String> {
    println!("\nO que você deseja fazer?");
    println!("A - Jogar novamente");
    println!("C - Escolher outro modo de jogo");
    println!("E - Encerrar o jogo");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a entrada");

    let input = input.trim().to_uppercase();

    match input.as_str() {
        "A" => Some("A".to_string()),
        "C" => {
            game_mode(); // Redireciona para o menu de modos
            None // Retorna None para voltar ao menu principal
        },
        "E" => {
            println!("Jogo encerrado. Até a próxima!");
            None
        },
        _ => {
            println!("Opção inválida. Tente novamente.");
            continue_game() // Repete o menu até ter uma entrada válida
        }
    }
}
