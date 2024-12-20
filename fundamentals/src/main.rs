fn main() {

    //boas vindas ao rust c

    //println!("Hello, world!!");
    //println!("Bem vindo ao RUST!")

    
    //variaveis e mutabilidade
    
    /*
    println!("Inicio amostra de mutabilidade");
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

    /*
    const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60;

    println!("Iniciando mutabilidade...");
    let mut x = 1;
    println!("Até aqui, x equivale a {x}");

    println!("A partir de agora, mudará \nde valor para mostrar quantos \nsegundos existem em uma hora.");
    x = UMA_HORA_EM_SEGUNDOS;
    println!("Então, x agora vale {x} segundos");
    println!("...Encerrando.");
    */

    //blocos e sombreamentos
    /*
    println!("Iniciando blocos...");
    const X: i32 = 5;
    let y = 6;
    let mut z = 7;
    z = z + 1;

    println!("fora do bloco, os numeros sao: x={X}, y={y} e z={z}.");

    {
        const X: i32 = 555;
        let y = 666;
        let mut z = 777;
        z = z + 1;
        println!("Dentro do bloco, os numeros sao: x={X}, y={y} e z={z}.");
    }

    println!("fora novamente do bloco, os numeros voltam a ser: x={X}, y={y} e z={z}.");

    println!("Encerrando...");
    */


    println!("Iniciando sombreamento...");
    let x = 5;
    println!("O valor de x é {x}.");
    let x = x + 1;
    println!("O valor de x é {x}.");

    {
        let x = x * 2; //ele acessa o ultimo x (variavel) com esse valor fora do bloco
        println!("O valor de x no bloco interno é {x}.");
    }
    println!("fora do bloco novamente... x={x}.");

    let spaces = "   ";
    let spaces = spaces.len();


}