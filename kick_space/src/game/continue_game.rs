use std::io; //biblioteca para insercao de caracteres pelo teclado

use crate::game::game_mode::game_mode;


pub fn continue_game() -> Option<String> { //verifica se quer continuar ou sair do jogo
    println!("\nO que você deseja fazer?");
    println!("A - Jogar novamente");
    println!("C - Escolher outro modo de jogo");
    println!("E - Encerrar o jogo");

    let mut input = String::new(); //verifica o caractere inserido para decidir o match (switch)
    io::stdin()
        .read_line(&mut input) //armazenapara verificar a opcao de continuar ou sair do jogo
        .expect("Erro ao ler a entrada");

    let input = input.trim().to_uppercase(); // normaliza entrada para uppercase

    match input.as_str() {
        "A" => Some("A".to_string()), //retorna a escolha como Some(String)
        "C" => {
            game_mode(); //redireciona para o menu de modos
            Some("C".to_string()) //continua com a escolha
        },
        "E" => {
            println!("Jogo encerrado. Até a próxima!");
            None //indica que o jogo deve encerrar
        },
        _ => {
            println!("Opção inválida. Tente novamente.");
            continue_game() //repete o menu ate ter uma entrada valida
        }
    }
}
