use crate::game::random_number::random_number;
use crate::game::check_result::check_result;
use std::io;

pub fn keywar_multiplayer() {
    println!("Modo Multiplayer Selecionado!");
    
    println!("Digite o número de jogadores:"); //pede o numero de jogadores
    let num_players: usize = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 => break num,
            _ => println!("Número inválido. Digite um número maior que 0."),
        }
    };

    let mut players = Vec::new();

    //registra os nomes dos jogadores
    for i in 1..= num_players {
        println!("Digite o nome do jogador {}:", i);
        let mut player_name = String::new();
        io::stdin().read_line(&mut player_name).expect("Erro ao ler entrada");
        players.push(player_name.trim().to_string());
    }

    let challenging = random_number(); // Gera um número aleatório entre 1 e 100
    println!("O valor-alvo foi definido! Desafio: {} espaços!", challenging);
    println!(" Vamos começar!");

    let mut scores = Vec::new();

    // Cada jogador faz uma tentativa
    for player in &players {
        println!("Tentativa de {}: ", player);
        let guess: usize = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
            match input.trim().parse::<usize>() {
                Ok(num) => break num,
                _ => println!("Tentativa inválida. Digite um número válido."),
            }
        };
        scores.push((player.clone(), guess));
    }

    //verifica quem chegou mais proximo ao valor-alvo
    let (mut closest_player, mut smallest_difference) = scores[0].clone();
    let mut smallest_difference_value = (smallest_difference as isize - target as isize).abs();

    for (player, guess) in &scores {
        let difference = (*guess as isize - target as isize).abs();
        if difference < smallest_difference_value {
            smallest_difference = *guess;
            closest_player = player.clone();
            smallest_difference_value = difference;
        }
    }

    //exibe o vencedor
    println!(
        "O valor-alvo era: {}. O vencedor é {} com a tentativa de {}!",
        target, closest_player, smallest_difference
    );

    //exibe as pontuações
    println!("Pontuações:");
    for (player, guess) in &scores {
        let difference = (*guess as isize - target as isize).abs();
        println!("{} tentou {}, diferença de {}", player, guess, difference);
    }

    //verifica o resultado final
    check_result(target, smallest_difference);
}
