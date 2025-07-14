// use super twice to go up twice
// a > my > out > foo
use super::super::foo;
#[derive(Debug)]
pub struct S {
    pub id: u32, // must be public
    pub name: String,
}
pub fn call_foo() {
    foo::print();
}
pub fn print() {
    println!("a");
}
