#![allow(unused)]

// String and str
fn take_string(s: String) {}

fn borrow_string(s: &String) {}

fn make_string() -> String {
    "".to_string()
}

fn mut_string(s: &mut String) {
    s.push_str("?");
}

fn borrow_str(s: &str) {}

// These wont compile
// fn take_str(s: str) {} // str needs to be a reference
//
// fn make_str() -> str { // str needs to be a reference
//     ""
// }
//
// referece outlives the value
// fn make_str() -> &str { // can't return dropped value
//     let s = "";
//     s // s is dropped here
// }

fn main() {
    // String
    // pub struct String {
    //     vec: Vec<u8>,
    // }
    // - Owned
    // - Mutable, growable
    // - Allocated on the heap
    // - &String can be coerced into &str
    let s = String::from("rust");
    take_string(s); // took ownership of s
    // println!("{s}"); // wont work

    // mut String
    let mut s = String::from("rust");
    s += "!";

    // &String
    let s = String::from("rust");
    borrow_string(&s); // borrows value 
    println!("{s}"); // works

    // &String can be coerced into &str
    borrow_str(&s);
    println!("{s}"); // works

    // &mut String
    let mut s = String::from("rust");
    let s1: &mut String = &mut s;
    mut_string(s1);
    println!("&mut String: {s}");

    // str - string slice
    // - Dynamically sized type / unsized type
    // - Size of the type not known at compile time
    // let a: str = "hello";
    // let b: str = "hello rust";

    // &str
    // - size known at compile time (pointer)
    // - immutable borrow
    let s: &str = "hello";
    borrow_str(s);
    println!("{s}");

    // &mut str
    let mut s = String::from("hello");
    let r: &mut str = &mut s;
}
