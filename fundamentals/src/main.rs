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

    /*
    println!("Iniciando sombreamento...");
    let x = 5;
    println!("O valor de x é {x}.");
    let x = x + 1;          // ele recria a variavel, sem a ncessidade de mut
    println!("O valor de x é {x}.");

    {
        let x = x * 2; //ele acessa o ultimo x (variavel) com esse valor fora do bloco
        println!("O valor de x no bloco interno é {x}.");
    }
    println!("fora do bloco novamente... x={x}.");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    let mut spaces2 = "  ";
    println!("o valor de scaes2 é {spaces2}");
    spaces2 = "qwerty";
    println!("o valor de scaes2 é {spaces2}");
    */

    /*
    println!("numéricos...");

    const VEL_MAX: f64 = 200.0 * (1000.0 / 3600.0);

    let chassi = 123321;
    let acel_max = 3.0;
    let acel_min = -10.0;
    let vel_max = VEL_MAX as f32; //as f32 já esta sendo aceito automaticamente
    let comprimento = 4;
    let posicao_atual = -100.0;
    let vel_atual = 0.0;
    let acel_atual = 0.0;

    //adicao
    let sum = posicao_atual + 10.0;

    //subt
    let diferenca = vel_atual - 4.3;

    //mult
    let produto = comprimento * 2;  //pode 2.0? nao pode pois o 'comprimento' já está definido como i32

    //divisao
    let coef = acel_atual / 2.0;
    let floored = 2 / 3;

    //resto
    let resto = 43 % 5;

    //transformando tipo
    let xxx: f64 = 123.06 as f64;

    //let yyy: <f64 as Add<i32>>::output = xxx + 88; //nao pode adicionar um inteiro a um f64
    //let yyy = xxx + 88 as f64;
    let yyy = xxx + 88f64; //informando o compilador que o tipo da variavel pode ser alterado sem problemas

    println!("trunc {}, round {}, ceil {}, floor {}", 
            xxx.trunc(), xxx.round(), xxx.ceil(), xxx.floor());
            //trunc -> desconsidera o decimal (X.0)
            //round -> arredondar (1.4 -> 1)
            //ceil  -> arredonda para cima (1.4 -> 2)
            //floor -> arredonda para baixo (1.4 -> 1)

    println!("é isso... encerrando numéricos!")
    */


    //booleanos
    /*
    println!("E aqui estao os valores booleanos...");
    let t: bool = true;
    let f = false;

    let x = t && f;
    let y = t || !f;
    let _z = 12 < 13;
    
    let cc = 'z';
    let _c = 'z'; //underline somente para nao apresentar o aviso de erro/falha/problema

    let z = 'ℤ'; //caracteres especiais no linux >>shift + ctrl + u + valor<<

    println!("{}, {}, {}, {}, _z = {}, {}, {}, {}", 
                t, f, x, y, _z, cc, _c, z);
    
    println!("E aqui estao os valores booleanos...");
    */

    /*

    println!("tuplas");

    let tup1: (i32, f64, bool) = (500, 6.4, true); //vc pode definir o tipo da variavel
    let tup2 = (500, 6.4, true); //o proprio rust define o tipo das variaveis

    println!("{:?}", tup1);
    println!("{:?}", tup2);
    println!("{:?}, {:?}", tup1.0, tup1.2); //pegar osmente elementos seprados da tupla

    println!("{:?}", () ); //tupla vazia

    */

    /*

    println!("Arrays...");
    let aa = [1, 2, 3, 4, 5, 6]; //o compilador sabe que se trata de um array com 6 valores
    let meses = ["Jan", "Fev", "Mar", "Abr", "Maio", "Jun", "Jul", "Ago", "Set", "Out", "Nov", "Dez"];

    let bb: [i32; 5] = [1, 2, 3, 4]; //se vc informa o tipo e a qtdade de elementos para o compilador, ele vai reclamar

    let cc = [3; 5]; //vai repedir 5 vezes o valor 3, que é o elemento
    let dd = [3, 5]; // vai imprimir o 3 e o 5

    println!("{:?}", cc); //pode-se pegar tbm somente um elemento com cc[2]
    println!("{:?}", meses[2]);

    println!("Encerrando arrays!");
    
     */


    println!("Vamos as funcoes...");

        /*
        fn outra_funcao() {
            println!("Hello World!");
        }

        fn outra_funcao(x: i32) {
            println!("Outra funcao recebeu {x}.");
        }

        fn print_labeled_measurement(valor: f64, unidade: char) {
            println!("A medida é: {valor} {unidade}.");
        }

        fn soma(x: i32, y: i32) -> i32 {
            x+y //sem o ponto e virgula = é o retorno da funcao
            //return x+y;    // ; o que acontece? = vai ter o mesmo resultado
        }


        */

    


}