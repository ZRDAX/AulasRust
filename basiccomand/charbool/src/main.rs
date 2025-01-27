fn main() {
    let t = true;
    let f: bool = false;

    let x = t && f;
    let y = t || !f;
    let _z = 12 > 13;

    let c = 'z';
    let _c = 'z';

    let z: char = 'æ';

    println!("bool: {x}, char: {c}");
    println!("booly: {y}, charz: {z}");
    
}
