fn main() {

    /*

    /*
        1. Programa Fácil
        Crie um programa que peça ao usuário para inserir um número e,
        em seguida, informe se ele é par ou ímpar. Utilize as estruturas
        de controle e tipos numéricos que já aprendeu.
    */

    println!("::Desenvolvimento fácil::");
    println!("Par ou Ímpar");

    let number: i32 = 432; //numero do usuario

    let resultado = if number % 2 == 0 { //variavel tendo a atribuicao de um if
        "par" //nao precisa de println para o resultado
    } else {
        "impar"
    };

    println!("O número que o usuário digitou é {number}, ele é {resultado}.");

    */

    use std::io;
    
    println!("Bóra calcular números primos!");
    
        println!("Digite o número para saber se ele é primo: ");
        let mut numero = String::new();
    
        io::stdin()                             //ler entrada de dados pelo teclado
            .read_line(&mut numero)             //carrega consigo uma quebra de linha* usei o .trim() para retirar a quebra de linha
            .expect("Falha ao ler a entrada");
    
        let numero: u64 = match numero.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Por favor, insira um número válido.");
                return;
            }
        };
    
        if is_primo(numero) {
            println!("O número {numero} é primo!");
        } else {
            println!("O número {numero} não é primo!");
        }
    
    // Função para verificar se um número é primo
    fn is_primo(numero: u64) -> bool {
        if numero <= 1 {
            return false;
        }
    
        let mut divisor: u64 = 2;
            while divisor * divisor <= numero {
                if numero % divisor == 0 {
                    return false;
            }
            divisor += 1;
        }
    
        true
    }

}