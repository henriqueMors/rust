use std::io;
use rand::Rng;


fn main() {

    keywar_title();

    loop {
        game_mode(); // aciona o jogo principal
        if !continue_game() { // verifica se o jogador quer continuar
            break;
        }
    }

    println!("Jogo encerrado, até a próxima!");
}


// para gerar o nome do jogo quando for necessario
fn keywar_title() {
    println!("@#=-KEY WAR-=#@");
}

// para imprimir uma instrucao de jogo
fn instruction_01() {
    println!("#Pressione somente a barra de espaço e pressione Enter quando terminar# \nGood LucK!");
}


// gerador automatico
fn random_number() -> usize { // -> usado para que ele seja um return com um valor i32
    let mut rng = rand::thread_rng(); // gerador de numeros aleatorios
    rng.gen_range(1..=100) // retorna um número entre 1 e 100
}


// verificar se o jogador deseja continuar
fn continue_game() -> bool {
    println!("Deseja jogar mais uma vez? (s/n)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a entrada");

    let input = input.trim().to_lowercase();

    match input.as_str() {
        "s" => true,  // true se o gamer quiser continuar
        "n" => false, // false se o gamer quiser encerrar
        _ => {
            println!("Entrada inválida. Por favor, digite 's' para continuar ou 'n' para encerrar.");
            continue_game() // função para pedir uma entrada válida
        }
    }
}


// verifica o resultado e exibe uma mensagem
fn check_result(space_count: usize, challenging: usize) {
    if space_count == challenging {
        
        let messages = vec![ // vetor com mensagens de comemoração
            "VOCÊ ACERTOU!!!",
            "NA MOSCA!",
            "PARABÉNS, DESAFIO COMPLETO!",
            "INCRÍVEL, VOCÊ CONSEGUIU!"
        ];

        // gera um índice aleatório para escolher a mensagem
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..messages.len()); // Gera um índice válido para o vetor

        // exibe uma mensagem aleatória
        println!("{}", messages[random_index]);
    } else {
        println!("Errooooooooooouu, a quantidade era de {}!", challenging);
    }
}


fn keywar_game_random_number() {
    
    println!("Como está sua percepção?");

    let challenging = random_number(); // o desafiante digita quantidade de a ser acertada pelo desafiado
    
        println!("Hora de testar! Seu desafio: {}!", challenging);

    let mut key_space = String::new();
    loop {
        instruction_01();
        key_space.clear(); // limpa a entrada anterior
        io::stdin()
            .read_line(&mut key_space)
            .expect("Falha ao ler a entrada");
        
        if key_space.trim().chars().all(|c| c == ' ') { // verifica se todos os caracteres sao espacos
            break; // se todos forem espacos, sai do loop
        } else {
            println!("Entrada inválida! Digite apenas espaços.");
        }
    }

    let space_count = key_space.chars().filter(|&c| c == ' ').count();

    check_result(space_count, challenging); // verificação entre os resultados obtidos para retornar a msg final

    println!("Você digitou {space_count} caracteres de espaço."); // mensagem de encerramento
}


fn keywar_game_challenging() {
    
    println!("Quantos espaços seu adversário deve acertar?");

    let mut challenging = String::new(); // o desafiante digita quantidade de a ser acertada pelo desafiado
        io::stdin()
            .read_line(&mut challenging)
            .expect("Falha ao ler a entrada");
        
    let challenging: usize = match challenging.trim().parse() { // alterando o tipo da variavel para que seja lido como numero
            Ok(num) => num, // vai tratar o erro de caso o usuario digite algo fora do correto (somente espaco)
            Err(_) => {
                println!("Por favor, insira um número válido!");
                return; // sai do programa #preciso entender melhor o uso do 'return;'#
            }
        };
    
        println!("Você digitou o desafio: {},\nagora, é a vez do próximo acertar essa quantidade, \nsomente segurando a barra de espaço.", challenging);

    let mut key_space = String::new();
    loop {
        instruction_01();
        key_space.clear(); // limpa a entrada anterior
        io::stdin()
            .read_line(&mut key_space)
            .expect("Falha ao ler a entrada");
        
        if key_space.trim().chars().all(|c| c == ' ') { // verifica se todos os caracteres sao espacos
            break; // se todos forem espacos, sai do loop
        } else {
            println!("Entrada inválida! Digite apenas espaços.");
        }
    }

    let space_count = key_space.chars().filter(|&c| c == ' ').count();
    /*
            space_count recebe a quantidade de espacos digitados.
            key_space esta configurado para receber e aceitar 
            o tipo de caracter espaco, inserido no teclado.
            c é uma configuracao de caracter (mais ou menos o que o loop faz)
    */

    check_result(space_count, challenging); // verificação entre os resultados obtidos para retornar a msg final

    println!("Você digitou {space_count} caracteres de espaço."); // mensagem de encerramento
}


// selecionar o modo do game
fn game_mode() {

    println!("Escolha o modo de jogo:");
    println!("1 - Número aleatório");
    println!("2 - Desafiado");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Erro ao ler a entrada");

    match choice.trim() {
        "1" => {
            println!("Você escolheu o Modo Clássico!");
            keywar_game_random_number(); // modo numero randomico
            
        }
        "2" => {
            println!("Você escolheu o Novo Modo de Jogo!");
            keywar_game_challenging(); // modo desafio
        }
        _ => {
            println!("Opção inválida. Por favor, escolha novamente.");
            game_mode(); // solicita uma entrada válida
        }
    }

}