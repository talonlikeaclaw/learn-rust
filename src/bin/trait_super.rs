#![allow(unused)]

trait Language {
    fn name(&self) -> String;
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

// Anything that implements this trait must implement the others
// kind of like inheritance!
trait CompiledLanguage: Language + Compiler {
    fn execute(&self, file_path: &str) {
        // can call sub trait functions
        let name = self.name();
        println!("name: {name}");

        let cmd = self.compile(file_path);
        println!("cmd: {cmd}");
    }
}

// Create struct
struct Rust;

// Implement first trait
impl Language for Rust {
    fn name(&self) -> String {
        "rust".to_string()
    }
}

// Implement second trait
impl Compiler for Rust {
    fn compile(&self, file_path: &str) -> String {
        format!("cargo build {file_path}")
    }
}

// Now we can implement super trait
impl CompiledLanguage for Rust {}

fn main() {
    let rust = Rust;

    rust.execute("hello.rs"); // neat!
}
