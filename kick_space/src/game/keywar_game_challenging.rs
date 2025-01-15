use std::io;
use crate::game::instruction::instruction_01;
use crate::game::check_result::check_result;

pub fn keywar_game_challenging() {
    println!("Quantos espaços seu adversário deve acertar?");
    let mut challenging = String::new();
    io::stdin()
        .read_line(&mut challenging)
        .expect("Falha ao ler a entrada");

    let challenging: usize = match challenging.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, insira um número válido!");
            return;
        }
    };

    println!(
        "Você digitou o desafio: {},\nagora, é a vez do próximo acertar essa quantidade, \nsomente segurando a barra de espaço.",
        challenging
    );

    let mut key_space = String::new();
    loop {
        instruction_01();
        key_space.clear();
        io::stdin()
            .read_line(&mut key_space)
            .expect("Falha ao ler a entrada");

        if key_space.trim().chars().all(|c| c == ' ') {
            break;
        } else {
            println!("Entrada inválida! Digite apenas espaços.");
        }
    }

    let space_count = key_space.chars().filter(|&c| c == ' ').count();
    check_result(space_count, challenging);
    println!("Você digitou {} caracteres de espaço.", space_count);
}
