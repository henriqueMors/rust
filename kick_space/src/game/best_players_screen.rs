use crate::db::{establish_connection, get_best_players};

pub fn show_best_players() {
    let mut conn = establish_connection();
    let players = get_best_players(&mut conn);

    println!("\n=== MELHORES JOGADORES ===");
    for player in players {
        println!("{} - {} espaços", player.name, player.score);
    }
    println!("=========================\n");
}