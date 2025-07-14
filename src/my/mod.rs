use super::foo;
pub fn call_foo() {
    foo::print();
}
// make public
pub fn print() {
    f(); // can call private with same module
    println!("my");
}

fn f() {}

pub mod a;
