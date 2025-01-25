use diesel::prelude::*;
use crate::schema::players;

#[derive(Insertable)]
#[table_name = "players"]
pub struct NewPlayer<'a> {
    pub name: &'a str,
    pub score: i32,
}

pub fn create_player(conn: &SqliteConnection, name: &str) {
    let new_player = NewPlayer { name, score: 0 };

    diesel::insert_into(players::table)
        .values(&new_player)
        .execute(conn)
        .expect("Erro ao inserir jogador");
}
