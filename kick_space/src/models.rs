use diesel::prelude::*;
use crate::schema::best_players;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = best_players)]
pub struct NewPlayer {
    pub name: String,
    pub score: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = best_players)]
pub struct BestPlayer {
    pub id: Option<i32>,
    pub name: String,
    pub score: i32,
}