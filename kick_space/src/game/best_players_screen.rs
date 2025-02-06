use crate::db::{establish_connection, get_best_players};

pub fn show_best_players() {
    let mut conn = establish_connection();
    let mut players = get_best_players(&mut conn);

    // Ordenar do maior para o menor score
    players.sort_by(|a, b| b.score.cmp(&a.score));

    println!("#####################################");
    println!("##       MELHORES JOGADORES        ##");
    println!("#####################################");
    println!("=====================================");
    for player in players {
        println!("{} - {} espaços", player.name, player.score);
    }
    println!("=====================================");
}
