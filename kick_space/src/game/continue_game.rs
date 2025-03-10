use std::io; //biblioteca para insercao de caracteres pelo teclado

pub fn continue_game() -> Option<String> {
    println!("#####################################");
    println!("Escolha uma opção:");
    println!("A - Jogar novamente");
    println!("C - Menu principal");
    println!("E - Encerrar o jogo");
    
    print!("Digite sua opção: "); //exibe a frase antes da entrada
    io::Write::flush(&mut std::io::stdout()).expect("Falha ao limpar o buffer"); // Garante que o print apareça antes da entrada do usuário

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a entrada");

    let input = input.trim().to_uppercase();

    match input.as_str() {
        "A" => Some("A".to_string()), //reinicia o mesmo modo
        "C" => Some("C".to_string()), //volta ao menu para mudar o modo
        "E" => {
            println!("Jogo encerrado!");
            None //encerra o jogo
        },
        _ => {
            println!("Opção inválida. Tente novamente.");
            continue_game() //repete o menu ate ter uma entrada valida
        }
    }
}