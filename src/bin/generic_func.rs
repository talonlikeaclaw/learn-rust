#![allow(unused)]

// Syntax to enable generic types for functions
fn swap<A, B>(t: (A, B)) -> (B, A) {
    (t.1, t.0)
}

use std::cmp::PartialOrd;

// T must implement Partial Order to compare different types
fn max<T: PartialOrd>(s: &[T]) -> Option<&T> {
    if s.len() == 0 {
        return None;
    }

    let mut largest = &s[0];
    for item in s {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

fn main() {
    let t: (u32, u32) = (1, 2);
    let s = swap(t);
    println!("{:?}", s);

    let t: (i32, u32) = (-1, 2);
    let s = swap(t);
    println!("{:?}", s);

    // Function works with numbers
    let nums = vec![33, 1, 22, 54, 99, 10];
    let largest = max(&nums);
    println!("largest num: {:?}", largest);

    // Function works with chars
    let chars = vec!['a', 'c', 'y', 'i', 'm'];
    let largest = max(&chars);
    println!("largest char: {:?}", largest)
}
