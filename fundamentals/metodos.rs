// ---------------------------
// Métodos para Strings
// ---------------------------
.trim()        // Remove espaços em branco e caracteres de nova linha do início e fim de uma string.
.to_string()   // Converte um valor em uma String.
.len()         // Retorna o comprimento da string.
.push()        // Adiciona um único caractere ao final de uma string mutável.
.push_str()    // Adiciona uma string inteira ao final de uma string mutável.
.replace()     // Substitui todas as ocorrências de uma substring por outra.
.to_uppercase()// Converte a string para maiúsculas.
.to_lowercase()// Converte a string para minúsculas.

// ---------------------------
// Métodos para Arrays e Vetores
// ---------------------------
.len()         // Retorna o número de elementos.
.push()        // Adiciona um elemento ao final de um vetor mutável.
.pop()         // Remove e retorna o último elemento do vetor.
.contains()    // Verifica se o vetor contém um valor específico.
.sort()        // Ordena os elementos em ordem crescente.
.reverse()     // Inverte a ordem dos elementos.

// ---------------------------
// Métodos para Iteradores
// ---------------------------
.iter()        // Cria um iterador imutável.
.map()         // Aplica uma função a cada elemento do iterador.
.filter()      // Retorna apenas os elementos que satisfazem uma condição.
.collect()     // Converte o iterador em uma coleção, como um vetor.
.enumerate()   // Retorna um iterador que inclui o índice e o valor.

// ---------------------------
// Métodos para Option e Result
// ---------------------------
.unwrap()      // Retorna o valor contido, mas gera um erro se for None ou Err.
.unwrap_or()   // Retorna o valor contido ou um valor padrão caso seja None.
.expect()      // Similar ao .unwrap(), mas com uma mensagem de erro personalizada.
.is_some()     // Verifica se o valor é Some.
.is_none()     // Verifica se o valor é None.

// ---------------------------
// Métodos Matemáticos
// ---------------------------
// Operações Básicas
add            // Soma de dois valores.
sub            // Subtração de dois valores.
mul            // Multiplicação de dois valores.
div            // Divisão de dois valores.
rem            // Resto da divisão (módulo).

// Funções Avançadas
.abs()         // Retorna o valor absoluto.
.powi(exp)     // Eleva o número a uma potência inteira.
.powf(exp)     // Eleva o número a uma potência de ponto flutuante.
.sqrt()        // Retorna a raiz quadrada.
.cbrt()        // Retorna a raiz cúbica.
.exp()         // Retorna e^x, onde e é a base do logaritmo natural.
.ln()          // Retorna o logaritmo natural (base e).
.log(base)     // Retorna o logaritmo em uma base especificada.
.sin()         // Calcula o seno (em radianos).
.cos()         // Calcula o cosseno (em radianos).
.tan()         // Calcula a tangente (em radianos).
.asin()        // Retorna o arco-seno (em radianos).
.acos()        // Retorna o arco-cosseno (em radianos).
.atan()        // Retorna o arco-tangente (em radianos).
.atan2(y, x)   // Retorna o arco-tangente de y/x, levando em conta os sinais.
.hypot(x, y)   // Retorna a hipotenusa (√(x² + y²)).

// Arredondamento e Truncamento
.round()       // Arredonda para o inteiro mais próximo.
.ceil()        // Arredonda para o maior inteiro mais próximo.
.floor()       // Arredonda para o menor inteiro mais próximo.
.trunc()       // Descarta a parte decimal (truncamento).
.fract()       // Retorna a parte fracionária do número.

// Comparações
.min()         // Retorna o menor valor entre dois números.
.max()         // Retorna o maior valor entre dois números.
.clamp(min, max)// Restringe um valor para que fique dentro de um intervalo.

// Aleatoriedade
rand::random() // Gera um número aleatório.
rand::thread_rng().gen_range(start..end)
// Gera um número aleatório dentro de um intervalo.
