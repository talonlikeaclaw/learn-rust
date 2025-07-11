#![allow(unused)]

// Attributes
#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl { h: u8, s: u8, l: u8 },
}

fn main() {
    // Create Enum
    let color: Color = Color::Red;
    let color = Color::Green;
    let color = Color::Rgba(0, 0, 255, 0.1);
    let color = Color::Hex("#ffffff".to_string());
    let color = Color::Hsl { h: 0, s: 1, l: 200 };

    // Attributes -- Debug and PartialEq
    // Debug
    println!("{:#?}", color);
    // PartialEq
    println!("{}", Color::Red == Color::Green);
    println!("{}", Color::Green == Color::Green);

    // Common Enums
    // Option = Some(11) | None
    // Use when getting item from array to avoid missing index
    let x: Option<i32> = None;
    let x: Option<i32> = Some(-11);
    println!("option: {:?}", x);

    // Result = Ok(10) | Err("div by 0")
    let res: Result<u32, String> = Ok(5);
    let res: Result<u32, String> = Err("div by 0".to_string());
    println!("result: {:?}", res);
}
