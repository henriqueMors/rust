use std::io;

//seleciona o modo de jogo
pub fn game_mode() -> String {
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
        "1" => "random".to_string(),
        "2" => "challenging".to_string(),
        "3" => "multiplayer".to_string(),
        "0" => "exit".to_string(),
        _ => {
            println!("Opção inválida. Escolha novamente.");
            game_mode() // Solicita uma nova entrada válida
        }
    }
}