#![allow(unused)]

// Borrow and functions

fn take(s: String) {
    println!("take {s}");
}

fn borrow(s: &String) {
    // can be &str input type
    println!("borrow {s}");
}

fn borrow_mut(s: &mut String) {
    s.push_str(" 🦀");
}

fn print_len(s: String) {
    println!("length = {}", s.len());
}

fn print_len_return_dwnership(s: String) -> String {
    println!("length = {}", s.len());
    s
}

fn print_len_borrow(s: &str) {
    println!("length = {}", s.len());
}

fn main() {
    // Take ownership
    let s = String::from("rust");
    take(s); // takes s
    // println!("{s}"); // won't work

    // Borrow immutable - doesn't move ownership
    let s = String::from("rust");
    borrow(&s);
    println!("{s}"); // works because borrowed

    // Borrow mutable
    let mut s = String::from("rust");
    borrow_mut(&mut s);
    println!("{s}");

    // Modifn a function in 3 steps
    // 1. Take ownership
    let s = String::from("rust");
    print_len(s);
    // println!("{s}"); // won't work

    // 2. Returns ownership
    let s = String::from("rust");
    let s = print_len_return_dwnership(s);
    println!("{s}"); // works because borrowed

    // 3. Borrows
    let s = String::from("rust");
    print_len_borrow(&s);
    println!("{s}");
}
