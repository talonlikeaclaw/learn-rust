#![allow(unused)]

fn main() {
    let s = String::from("rust");
    let s1 = &s;
    let s2 = &s;

    // Borrow - temporarily use a value without taking ownership
    // - Creates a reference (eitaer mutable or immutable)
    // - Doesn'r move ownership

    // Immutable borrow
    let s = String::from("rust");
    let s1 = &s;
    // any number of read-only access to a value
    let s2 = &s;
    let s3 = s2;

    // Mutable borrow
    // only one mutable reference at a time
    let mut s = String::from("rust");
    let s0 = &mut s; // only valid until next line
    let s1 = &mut s;
    // let s2 = &mut s;
    s1.push_str("🦀");
    println!("{s}"); // s was changed through s1

    // Cannot borrow immutable and mutable simultaneously
    let mut s = String::from("rust");
    let s1 = &s;
    let s2 = &s;
    // let s3 = &mut s; // was already borrow immutable
    println!("{s1} {s2}");
    // println!("{s3}");

    // Reference must not outlive the value
    let s = String::from("rust");
    let s1 = &s;
    // {
    //     let s1 = s;
    // } // s1 is dropped
    // std::mem::drop(s); // makes compiler error

    println!("{s1}");
}

// fn f(s: String) -> &String {
//     &s // s is dropped here so you cannot reference it
// }
