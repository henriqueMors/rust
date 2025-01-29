use std::io; //biblioteca para insercao de caracteres pelo teclado

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

    match input.as_str() { //match funciona como switch em java, js, py
        "A" => Some("again".to_string()),   //jogar o mesmo modo
        "C" => Some("change".to_string()),  //mudar o modo de jogo
        "E" => None,                        //sair do jogo
        _ => {
            println!("Entrada inválida! Digite 'A', 'C' ou 'E'.");
            continue_game() //volta a mensagem em caso de caractere incorreto para verificacao
        }
    }
}
