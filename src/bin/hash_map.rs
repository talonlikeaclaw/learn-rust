#![allow(unused)]

use std::collections::HashMap;

fn main() {
    // Initialize
    let mut scores: HashMap<String, u32> = HashMap::new();

    //Insert
    scores.insert("red".to_string(), 100);
    scores.insert("green".to_string(), 100);

    // Get
    let val: Option<&u32> = scores.get("green");
    println!("val: {:?}", val); // returns Some(100)

    let val: Option<&u32> = scores.get("blue");
    println!("val: {:?}", val); // returns None (doesn't exist)

    // Update value via key
    scores.insert("green".to_string(), 200);
    let val: Option<&u32> = scores.get("green");
    println!("val: {:?}", val); // returns Some(200)

    // Upsert
    let v: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *v += 200;

    let val: Option<&u32> = scores.get("blue");
    println!("val: {:?}", val); // returns Some(200)

    let v: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *v += 200;

    let val: Option<&u32> = scores.get("blue");
    println!("val: {:?}", val); // returns Some(400)
}
