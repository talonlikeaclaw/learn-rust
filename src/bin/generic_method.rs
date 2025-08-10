#![allow(unused)]

// Create a generic typed struct
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Enable generic types for implementation block
impl<T> Point<T> {
    // Generic type constructor
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    // Generic type instance method
    fn move_to(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    // Creating a concrete typed instance
    let mut p: Point<u32> = Point::new(1, 2);
    p.move_to(2, 3);
    println!("{:?}", p);
}
