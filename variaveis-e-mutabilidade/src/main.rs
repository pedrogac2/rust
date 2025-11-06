const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60;

fn main() {
    let x: i32 = 5; // imutável por padrão
    println!("O valor de x é: {x}");

    // x = 6; /* não é possível alterar porque a variável é imutável */

    let x: i32 = 66; /* Mas é possível redeclarar a variável */
    println!("O valor de x é: {x}");

    /* Caso queira criar uma variavel mutavel é necessário usar a palavra 'mut' */
    let mut y = 5; /* Se declarar como mut e não alterar o valor, o compilador irar gerar um warning */
    println!("1 - O valor de y é: {y}");

    y = UMA_HORA_EM_SEGUNDOS;
    println!("2 - O valor de y agora é: {y}");
    
    println!("3 - O valor da constante é: {UMA_HORA_EM_SEGUNDOS}");

    // const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60; /* Ao redaclarar a variável, fica valendo a do escopo mais interno */
}
