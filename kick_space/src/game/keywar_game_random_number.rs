use crate::game::check_result::check_result;
use crate::game::instruction::instruction_01;
use crate::game::random_number::random_number;
use std::io;

pub fn keywar_game_random_number() {
        println!("#####################################");
        println!("Hora de testar sua percepção!");
    let challenging = random_number(); // Gera um número aleatório entre 1 e 100

    println!("Seu desafio: {}!", challenging);

    let mut key_space = String::new();
    loop {
        instruction_01(); // Exibe as instruções do jogo
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


    
    let mut conn = establish_connection();
check_result(&mut conn, space_count, challenging);

 // Adicione o nome do jogador aqui
    println!("Você digitou {} caracteres de espaço.", space_count);
    println!("=====================================");
}