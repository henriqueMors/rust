use crate::game::random_number::random_number;
use crate::game::instruction::instruction_01;
use std::io;

pub fn keywar_multiplayer() {
    println!("#####################################");
    println!("Modo Multiplayer Selecionado!");

    print!("Número de jogadores: ");
    io::Write::flush(&mut std::io::stdout()).expect("Falha ao limpar o buffer");

    let num_players: usize = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 => break num,
            _ => println!("Número inválido. Digite um número maior que 0."),
        }
    };

    println!("\nO valor-alvo foi definido!");
    let challenging = random_number();
    println!("Vamos começar!\n");

    for _ in 0..num_players {
        instruction_01();
    }

    println!("\nFim do jogo!");
}
