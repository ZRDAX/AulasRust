fn main() {
    let _aa = [1, 2, 3, 4, 5];
    let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];

    let _bb: [i32; 5] = [1, 2, 3, 4, 5];

    let cc = [3; 5];
    let dd = [3, 5];

    println!("cc {:?}", cc);
    println!("dd {:?}", dd);

    // indexa começando pelo elemento 0
    println!("Elemento 2 do array 'meses' é: {}", meses[2]);

    //let errado = cc[11]; //Erro de compilação, panico se detectado na execução
}
