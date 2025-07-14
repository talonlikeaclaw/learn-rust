#![allow(unused)]

fn main() {
    // unwrap and expect
    // Option
    let x: Option<i32> = Some(3);
    match x {
        Some(val) => println!("val = {val}"),
        None => println!("none"),
    }
    // get value out of Some
    let v = x.unwrap();
    let v = x.expect("custom error message");
    println!("val = {v}");

    // Result
    let x = 1;
    let y = 1;
    let z: Result<u32, String> = Ok(x / y);
    // match z {
    //     Ok(val) => println!("val = {val}"),
    //     Err(err) => println!("err = {:?}", err),
    // }
    // gets the Ok value
    let q = z.unwrap();
}
