use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenvy::dotenv().ok(); // Carrega o arquivo .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não foi definido");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Erro ao conectar ao banco de dados em {}", database_url))
}