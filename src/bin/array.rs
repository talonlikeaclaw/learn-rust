#![allow(unused)]

// Compound data types
// - tuple
// - array

fn main() {
    // Array - fixed length, known at compile time
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[1] = {}", arr[1]);

    let mut arr: [u32; 3] = [1, 2, 3];
    arr[1] = 9;

    // Create array of size 10 with all elements 0
    let arr: [u32; 10] = [0; 10];

    // Slice - length not known at compile time
    let nums: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];

    // First 3 elements
    let s = &nums[0..3]; // 0, 1, 2
    println!("first 3 elements: {:?}", s);

    // Middle 4 elements
    let s = &nums[3..7];
    println!("middle 4 elements: {:?}", s);

    // Last 3 elements
    let s = &nums[7..10]; // last index optional
    println!("last 3 elements: {:?}", s);

    // All elements
    let s = &nums[..];
    println!("all elements {:?}", s);
}
