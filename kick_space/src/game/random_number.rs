use rand::Rng;

//funcao para gerar um numero aleatorio entre 1 e 100
pub fn random_number() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=100) // Retorna um número aleatório dentro do intervalo especificado
}
