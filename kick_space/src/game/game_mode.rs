use crate::game::best_players_screen::show_best_players;
use std::io;

pub fn game_mode() -> String {
    loop {
        println!("#####################################");
        println!("Escolha o modo de jogo:");
        println!("1 - Número aleatório");
        println!("2 - Desafiador");
        println!("3 - Multiplayer");
        println!("4 - Ver melhores jogadores");
        println!("0 - Sair do jogo");

        print!("Digite sua opção: "); // Exibe a frase antes da entrada
        io::Write::flush(&mut std::io::stdout()).expect("Falha ao limpar o buffer"); // Garante que o print apareça antes da entrada do usuário

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