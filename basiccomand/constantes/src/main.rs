/* 

    CONSTANTES

*/


//const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60;

//const UMA_HORA_EM_SEGUNDOS = 1 * 60 * 60; pode ? 
//não pode por que em uma constante é nescessario o tipo dela.

//const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 600; //pode ?
// pode, mas não está sendo usando dentro da função main.

fn main() {
    const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60; //escopo interno

    println!("Inicio do programa");
    let mut x: i32 = 5;
    // let mut x: f32 = 5.5;
    println!("x = {}", x);

    x = UMA_HORA_EM_SEGUNDOS;
    println!("x = {}", x);

    println!("Fim do programa");
}
