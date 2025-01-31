use std::io; //biblioteca para insercao de caracteres pelo teclado

pub fn continue_game() -> Option<String> {
    println!("\n#####################################");
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
        "A" => Some("A".to_string()), // reinicia o mesmo modo
        "C" => Some("C".to_string()), // volta ao menu para mudar o modo
        "E" => {
            println!("Jogo encerrado!");
            None // Encerra o jogo
        },
        _ => {
            println!("Opção inválida. Tente novamente.");
            continue_game() // Repete o menu até ter uma entrada válida
        }
    }
}