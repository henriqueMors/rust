use std::io;

fn main() {

    println!("@#=-KEY WAR-=#@"); // nome do jogo

    println!("Quantos espaços seu adversário deve acertar?");

    let mut challenging = String::new(); // o desafiante digita quantidade de a ser acertada pelo desafiado
        io::stdin()
            .read_line(&mut challenging)
            .expect("Falha ao ler a entrada");
        println!("Você digitou o desafio: {},\nagora, é a vez do próximo acertar essa quantidade, \nsomente segurando a barra de espaço... \nboa sorte!", challenging.trim());
        println!("#Segure a barra de espaço e pressione Enter quando terminar#");
    
    let challenging: usize = match challenging.trim().parse() { // alterando o tipo da variavel para que seja lido como numero
            Ok(num) => num, // vai tratar o erro de caso o usuario digite algo fora do necessario
            Err(_) => {
                println!("Por favor, insira um número válido!");
                return; // Sai do programa
            }
        };

    let mut key_space = String::new(); // armazena a quantidade de espacos
        io::stdin()
            .read_line(&mut key_space)
            .expect("Falha ao ler a entrada");

    let space_count = key_space.chars().filter(|&c| c == ' ').count();
    
    /*  
    space_count recebe a quantidade de espacos digitados.
    key_space esta configurado para receber e aceitar 
    o tipo de caracter espaco, inserido no teclado.
    c é uma configuracao de caracter
    */

    if space_count == challenging { // um IF/ELSE somente para um comparativo
        println!("VOCÊ ACERTOU!!!");
    } else {
        println!("Errooooooooooouu, a quantidade era de {challenging}!");
    }

    println!("Você digitou {space_count} caracteres de espaço."); // mensagem de encerramento
    
}
