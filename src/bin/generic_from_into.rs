#![allow(unused)]

use std::convert::{From, Into};
// From - value-to-value conversions that consumes the input.
#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

/*
pub trait From<T>: Sized {
    // Required method
    fn from(value: T) -> Self;
}
*/

// (u32, u32) -> Point
impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

// Into - value-to-value conversion that consumes the input.
// Automatically implemented if From is implemented
/*
pub trait Into<T>: Sized {
    // Require method
    fn into(self) -> T;
}
*/
fn main() {
    let t: (u32, u32) = (1, 2);
    let p = Point::from(t);
    println!("{:?}", p);

    let p: Point = t.into();
    println!("{:?}", p);
}
