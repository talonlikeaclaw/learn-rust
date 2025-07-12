#![allow(unused)]

fn main() {
    // loop
    let mut i = 0;
    loop {
        println!("loop: {i}");
        if i == 5 {
            break;
        }
        i += 1;
    }

    // while
    let mut j = 0;
    while j <= 3 {
        println!("while: {j}");
        j += 1;
    }

    // for loop
    for l in 0..5 {
        println!("for loop: {l}");
    }

    // for loop array
    let arr = [1, 2, 3];
    for a in arr {
        println!("array: {a}");
    }

    // usize and range
    let n: usize = arr.len();
    for i in 0..n {
        println!("array: {}", arr[i]);
    }

    // for loop vector
    let v = vec![1, 2, 3];

    for x in v {
        println!("vector: {x}");
    }

    // Second time won't work due to ownership
    // for x in v {
    //     println!("vector: {x}");
    // }

    // Use iterator instead
    let v2 = vec![1, 2, 3];
    for x in v2.iter() {
        println!("vector2 {x}");
    }

    // Return value - only loop
    let mut i = 0;
    let z = loop {
        if i == 3 {
            break 99;
        }
        i += 1;
    };
    println!("return loop {z}");

    // Labels - for nested loops
    'outer: for i in 0..5 {
        'inner: for j in 0..5 {
            println!("{i}, {j}");
            if i == 1 && j == 2 {
                break 'outer;
            }
        }
    }
}
