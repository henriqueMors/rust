// verifica o resultado e exibe uma mensagem
fn check_result(space_count: usize, challenging: usize) {
    if space_count == challenging {
        
        let messages = vec![ // vetor com mensagens de comemoracao (nao consegui ver nenhuma em tela kkkkkkk)
            "VOCÊ ACERTOU!!!",
            "NA MOSCA!!!",
            "PARABÉNS, DESAFIO COMPLETO!",
            "IIINCRÍÍÍVEEEL, VOCÊ CONSEGUIU!"
        ];

        // indice aleatorio para escolher a mensagem
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..messages.len()); // escolhe um índice válido para o vetor

        // exibe uma mensagem aleatoria
        println!("{}", messages[random_index]);
    } else {
        println!("Errooooooooooouu, a quantidade era de {}!", challenging);
    }
}