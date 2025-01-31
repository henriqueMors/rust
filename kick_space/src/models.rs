use crate::schema::best_players;

#[derive(Insertable)]
#[diesel(table_name = best_players)]
pub struct NewPlayer {
    pub name: String,
    pub score: i32,
}

#[derive(Queryable)]
pub struct BestPlayer {
    pub id: i32,
    pub name: String,
    pub score: i32,
}