use std::io;

pub fn continue_game() -> bool {
    println!("Encerrar o jogo? (s/n)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a entrada");

    match input.trim().to_lowercase().as_str() {
        "n" => true,
        "s" => false,
        _ => {
            println!("Entrada inválida. Por favor, digite 's' para continuar ou 'n' para encerrar.");
            continue_game()
        }
    }
}
