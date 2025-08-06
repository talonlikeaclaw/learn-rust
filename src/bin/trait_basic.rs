#![allow(unused)]

struct Solidity {
    version: String,
}

struct Vyper {
    version: String,
}

// Define a trait
// Similar to an interface
trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

// Create a default implementation
trait Test {
    fn test(&self, file_path: &str) -> String {
        format!("test {}", file_path)
    }
}

// Use the trait on a struct
impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {file_path}")
    }
}

impl Test for Solidity {
    fn test(&self, file_path: &str) -> String {
        format!("forge {file_path}")
    }
}

// Now both structs implement the Compiler trait
impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vyper {file_path}")
    }
}

// Implement default implementation
impl Test for Vyper {}

// Function can take a reference to anything implementing Compiler trait
fn compile(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}

fn main() {
    let sol = Solidity {
        version: "0.8".to_string(),
    };
    let vy = Vyper {
        version: "0.4".to_string(),
    };

    println!("compile sol: {}", compile(&sol, "hello.sol"));
    println!("compile vy: {}", compile(&vy, "hello.vy"));
    println!("compile sol: {}", sol.test("hello.sol"));
    println!("compile vy: {}", vy.test("hello.vy"));
}
