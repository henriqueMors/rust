//checagem de selecao de mensagem conforme resultado
use rand::Rng; //biblioteca para obter um numero randomico
use crate::db::{establish_connection, save_player};

pub fn check_result(space_count: usize, challenging: usize, player_name: &str) {
    if space_count == challenging {
        let messages = vec![
            "VOCÊ ACERTOU!!!",
            "NA MOSCA!!!",
            "PARABÉNS, DESAFIO COMPLETO!",
            "IIINCRÍÍÍVEEEL, VOCÊ CONSEGUIU!"
        ];
        
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..messages.len());
        println!("{}", messages[random_index]);

        // Salva o jogador no banco de dados
        let mut conn = establish_connection();
        if let Err(e) = save_player(&mut conn, player_name, space_count as i32) {
            eprintln!("Erro ao salvar o jogador no banco de dados: {}", e);
        }
    } else {
        println!("Errooooooooooouu, a quantidade era de {}...", challenging);
    }
}