use crate::random_number::random_number;
use crate::instruction::instruction;
use crate::check_result::check_result;

pub fn keywar_multiplayer() {
    println!("Modo Multiplayer Selecionado!");

    println!("Digite o número de jogadores:");
    let num_players = instruction("Digite o número de jogadores:").parse::<usize>().unwrap_or(0); //informa o numero de jogadores
    if num_players == 0 {
        println!("Número de jogadores inválido.");
        return;
    }

    let mut players = Vec::new();
    for i in 1..=num_players { //registrar nomes dos jogadores
        let player_name = instruction(&format!("Digite o nome do jogador {}:", i));
        players.push(player_name);
    }

    let target = random_number();
    println!("O valor-alvo foi definido! Vamos começar!");

    let mut scores = Vec::new(); //registra as tentativas
    for player in &players {
        println!("{}: Faça sua tentativa:", player);
        let guess = instruction(&format!("Tentativa de {}: ", player)).parse::<usize>().unwrap_or(0);
        scores.push((player.clone(), guess));
    }

    let mut closest_player = &scores[0]; //verifica quem chegou mais proximo
    let mut smallest_difference = (scores[0].1 as isize - target as isize).abs();

    for (player, guess) in &scores {
        let difference = (*guess as isize - target as isize).abs();
        if difference < smallest_difference {
            smallest_difference = difference;
            closest_player = &(player.clone(), *guess);
        }
    }

    println!( //parciais
        "O valor-alvo era: {}. O vencedor é {} com a tentativa de {}!",
        target, closest_player.0, closest_player.1
    );

    println!("Pontuações:"); //exibe pontuacao
    for (player, guess) in &scores {
        let difference = (*guess as isize - target as isize).abs();
        println!("{} tentou {}, diferença de {}", player, guess, difference);
    }

    check_result(target, closest_player.1); //check final
}