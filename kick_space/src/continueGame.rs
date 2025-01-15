use std::io;

// verificar se o jogador deseja continuar
pub fn continue_game() -> bool {
    println!("Encerrar o jogo? (s/n)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a entrada");

    let input = input.trim().to_lowercase();

    match input.as_str() {
        "n" => true,  // true se o gamer quiser continuar
        "s" => false, // false se o gamer quiser encerrar
        _ => {
            println!("Entrada inválida. Por favor, digite 's' para continuar ou 'n' para encerrar.");
            continue_game() // funcao para pedir uma entrada valida
        }
    }
}