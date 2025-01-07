use std::io;

fn main() {
    println!("Advinhe o número!");

    println!("Insira o número que acha ser...");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Falha na leitura");

    let inserted = guess.trim();

    println!("Você digitou: {}.", inserted);

}
