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

    let palavra = String::from("palavra");

    let ql = contar_letras(&palavra);

    println!("A palavra '{palavra}' tem exatamente '{ql}' letras.");


}

fn contar_letras(palavra: &str) -> usize {
    palavra.chars().count()
}