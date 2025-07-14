#![allow(unused)]

// Function
// Implicit return
fn add(x: u32, y: u32) -> u32 {
    x + y // implicit return
}

// No output
fn print() {
    println!("no output");
}

// Diverge - function that never returns
fn forever() -> ! {
    loop {}
}

fn crash() -> ! {
    panic!("crash");
}

fn main() {
    // Calling a function
    let x = 1;
    let y = 2;
    let z = add(x, y);
    println!("{x} + {y} = {z}");

    // No output
    print();

    // Diverge - function that never returns
    crash();
}
