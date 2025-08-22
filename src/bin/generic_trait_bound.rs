#![allow(unused)]

// Generic types must implement the debug trait to be printed.
fn f<T: std::fmt::Debug>(t: T) {
    println!("{:?}", t);
}

// Trait bound - specifies constraints on a generic type

trait A {}
trait B {}
trait C {}

impl A for u32 {}
impl B for u32 {}
impl C for u32 {}
impl A for i32 {}

fn c<T: A>(x: T) {} // T must implement A
fn m<T: A + B>(x: T) {} // T must implement A and B
                        // T must implement A and B, U must impement B and C
fn w<T, U>(x: T, y: U)
where
    T: A + B,
    U: B + C,
{
}

// Difference between impl trait syntax and trait bounds
// x and y can be different types
fn k(x: impl A, y: impl A) {}
// x and y must be the same type
fn g<T: A>(x: T, y: T) {}
fn h<T: A, U: A>(x: T, y: U) {}

fn main() {
    let u: u32 = 1;
    let i: i32 = -1;
    let f: f32 = 1.0;

    c(u);
    c(i);
    // c(f); won't compile because it doesn't implement A

    m(u);
    // m(i); won't compile because it doesnt implement B

    w(u, u);
    // w(i, f); won't compile because i doesn't implement B and f B and C

    k(u, i);
    g(u, u);
    // g(u,i); won't compile, not same data types
    h(u, i);
}
