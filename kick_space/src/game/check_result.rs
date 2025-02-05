use rand::Rng;
use std::io;
use crate::db::{establish_connection, save_player, get_best_players};
use diesel::SqliteConnection;

pub fn check_result(conn: &mut SqliteConnection, space_count: usize, challenging: usize) {
    if space_count == challenging {
        let messages = vec![
            "VOCÊ ACERTOU!!!",
            "NA MOSCA!!!",
            "PARABÉNS, DESAFIO COMPLETO!",
            "IIINCRÍÍÍVEEEL, VOCÊ CONSEGUIU!"
        ];
        
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..messages.len());
        println!("=====================================");
        println!("{}", messages[random_index]);

        // Pede o nome do jogador
        let mut player_name = String::new();
        println!("Digite seu nome para registrar no ranking: ");
        io::stdin()
            .read_line(&mut player_name)
            .expect("Erro ao ler entrada");
        let player_name = player_name.trim();

        // Salva o jogador no banco de dados
        if let Err(e) = save_player(conn, player_name, space_count as i32) {
            eprintln!("Erro ao salvar o jogador no banco de dados: {}", e);
        } else {
            println!("✅ Jogador salvo no ranking!");
        }

        // Exibir o ranking atualizado
        println!("\n🏆 Top 10 Melhores Jogadores:");
        let best_players = get_best_players(conn);
        for (index, player) in best_players.iter().enumerate() {
            println!("{}. {} - Quantidade Acertada: {}", index + 1, player.name, player.score);
        }
    } else {
        println!("=====================================");
        println!("Errooooooooooouu, era de {}...", challenging);
    }
}
