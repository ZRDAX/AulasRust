fn main() {
    println!("START");
    const X:i32 = 5; 
    let y = 6;
    let mut z = 7;
    z = z + 1;

    println!("No inicio os valores são: X={X}, y={y}, z={z}");

    {
        const X:i32 = 555;
        let y = 777;
        let mut z = 888;
        z = z + 1;
        println!("Dentro do bloco os valores são: X={X}, y={y} z={z}");
    }

    println!("Depois do bloco os valores são: X={X}, y={y} z={z}");

    println!("END");
}
