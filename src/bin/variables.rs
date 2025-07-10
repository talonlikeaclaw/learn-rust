#![allow(unused)]

fn main() {
    // Variables are immutable by default in Rust
    let x: i32 = -123;
    // Compiler error: cannot assign twice to immutable variable `x`.
    // x += 1;

    // Use `mut` keyword to declare mutable variables
    let mut y: i32 = 123;
    y += 1;

    // Rust can infer type automatically
    let z = -123;

    // Trick to get compiler to tell what type a variable is
    // let w: () = 123;

    // Constants require type and use CAP_LOCK styling
    const NUM: u32 = 1;

    // You can "shadow" variables using `let`
    let x: i32 = -1;
    let x: bool = true;

    // Type placeholder to tell Rust to infer type
    let v: Vec<_> = vec![1, 2, 3];
}
