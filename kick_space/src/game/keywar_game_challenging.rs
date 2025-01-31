use std::io;
use crate::game::instruction::instruction_01;
use crate::game::check_result::check_result;

pub fn keywar_game_challenging() { //metodo de jogo com desafio
    println!("Peça para alguem inserir um valor?");
    let mut challenging = String::new(); //armazena uma quantidade numerica para desafio
    io::stdin()
        .read_line(&mut challenging)
        .expect("Falha ao ler a entrada");

    let challenging: usize = match challenging.trim().parse() { //armazena a quantidade e retira espaco
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, insira um número válido!");
            return;
        }
    };

    println!(
        "Você foi desafiado na quantidade de {}!",challenging
    );

    let mut key_space = String::new(); //sera inserido os espaco
    loop {
        instruction_01(); //instrucao
        key_space.clear();//limpa qualquer dados que venha com o key_space
        io::stdin()
            .read_line(&mut key_space)
            .expect("Falha ao ler a entrada");

        if key_space.trim().chars().all(|c| c == ' ') { //aceita somente espaco, invalidando se reconhecer outro caractere
            break;
        } else {
            println!("Entrada inválida! Digite apenas espaços.");
        }
    }

    let space_count = key_space.chars().filter(|&c| c == ' ').count(); //contagem de espacos
    check_result(space_count, challenging); //faz comparativo entre as variaveis para saber o resultado
    println!("Você digitou {} caracteres de espaço.", space_count); //apresenta o valor final
}