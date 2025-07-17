#![allow(unused)]

fn modify(s: &mut String) {
    // Does this take ownership?
    // No it doesn't
    *s += "?";
}

fn main() {
    // Deref
    let mut s = String::from("rust");
    let s1 = &mut s;
    *s1 += "?"; // dereference
    println!("{s}");

    let mut s = String::from("rust");
    modify(&mut s);
    println!("{s}");

    // Deref coercion
    // Automatically dereferenced in some situations
    let x = 1;
    let y = &x;
    let z = &x;
    let w = y + z; // automatically dereferences *y + *z
    println!("{y} + {z} = {w}");
}
