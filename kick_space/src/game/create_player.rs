use diesel::prelude::*;
use crate::schema::bestplayers;

#[derive(Insertable)]
#[diesel(table_name = bestplayers)] // Atualize se o nome da tabela for diferente
pub struct NewPlayer<'a> {
    pub name: &'a str,
    pub quantity: i32,
}

pub fn create_player(conn: &mut SqliteConnection, name: &str) { // Conexão mutável
    let new_player = NewPlayer { name, quantity: 0 };

    diesel::insert_into(bestplayers::table)
        .values(&new_player)
        .execute(conn)
        .expect("Erro ao inserir jogador");
}
