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

    let mut players = Vec::new();

    //captura os nomes dos jogadores
    for i in 1..=num_players {
        print!("Digite o nome do jogador {}: ", i);
        io::Write::flush(&mut std::io::stdout()).expect("Falha ao limpar o buffer");
        let mut player_name = String::new();
        io::stdin().read_line(&mut player_name).expect("Erro ao ler entrada");
        players.push(player_name.trim().to_string());
    }

    println!("\nO valor-alvo foi definido!");
    let challenging = random_number();
    println!("O desafio é acertar {} espaços!", challenging);
    println!("Vamos começar!\n");

    let mut scores = Vec::new();

    //cada jogador faz uma tentativa
    for player in &players {
        println!("\nTentativa de {}: ", player);
        instruction_01();

        let mut key_space = String::new();
        loop {
            key_space.clear();
            io::stdin().read_line(&mut key_space).expect("Falha ao ler a entrada");

            if key_space.trim().chars().all(|c| c == ' ') {
                break;
            } else {
                println!("Entrada inválida! Digite apenas espaços.\n");
            }
        }

        //conta quantos espacos foram digitados
        let space_count = key_space.chars().filter(|&c| c == ' ').count();
        scores.push((player.clone(), space_count));
        println!("Contabilizado!");
    }

    //verifica quem chegou mais proximo ao valor-alvo
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

    //exibe o vencedor
    println!(
        "\nO valor-alvo era: {}. \nE o vencedor é... \n{}, com score de {} espaços!",
        challenging, closest_player, closest_guess
    );

    //classifica os jogadores por proximidade ao valor-alvo
    let mut ranked_scores = scores.clone();
    ranked_scores.sort_by(|a, b| {
        let diff_a = (a.1 as isize - challenging as isize).abs();
        let diff_b = (b.1 as isize - challenging as isize).abs();
        diff_a.cmp(&diff_b) // Ordena do menor para o maior (mais próximo ao mais distante)
    });

    //exibe a classificacao final
    println!("\nClassificação:");
    for (i, (player, guess)) in ranked_scores.iter().enumerate() {
        let difference = (*guess as isize - challenging as isize).abs();
        println!(
            "{}. {}: {} espaços (diferença de {})",
            i + 1, player, guess, difference
        );
    }
}
