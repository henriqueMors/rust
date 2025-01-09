use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {

    println!("Advinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        //println!("O número secreto é: {secret_number}");
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Falha na leitura");


        let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
        //let inserted = guess.trim();
        };

        println!("Você digitou: {guess}.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("é baixo..."),
            Ordering::Greater => println!("é alto..."),
            Ordering::Equal => {
                println!("Acertou!!!");
                break;
            } 
        }

    }



/*

//CODIGO GERADO COM IA


use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Bem-vindo ao jogo: Adivinhe o número!");

    // Gera o número secreto
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Por favor, insira o seu palpite:");

        let mut guess = String::new();

        // Lê a entrada do usuário
        if io::stdin().read_line(&mut guess).is_err() {
            println!("Falha ao ler a entrada. Tente novamente.");
            continue;
        }

        // Converte o palpite para u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, insira um número válido.");
                continue;
            }
        };

        println!("Você digitou: {guess}.");

        // Compara o palpite com o número secreto
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Parabéns, você acertou!");
                break;
            }
        }
    }
}
*/



}
