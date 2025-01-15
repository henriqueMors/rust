use std::io; //biblioteca para insercao de caracteres pelo teclado

pub fn continue_game() -> bool { //verifica se quer continuar ou sair do jogo
    println!("Encerrar o jogo? (s/n)");

    let mut input = String::new(); //verifica o caractere inserido para decidir o match (switch)
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a entrada");

    match input.trim().to_lowercase().as_str() {
        "n" => true,
        "s" => false,
        _ => {
            println!("Entrada inválida. Por favor, digite 's' para continuar ou 'n' para encerrar.");
            continue_game() //volta a mensagem em caso de caractere incorreto para verificacao
        }
    }
}