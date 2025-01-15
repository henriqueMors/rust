use std::io;
use rand::Rng;

mod continueGame; //importando uma fn externa / lembrando de implementar o pub e o :: no metodo
mod gameMode;


fn main() {

    println!("@#=-KEY WAR-=#@");

    loop {
        gameMode::game_mode(); // aciona o jogo principal
        if !continueGame::continue_game() { // verifica se o jogador quer continuar
            break;
        }
    }
    
    println!("Jogo encerrado, até a próxima!");
}
