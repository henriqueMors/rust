#[diesel(table_name = players)] // Novo formato
#[derive(Insertable)]
pub struct NewPlayer<'a> {
    pub name: &'a str,
    pub score: i32,
}