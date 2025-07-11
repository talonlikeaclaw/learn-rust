#![allow(unused)]

fn main() {
    // Basic +, -, *, /
    let a: i32 = 1;
    let b: i32 = 2;
    // Addition
    let c: i32 = a + b;
    println!("{a} + {b} = {c}");
    // Subtraction
    let c: i32 = a - b;
    println!("{a} - {b} = {c}");
    // Multiplication
    let c: i32 = a * b;
    println!("{a} * {b} = {c}");
    // Division rounds down (1/2 = 0)
    let c: i32 = a / b;
    println!("{a} / {b} = {c}");

    // % (remainder != mod operator)
    // a % b = r, 0 <= r < b
    // -1 % 2 = 1
    // -1 % 2 = -1
    let a = -23;
    let b = 8;
    let rem = a % b;
    println!("{a} % {b} = {rem}");

    // Literals
    // You can add type notations to values
    let a = 1i32;
    let b = 3u64;
    // You can use scientific notation
    let c = 1.23e3; // 1.23 x 10^3 = 1230
    // Use underscores as thousands separators
    let d = 1_000_000_000u32;

    // Boolean
    let a = true && false;
    let a = true || false;
    let a = !true;

    // Bitwise - binary operators
    // 181
    let a: u8 = 5;
    // 011
    let b: u8 = 3;
    println!("a & b = {:03b}", a & b); // 001
    println!("a | b = {:03b}", a | b); // 111
    println!("a ^ b = {:03b}", a ^ b); // 110
    println!("!a = {:03b}", !a); // 11111010
    // 1 shifted over 3 times
    println!("1 << 3 = {:03b}", 1u32 << 3); // 1000
    println!("10 >> 3 = {:03b}", 1u32 >> 2); // 000
}
