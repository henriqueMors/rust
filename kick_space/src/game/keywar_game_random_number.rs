use std::io;
use crate::game::check_result::check_result;
use crate::game::instruction::instruction_01;
use crate::game::random_number::random_number;
use diesel::SqliteConnection;

pub fn keywar_game_random_number(conn: &mut SqliteConnection) {
    println!("Como está sua percepção?");
    let challenging = random_number(); // Gera um número aleatório entre 1 e 100

    println!("Hora de testar! Seu desafio: {}!", challenging);

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
    check_result(conn, space_count, challenging);
    println!("Você digitou {} caracteres de espaço.", space_count);
}
