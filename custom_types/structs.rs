// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use std::fmt::{self,Formatter,Display};

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Rectangle { 
            top_left: Point {
                x: x1,
                y: y1,
            },
            bottom_right: Point {
                x: x2,
                y: y2,
            }
        } = self;
        write!(f, "Width: {}, Height: {}", x2 - x1, y2 - y1)
    }
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { 
        top_left: Point {
            x: x1,
            y: y1,
        },
        bottom_right: Point {
            x: x2,
            y: y2,
        }
    } = rect;
    (x2 - x1) * (y2 - y1)
}

fn square(point: Point, num: f32) -> Rectangle {
    let Point { x: x1, y: y1 } = point;
    Rectangle {
        top_left: point,
        bottom_right: Point { x: x1 - num, y: y1 - num }
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let test_rectangle = Rectangle {
        top_left: Point { x: 2.0, y: 3.0 },
        bottom_right: Point { x: 5.0, y: 1.0 },
    };
    println!("Rectangle: {}", test_rectangle);
    println!("Rectangle area: {}", rect_area(test_rectangle));

    let test_point = Point {
        x: 22.3,
        y: 33.6,
    };
    println!("Square from point x: {}, y: {} and w+h: {} == {}", test_point.x, test_point.y, 5.0, square(point, 5.0));
}

