use rand::Rng;

pub fn check_result(space_count: usize, challenging: usize) {
    if space_count == challenging {
        let messages = vec![
            "VOCÊ ACERTOU!!!",
            "NA MOSCA!!!",
            "PARABÉNS, DESAFIO COMPLETO!",
            "IIINCRÍÍÍVEEEL, VOCÊ CONSEGUIU!"
        ];

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..messages.len());
        println!("{}", messages[random_index]);
    } else {
        println!("Errooooooooooouu, a quantidade era de {}!", challenging);
    }
}
