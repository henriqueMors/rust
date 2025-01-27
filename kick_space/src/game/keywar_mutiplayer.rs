use std::io;

pub fn keywar_multiplayer() {
    println!("Modo Multiplayer Selecionado!");

    // 1. Número de jogadores
    println!("Digite o número de jogadores:");
    let mut num_players = String::new();
    io::stdin()
        .read_line(&mut num_players)
        .expect("Erro ao ler entrada");
    let num_players: usize = match num_players.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Por favor, insira um número válido.");
            return;
        }
    };

    // 2. Registrar os nomes dos jogadores
    let mut players: Vec<String> = Vec::new();
    for i in 1..=num_players {
        println!("Digite o nome do jogador {}:", i);
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Erro ao ler entrada");
        players.push(name.trim().to_string());
    }

    // 3. Valor-alvo
    let target: usize = rand::thread_rng().gen_range(1..=100); // Valor aleatório entre 1 e 100
    println!("O valor-alvo foi definido. Vamos começar!");

    // 4. Tentativas dos jogadores
    let mut scores: Vec<(String, usize)> = Vec::new(); // Nome e tentativa de cada jogador
    for player in &players {
        println!("{}: Faça sua tentativa:", player);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Erro ao ler entrada");
        let guess: usize = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Entrada inválida, sua tentativa será 0.");
                0
            }
        };
        scores.push((player.clone(), guess));
    }

    // 5. Determinar o vencedor
    let mut closest_player = &scores[0];
    let mut smallest_difference = (scores[0].1 as isize - target as isize).abs();

    for (player, guess) in &scores {
        let difference = (*guess as isize - target as isize).abs();
        if difference < smallest_difference {
            smallest_difference = difference;
            closest_player = &(player.clone(), *guess);
        }
    }

    // 6. Resultado
    println!(
        "O valor-alvo era: {}. O vencedor é {} com a tentativa de {}!",
        target, closest_player.0, closest_player.1
    );

    println!("Pontuações:");
    for (player, guess) in scores {
        let difference = (guess as isize - target as isize).abs();
        println!("{} tentou {}, diferença de {}", player, guess, difference);
    }
}