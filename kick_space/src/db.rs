use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::models::{NewPlayer, BestPlayer};
use crate::schema::best_players::dsl::*;
use crate::schema::best_players;

pub fn establish_connection() -> SqliteConnection {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Erro ao conectar ao banco de dados: {}", database_url))
}

pub fn save_player(conn: &mut SqliteConnection, player_name: &str, player_score: i32) -> Result<(), diesel::result::Error> {
    use crate::schema::best_players::dsl::*;

    let count: i64 = best_players.count().get_result(conn)?;

    if count < 10 {
        // Ainda há espaço, inserir normalmente
        let new_player = NewPlayer {
            name: player_name.to_string(),
            score: player_score,
        };

        diesel::insert_into(best_players)
            .values(new_player)
            .execute(conn)?;

    } else {
        // Ranking cheio, verificar se o novo jogador pode substituir alguém
        let worst_player = best_players.order(score.asc()).first::<BestPlayer>(conn);

        if let Ok(worst) = worst_player {
            if player_score > worst.score {
                // Substituir o pior jogador
                diesel::delete(best_players.filter(id.eq(worst.id)))
                    .execute(conn)?;

                let new_player = NewPlayer {
                    name: player_name.to_string(),
                    score: player_score,
                };

                diesel::insert_into(best_players)
                    .values(new_player)
                    .execute(conn)?;
            }
        }
    }

    Ok(())
}


pub fn get_best_players(conn: &mut SqliteConnection) -> Vec<BestPlayer> {
    use crate::schema::best_players::dsl::*;

    best_players
        .order(score.desc()) // Ordena pela quantidade acertada (maior primeiro)
        .limit(10) // Mantém apenas os 10 melhores
        .load(conn)
        .expect("Erro ao carregar os melhores jogadores")
}
