use crate::game::random_number::random_number;
use crate::game::check_result::check_result;
use crate::game::instruction::instruction_01;
use std::io;

pub fn keywar_multiplayer() {
    println!("Modo Multiplayer Selecionado!");

    // Pede o número de jogadores
    println!("Digite o número de jogadores:");
    let num_players: usize = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 => break num,
            _ => println!("Número inválido. Digite um número maior que 0."),
        }
    };

    let mut players = Vec::new();

    // Registra os nomes dos jogadores
    for i in 1..=num_players {
        println!("Digite o nome do jogador {}:", i);
        let mut player_name = String::new();
        io::stdin().read_line(&mut player_name).expect("Erro ao ler entrada");
        players.push(player_name.trim().to_string());
    }

    let challenging = random_number(); // Gera um número aleatório entre 1 e 100
    println!("O valor-alvo foi definido! Desafio: {} espaços!", challenging);
    println!("Vamos começar!");

    let mut scores = Vec::new();

    // Cada jogador faz uma tentativa
    for player in &players {
        println!("Tentativa de {}: ", player);
        instruction_01(); // Exibe as instruções do jogo

        let mut key_space = String::new();
        loop {
            key_space.clear();
            io::stdin()
                .read_line(&mut key_space)
                .expect("Falha ao ler a entrada");

            // Verifica se a entrada contém apenas espaços
            if key_space.trim().chars().all(|c| c == ' ') {
                break;
            } else {
                println!("Entrada inválida! Digite apenas espaços.");
            }
        }

        // Conta quantos espaços foram digitados
        let space_count = key_space.chars().filter(|&c| c == ' ').count();
        scores.push((player.clone(), space_count));
        println!("{} digitou {} espaços.", player, space_count);
    }

    // Verifica quem chegou mais próximo ao valor-alvo
    let (mut closest_player, mut closest_guess) = scores[0].clone();
    let mut smallest_difference = (closest_guess as isize - challenging as isize).abs();

    for (player, guess) in &scores {
        let difference = (*guess as isize - challenging as isize).abs();
        if difference < smallest_difference {
            smallest_difference = difference;
            closest_player = player.clone();
            closest_guess = *guess;
        }
    }

    // Exibe o vencedor
    println!(
        "O valor-alvo era: {}. \nand, the winner is.... \n{}, com score de {} espaços!",
        challenging, closest_player, closest_guess
    );

    // Exibe as pontuações
    println!("\nPontuações:");
    for (player, guess) in &scores {
        let difference = (*guess as isize - challenging as isize).abs();
        println!("{} acertou {} espaços, diferença de {}", player, guess, difference);
    }

    // Verifica o resultado final
    check_result(challenging, closest_guess);
}