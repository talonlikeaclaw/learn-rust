#![allow(unused)]

// Match
// Will provide compiler error for missing cases

#[derive(Debug)]
enum Animal {
    Cat,
    Dog,
    Duck,
    Mouse,
}

fn main() {
    // Using traditional conditional
    let x = 1;
    if x == 1 {
        println!("one");
    } else if x == 2 {
        println!("two");
    } else if x == 3 {
        println!("three");
    } else {
        println!("other");
    }

    // Using match
    let y = 2;
    match y {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"), // catch all case
    }

    // multiple cases
    let x = 4;
    match x {
        1 | 2 | 3 => println!("1 or 2 or 3"),
        _ => println!("other"),
    }

    // using range
    match x {
        1..10 => println!("between 1 and 10"),
        _ => println!("other"),
    }

    // @ to get number that matched
    match x {
        i @ 1..10 => println!("matched {i}"),
        _ => println!("other"),
    }

    // return values
    let animal = Animal::Cat;
    let animal_sound = match animal {
        Animal::Cat => "meow",
        Animal::Duck => "quack",
        Animal::Mouse => "squeak",
        Animal::Dog => "woof",
        _ => "?",
    };
    println!("animal sound {animal_sound}");

    // Option
    let x: Option<i32> = Some(1);
    match x {
        Some(v) => println!("Some {v}"),
        None => println!("none"),
    }

    // Result
    let res: Result<u32, String> = Ok(10);
    match res {
        Ok(val) => println!("Ok {val}"),
        Err(msg) => println!("Err {msg}"),
    }
}
