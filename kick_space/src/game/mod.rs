//usado para organizar e gerenciar os submodulos de uma pasta no Rust. Ele funciona como o ponto central de configuracao de modulos, permitindo que outros arquivos (como main.rs) acessem os submodulos declarados na pasta.
//pub é para deixar a funcao publica

pub mod random_number;
pub mod game_mode;
pub mod keywar_game_random_number;
pub mod keywar_game_challenging;
pub mod instruction;
pub mod continue_game;
pub mod check_result;