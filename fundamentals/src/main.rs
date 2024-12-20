fn main() {

    //boas vindas ao rust c

    //println!("Hello, world! Versão Cargo!");
    //println!("Que venha o rust!")

    
    //variaveis e mutabilidade
    
    /*
    println!("Inicio do programa");
    let mut x = 50;        //o i32 vem automaticamente pelo codigo
                                //let é uma variavel imutavel. mutavel com MUT
    println!("O valor de x é: {x}"); //abre e fecha chaves e insere a variavel

    x = 6;

    let x = 666; // antiga variavel é destruida e a memoria liberada
    println!("O valor de x agora é: {x}");

    let mut y = 5;
    println!("O valor de y é: {y}");
    y = 6;
    println!("O valor de y agora é: {y}") 
    */

    const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60;

    println!("Iniciando...");
    let mut x = 1;
    println!("Até aqui, x equivale a {x}");

    println!("A partir de agora, mudará de valor para \nmostrar quantos segundos existem em uma hora");
    x = UMA_HORA_EM_SEGUNDOS;
    println!("Então, x agora vale {x} segundos");
    println!("...Encerrando.")
 
}
