#![allow(unused)]

// Declare a Struct
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Point3d(f32, f32, f32);

struct Empty;

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}

fn main() {
    // Create
    let p = Point { x: 1.0, y: 2.0 };
    println!("point.x = {}, point.y = {}", p.x, p.y);

    let p3d = Point3d(1.0, 2.0, 3.0);
    println!("point3d = {}, {}, {}", p3d.0, p3d.1, p3d.2);

    let empty = Empty;

    let circle = Circle {
        center: Point { x: 1.0, y: 2.0 },
        radius: 1,
    };

    // Debug
    // Read
    println!("{:#?}", circle);

    // Shortcuts
    let x = 1.0;
    let y = 2.0;
    // Can omit variable name on instantiation
    let p = Point { x, y };

    // Copy fields
    let p0 = Point { x: 1.0, y: 1.0 };
    // Fill remaining fields with p0
    let p1 = Point { x: 2.0, ..p0 };
    println!("{:#?}", p1);

    // Update
    let mut p = Point { x: 0.0, y: 0.0 };
    p.x += 1.0;
    p.y -= 1.0;
    println!("{:#?}", p);
}
