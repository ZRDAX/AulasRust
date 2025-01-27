use std::{collections::btree_set::Difference, iter::Product};

//velocidade de qualquer veiculo em metros por segundo. 
const VELOCIDADE_MAXIMA: f64 = 200.0 * (1000.0 / 3600.0);
// Comprimento máximo de qualquer veículo em metros
// const COMPRIMENTO_MAXIMO = 22;


/*
Table 3-1: Integer Types in Rust

Length	Signed	Unsigned
8-bit	i8		u8
16-bit	i16		u16
32-bit	i32		u32
64-bit	i64		u64
128-bit	i128	u128
arch	isize	usize

Obs1: i32 é default para inteiros
Obs2: Em caso de overflow temos "panico na execução" (debug mode) ou "dá a volta" (release mode)
Obs3: Existem vários métodos na biblioteca padrão para lidar com overflow
Obs4: Existem dois tipos de ponto flutuante: f32 e f64 (default)
*/


/*
Table 3-2: Integer Literals in Rust

Number literals		Example
Decimal				98_222
Hex					0xff
Octal				0o77
Binary				0b1111_0000
Byte (u8 only)		b'A'

Obs1: Ponto flutuante aceita  7.6e-2  ou  0.076
*/

fn main() {
    let chassi: i32 = 123456;
    let acel_max: f64 = 3.0;
    let acel_min: f64 = -10.0;
    //let vel_max: f32 = VELOCIDADE_MAXIMA; // as f32; //metros por segundo
    let comprimentro: i32 = 4;
    let posicao_atual: f64 = -100.0;
    let vel_atual: f64 = 0.0;
    let acel_atual: f64 = 0.0;

    // adição 
    let sum = posicao_atual + 10.0;

    // subtração 
    let difference = vel_atual - 4.3;

    //multiplicação 
    let product = comprimentro * 2;

    // divisão 
    let quotient = acel_atual / 2.0;
    let floored = 2 / 3;

    //resto da divisão
    let remainder = 43 % 5;

    // transformações de tipos
    let xxy: f64 = 123.55;

    //let yyy:  = xxy + 88 as f64 ;
    // let yyy = xxy + 88f64;
    // let yyy = xxy + 88 as f64;

    println!("trunc {}, round {}, ceil {}, floor {}",
            xxy.trunc(), xxy.round(), xxy.ceil(), xxy.floor());
    
    println!("Aló numéricos!");

}