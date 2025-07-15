#![allow(unused)]

#[derive(Debug)]
enum MathError {
    DivByZero,
}

#[derive(Debug)]
enum ParseError {
    InvalidInt,
}

// &self refers to the MathError
impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "math error {:?}", self)
    }
}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error {:?}", self)
    }
}
impl std::error::Error for MathError {}
impl std::error::Error for ParseError {}

fn f1() -> Result<u32, MathError> {
    Err(MathError::DivByZero)
}

fn f2() -> Result<u32, MathError> {
    Err(MathError::DivByZero)
}

// Math and Parse errors and converted to a trait Error
fn f3() -> Result<(), Box<dyn std::error::Error>> {
    f1()?;
    f2()?;
    Ok(())
}

fn main() {
    let z = f3();
    println!("z = {:?}", z);
}
