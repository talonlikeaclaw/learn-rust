#![allow(unused)]

// Compound data types
// - tuple
// - array

fn main() {
    // Tuple
    let t: (bool, u32, char) = (true, 1, 'c');

    // Destructure
    let (a, b, c) = t;

    // ignore with _
    let (_, b, _) = t;

    // Empty tuple - unit type
    let t = (); // return void

    // Nested tuple
    let nested = ((1.23, 'a'), (true, 1u32, 'b'), ());

    // Access values via var.index syntax
    let t: (bool, u32, char) = (true, 1, 'c');
    println!("t = {}, {}, {}", t.0, t.1, t.2);

    // Access nested values via var.index.index syntax
    println!("nested {} {}", nested.0.0, nested.1.1);
}
