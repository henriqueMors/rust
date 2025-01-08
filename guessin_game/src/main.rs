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


}
