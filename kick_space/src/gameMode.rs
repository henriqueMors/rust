use std::io;
mod typeGame;

// selecionar o modo do game
pub fn game_mode() {

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
            typeGame::keywar_game_random_number(); // modo numero randomico
            
        }
        "2" => {
            println!("Você escolheu o modo Desafio!");
            typeGame::keywar_game_challenging(); // modo desafio
        }
        "0" => {
            println!("Você escolheu sair do jogo."); // encerra o jogo
        }
        _ => {
            println!("Opção inválida. Por favor, escolha novamente.");
            game_mode(); // solicita uma entrada válida
        }
    }

}