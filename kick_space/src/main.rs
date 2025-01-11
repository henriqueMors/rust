use std::io;
use rand::{random, Rng};

fn main() {

    println!("@#=-KEY WAR-=#@"); // nome do jogo

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
        println!("#Pressione somente a barra de espaço e pressione Enter quando terminar# \nGood LucK!");
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

    if space_count == challenging { // um IF/ELSE somente para um comparativo
        println!("VOCÊ ACERTOU!!!");
    } else {
        println!("Errooooooooooouu, a quantidade era de {challenging}!");
    }

    println!("Você digitou {space_count} caracteres de espaço."); // mensagem de encerramento
    
}


fn random_number() {
    let mut rng = rand::thread_rng(); // cria um gerador de números aleatórios
    let random_number: usize = rng.gen_range(1..=100); // gera um número entre 1 e 100
    println!("Número aleatório gerado: {}", random_number);
}

