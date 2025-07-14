#![allow(unused)]

// Handle errors using Option or Result

#[derive(Debug)]
enum MathError {
    DivByZero,
    Other,
}

fn div(x: u32, y: u32) -> u32 {
    x / y // will panic if dividing by 0
}

fn div_safe(x: u32, y: u32) -> Result<u32, MathError> {
    if y == 0 {
        return Err(MathError::DivByZero);
    }
    Ok(x / y)
}

fn main() {
    // Error
    // panic!("crash");

    // Option or Result
    let arr = [];
    // arr[9]; // will panic, index out of range
    // handle gracefully
    // Option<&i32> = Some(&i32) | None
    let x: Option<&i32> = arr.get(9);
    match x {
        Some(val) => println!("val = {val}"),
        None => println!("none"),
    }

    let x = 3;
    let y = 2;
    // let z = div(x, y); // panics
    // Result<u32, MathError>
    let z = div_safe(x, y);
    match z {
        Ok(val) => println!("{x} / {y} = {val}"),
        Err(err) => println!("err = {:?}", err),
    }
}
