//checagem de selecao de mensagem conforme resultado
use rand::Rng; //biblioteca para obter um numero randomico

pub fn check_result(space_count: usize, challenging: usize) {
    if space_count == challenging {
        let messages = vec![ //vetor para para agrupar as mensagem que serao sortidas
            "VOCÊ ACERTOU!!!",
            "NA MOSCA!!!",
            "PARABÉNS, DESAFIO COMPLETO!",
            "IIINCRÍÍÍVEEEL, VOCÊ CONSEGUIU!"
        ];
        
        //para escolher uma mensagem aleatoria
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..messages.len());
        println!("{}", messages[random_index]);
    } else {
        println!("Errooooooooooouu, a quantidade era de {}!", challenging); //mensagem em caso de erro
    }
}