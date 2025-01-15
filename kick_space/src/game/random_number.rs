use rand::Rng;

// gerador automatico
pub fn random_number() -> usize { // -> usado para que ele seja um return com um valor i32
    let mut rng = rand::thread_rng(); // gerador de numeros aleatorios
    rng.gen_range(1..=100) // retorna um numero entre 1 e 100 -- como utilizei usize, pode-se indicar um valor maior 
}