#![allow(unused)]

// Add Debug attribute
#[derive(Debug)]
struct Lang {
    language: String,
    version: String,
}

fn main() {
    // Printing variables
    let lang = "rust";
    println!("hello {}", lang);
    println!("hello {} {}", lang, lang);
    println!("hello {lang}");

    // Positional arguments for printing
    let x = 2;
    println!("{0} x {0} = {1}", x, x * x);

    // Printing structs with Debug
    let lang = Lang {
        language: "rust".to_string(),
        version: "1.88".to_string(),
    };

    println!("{:?}", lang); // Without linebreaks
    println!("{:#?}", lang); // With linebreaks
}
