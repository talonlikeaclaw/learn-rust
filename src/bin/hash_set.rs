#![allow(unused)]

use std::collections::HashSet;

fn main() {
    // HashSet<type> - Can only have one type
    // Unique values only
    let mut set: HashSet<u32> = HashSet::new();

    // Insert
    // insert returns boolean indicating if insert worked
    let inserted: bool = set.insert(1);
    println!("inserted: {inserted}"); // true

    let inserted: bool = set.insert(1);
    println!("inserted: {inserted}"); // false

    // Get
    let contains: bool = set.contains(&1);
    println!("contains 1?: {contains}"); // true

    let contains: bool = set.contains(&2);
    println!("contains 2?: {contains}"); // false

    let contains: bool = set.contains(&3);
    println!("contains 2?: {contains}"); // false
}
