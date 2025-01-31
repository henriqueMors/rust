use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::models::{NewPlayer, BestPlayer};
use crate::schema::best_players::dsl::*;

pub fn establish_connection() -> SqliteConnection {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Erro ao conectar ao banco de dados: {}", database_url))
}

pub fn save_player(conn: &mut SqliteConnection, player_name: &str, player_score: i32) -> Result<(), diesel::result::Error> {
    let new_player = NewPlayer {
        name: player_name.to_string(),
        score: player_score,
    };

    diesel::insert_into(best_players)
        .values(&new_player)
        .execute(conn)?;

    Ok(())
}

pub fn get_best_players(conn: &mut SqliteConnection) -> Vec<BestPlayer> {
    best_players
        .load::<BestPlayer>(conn)
        .expect("Erro ao carregar os melhores jogadores")
}