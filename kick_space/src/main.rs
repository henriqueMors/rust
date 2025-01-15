use std::io;
use rand::Rng;

mod continue_game; //importando uma fn externa / lembrando de implementar o pub e o :: no metodo
mod mode_game;


fn main() {

    println!("@#=-KEY WAR-=#@");

    loop {
        game_mode(); // aciona o jogo principal
        if !continue_game::continue_game() { // verifica se o jogador quer continuar
            break;
        }
    }
    
    println!("Jogo encerrado, até a próxima!");
}

// gerador automatico
fn random_number() -> usize { // -> usado para que ele seja um return0 com um valor i32
    let mut rng = rand::thread_rng(); // gerador de numeros aleatorios
    rng.gen_range(1..=100) // retorna um numero entre 1 e 100
}



// verifica o resultado e exibe uma mensagem
fn check_result(space_count: usize, challenging: usize) {
    if space_count == challenging {
        
        let messages = vec![ // vetor com mensagens de comemoracao (nao consegui ver nenhuma em tela kkkkkkk)
            "VOCÊ ACERTOU!!!",
            "NA MOSCA!!!",
            "PARABÉNS, DESAFIO COMPLETO!",
            "IIINCRÍÍÍVEEEL, VOCÊ CONSEGUIU!"
        ];

        // indice aleatorio para escolher a mensagem
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..messages.len()); // escolhe um índice válido para o vetor

        // exibe uma mensagem aleatoria
        println!("{}", messages[random_index]);
    } else {
        println!("Errooooooooooouu, a quantidade era de {}!", challenging);
    }
}





// selecionar o modo do game
fn game_mode() {

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
            mode_game::keywar_game_random_number(); // modo numero randomico
            
        }
        "2" => {
            println!("Você escolheu o modo Desafio!");
            mode_game::keywar_game_challenging(); // modo desafio
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