#![allow(unused)]

// Trait input and output

use std::ops::RangeFull;

trait Animal {
    fn speak(&self) -> String;
}

struct Cat;
struct Dog;

impl Animal for Cat {
    fn speak(&self) -> String {
        "meow".to_string()
    }
}

impl Animal for Dog {
    fn speak(&self) -> String {
        "woof".to_string()
    }
}

// Static = known at compile time
fn greet(animal: &impl Animal) {
    println!("static: {}", animal.speak());
}

fn return_concrete_type() -> impl Animal {
    Dog
}

// Dynamic = known at runtime
fn greet_dyn(animal: &dyn Animal) {
    println!("dynamic: {}", animal.speak());
}

// use box to store on the heap
fn rand_animal(rand: u32) -> Box<dyn Animal> {
    if rand <= 10 {
        Box::new(Dog)
    } else {
        Box::new(Cat)
    }
}

fn main() {
    let cat = Cat;
    let dog = Dog;

    greet(&cat);
    greet(&dog);

    let animal = return_concrete_type();
    println!("animal.speak: {}", animal.speak());

    // compiler won't be known at compile time
    let animal_str = "dog";
    let animal: &dyn Animal = match animal_str {
        "dog" => &Dog,
        _ => &Cat,
    };

    // won't work because not known at compile time
    // greet(animal);
    greet_dyn(animal);

    let animal = rand_animal(11);
    println!("rand animal: {}", animal.speak());
}
