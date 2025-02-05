use diesel::prelude::*;
use diesel::Selectable;

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = best_players)]
pub struct BestPlayer {
    pub name: String,
    pub score: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = best_players)]
pub struct BestPlayer {
    //pub id: Option<i32>,
    pub name: String,
    pub score: i32,
}