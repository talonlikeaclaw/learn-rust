#![allow(unused)]

use learn_rust::my;
use learn_rust::my::a;

// Modules
// Organize code

fn main() {
    my::print();
    my::a::print();
    let s = my::a::S {
        id: 1,
        name: "S".to_string(),
    };
    println!("{:#?}", s);
    my::call_foo();
    my::a::call_foo();
}
