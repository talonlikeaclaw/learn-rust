#![allow(unused, warnings)]

// Monomorphization copys generic type code into typed versions at compilation
// Increases compilation time and binary size but reduces runtime overhead
struct Point<T> {
    x: T,
    y: T,
}

// Example of the type created by Rust
struct Point_u32 {
    x: u32,
    y: u32,
}

// Function with generic type
fn get_x<T>(p: Point<T>) -> T {
    p.x
}

// When calling generic function with a type Rust implements typed version
fn get_x_u32(p: Point_u32) -> u32 {
    p.x
}

fn main() {
    // When making this Point with a type Rust creates instance for that type
    let p0: Point<u32> = Point { x: 0, y: 0 };

    get_x(p0);
}
