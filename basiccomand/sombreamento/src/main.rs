fn main() {
    println!("START");
    let x = 5;
    println!("o valor de x é: {}", x);
    let x = x + 1;
    println!("o valor de x é: {}", x);

    {
        let x = 2;
        println!("o valor de x no bloco interno é: {}", x);
    }
    println!("o valor de x depois do bloco interno é: {}", x);


    let spaces = "  ";
    let spaces = spaces.len();         // let cria nova varivavel com novo tipo.
    println!("o valor do espaço é: {}", spaces);
    
    let mut spaces2 = "   ";
    println!("o valor do espaço é: {}", spaces2);
    spaces2 = "qwerty";                        //mesma variavel com mesmo tipo;
    println!("o valor do espaço é: {}", spaces2);

    //spaces2 = spaces2.len();   //valor é mutável. o tipo não é.
    
    println!("END");
}
