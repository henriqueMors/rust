// gerador automatico
fn random_number() -> usize { // -> usado para que ele seja um return0 com um valor i32
    let mut rng = rand::thread_rng(); // gerador de numeros aleatorios
    rng.gen_range(1..=100) // retorna um numero entre 1 e 100
}