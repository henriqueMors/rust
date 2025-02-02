use crate::game::check_result::check_result;
use crate::game::instruction::instruction_01;
use std::io;

pub fn keywar_game_challenging() {
    println!("#####################################");
    println!("Modo Desafiador Selecionado!");

    print!("Número de espaços: "); // Exibe a frase antes da entrada
    io::Write::flush(&mut std::io::stdout()).expect("Falha ao limpar o buffer"); // Garante que o print apareça antes da entrada do usuário

    let challenging: usize = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a entrada");

        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 => break num,
            _ => println!("Número inválido. Digite um número maior que 0."),
        }
    };

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

    let space_count = key_space.chars().filter(|&c| c == ' ').count();
    check_result(space_count, challenging, "Jogador"); // Adicione o nome do jogador aqui
    println!("Você digitou {} caracteres de espaço.", space_count);
}