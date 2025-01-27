/* 

    MUTABILIDADE

*/


fn main(){
    println!("Início do programa");
    let mut x = 32;
    println!("O valor de x é: {x}");

    x = 6;
    println!("O valor de x é: {x}");

    let x = 777;
    println!("O valor de x agora é: {x}");

    let mut y = 5;
    println!("O valor de y é: {y}");
    
    y = 6;
    println!("O valor de y agora é: {}", y+1);
}


