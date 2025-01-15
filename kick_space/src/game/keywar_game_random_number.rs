use std::io;

use crate::game::random_number::random_number;
use crate::game::instruction::instruction_01;
use crate::game::check_result::check_result;

pub fn keywar_game_random_number() { //modo de jogo com numero randomico
    println!("Como está sua percepção?");
    let challenging = random_number(); //gera um numero randomico e armazena em challenging ... entre 1 e 100 (pode-se aumentar)

    println!("Hora de testar! Seu desafio: {}!", challenging);

    let mut key_space = String::new(); //faz o armazenamento dos espacos digitados
    loop {
        instruction_01(); //instrucao de jogo
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
