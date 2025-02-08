use crate::game::best_players_screen::show_best_players;
use crate::db::{establish_connection, reset_best_players}; //adicionada a importacao
use std::io;

pub fn game_mode() -> String {
    loop {
        println!("#####################################");
        println!("Escolha o modo de jogo:");
        println!("1 - Número aleatório");
        println!("2 - Desafiador");
        println!("3 - Multiplayer");
        println!("4 - Ver melhores jogadores");
        println!("5 - Resetar melhores jogadores");
        println!("0 - Sair do jogo");

        print!("Digite sua opção: ");
        io::Write::flush(&mut std::io::stdout()).expect("Falha ao limpar o buffer");

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
            "5" => {
                let mut conn = establish_connection();
                if let Err(e) = reset_best_players(&mut conn) {
                    eprintln!("Erro ao resetar a tabela: {}", e);
                } else {
                    println!("✅ Tabela de melhores jogadores resetada com sucesso!");
                }
                continue; //continua o loop do menu
            }
            "0" => {
                println!("Saindo do jogo, até a próxima!");
                println!("#####################################");
                std::process::exit(0);
            }
            _ => {
                println!("Opção inválida. Por favor, escolha novamente.");
            }
        }
    }
}