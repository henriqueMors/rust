use diesel::prelude::*;
use crate::schema::bestplayers;

#[derive(Insertable)]
#[table_name = "bestplayers"]
pub struct NewPlayer<'a> {
    pub name: &'a str,
    pub quantity: i32,
}

pub fn create_player(conn: &SqliteConnection, name: &str) {
    let new_player = NewPlayer { name, quantity: 0 };

    diesel::insert_into(bestplayers::table)
        .values(&new_player)
        .execute(conn)
        .expect("Erro ao inserir jogador");
}
