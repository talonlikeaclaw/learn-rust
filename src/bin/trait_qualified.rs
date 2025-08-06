#![allow(unused)]

trait Color {
    fn get(&self) -> String;
}

trait Rectangle {
    fn get(&self) -> (i32, i32, u32, u32);
}

struct Square {
    color: String,
    top: i32,
    left: i32,
    size: u32,
}

// Square has two traits with the same name
impl Color for Square {
    fn get(&self) -> String {
        self.color.clone()
    }
}

impl Rectangle for Square {
    fn get(&self) -> (i32, i32, u32, u32) {
        (self.top, self.left, self.size, self.size)
    }
}

fn main() {
    // Trait - fully qualified syntax
    let square = Square {
        color: "red".to_string(),
        top: 0,
        left: 0,
        size: 10,
    };

    // Specify the trait to specify function with same name
    let color = Color::get(&square);
    let (x, y, width, height) = Rectangle::get(&square);

    println!("color: {color}");
    println!("x, y, width, height: {x}, {y}, {width}, {height}");
}
