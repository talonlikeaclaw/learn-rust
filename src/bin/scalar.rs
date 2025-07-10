#![allow(unused)]

use std::{char, i32};

fn main() {
    // Scalar types
    // - single value
    // - building blocks for more complex types

    // Integers
    //   Signed integers
    let i0: i8 = 0; // -128 ~ 127
    let i1: i16 = 1; // -32768 ~ 32767
    let i2: i32 = 2; // -2147483648 ~ 2147483647
    let i3: i64 = 3; // -9223372036854775808 ~ 9223372036854775807
    let i4: i128 = 4; // -170141183460469231731687303715884105728 ~ 170141183460469231731687303715884105727
    let i5: isize = 5; // defaults to system architecture size (32/64 bit)

    //   Unsigned integers
    let u0: u8 = 0; // 0 ~ 255
    let u1: u16 = 1; // 0 ~ 65535
    let u2: u32 = 2; // 0 ~ 4294967295
    let u3: u64 = 3; // 0 ~ 18446744073709551615
    let u4: u128 = 4; // 0 ~ 340282366920938463463374607431768211455
    let u5: usize = 5; // defaults to system architecture size (32/64 bit)

    // Floats
    let f0: f32 = 0.01;
    let f1: f64 = 0.02;

    // Boolean
    let b: bool = true;

    // Characters
    let c: char = 'c';

    // Type conversion
    let i: i32 = 1;
    let u: u32 = i as u32;
    let x: u32 = u + (i as u32);

    // Min and max
    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;

    println!("i32 min: {min_i}");
    println!("i32 max: {max_i}");

    let min_char: char = char::MIN;
    let max_char: char = char::MAX;

    println!("char min: {min_char}");
    println!("char max: {max_char}");

    // Overflow
    let mut u: u32 = u32::MAX;
    u += 1; // Causes u32 to overflow and a compiler panic
    println!("overflow u32: {u}");
    // Doesn't cause panic when using --release tag

    // Use checked_add - returns Some(x) | None
    let u = u32::checked_add(u32::MAX, 1);
    println!("checked_add: {:?}", u);
    let u = u32::checked_add(1, 1);
    println!("checked_add: {:?}", u);

    // wrapping_add - wraps around boundary
    let u = u32::wrapping_add(u32::MAX, 1);
    println!("wrapping_add: {:?}", u);
    let u = u32::wrapping_add(1, 1);
    println!("wrapping_add: {:?}", u);
}
