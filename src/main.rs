#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Copy, Clone)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    // this can work, but doesn't seem nested
    let Point { x: x1, y: y1 } = rect.top_left;
    let Point { x: x2, y: y2 } = rect.bottom_right;

    // nested destructuring instead of above
    // syntax? how to do this?
    //let Rectangle { p1: Point { x: x1, y: y1 } = rect.p1,
    //                p2: Point { x: x2, y: y2 } = rect.p2 };

    // the problem with this is that without a copy method,
    // rect will be moved, so it will have to be pass by value
    // and the caller won't be able to continue using the rect
    //let Rectangle { p1: Point { x: x1, y: y1 },
    //                p2: Point { x: x2, y: y2 } } = rect;

    (x1 - x2) * (y1 - y2)
}

fn main() {
    let mut a: i32 = 0;
    a = 1;
    println!("Hello, world!");


    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    let mut x: &i32;
    x = &10;

    println!("{}", x);

    let p1: Point = Point { x: 3.0, y: 4.0 };
    let p2: Point = Point { x: 6.0, y: 10.0 };

    let rect: Rectangle = Rectangle { top_left: p1, bottom_right: p2 };
    let Rectangle { top_left: Point { x: ref x1, y: ref y1 },
                    bottom_right: Point { x: ref x2, y: ref y2 } } = rect;
    println!("{:?}", peter);
}
