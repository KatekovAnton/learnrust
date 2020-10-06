use std::io;


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

impl Rectangle {
    fn area(&self) -> f32 {
        let Rectangle {top_left: Point {x: x1, y: y1}, bottom_right: Point {x: x2, y: y2}} = *self;
        let length = x2 - x1;
        let height = y1 - y2;
        (length * height * length * height).sqrt()
    }
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

mod aaa {
    pub struct TestObject {
        pub a: i32
    }

    impl TestObject {
        pub fn new(a: i32)->TestObject {
            TestObject {
                a: a
            }
        }

        pub fn new_simple()->TestObject {
            TestObject {
                a: 0
            }
        }
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    println!("return full string?");
    &s[..]
}

mod human;
use rand::Rng;
use std::cmp::Ordering;
use std::panic;
use std::fmt::Error;
use std::num::ParseIntError;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn testFunction(i: &u32) -> Result<u32, String> {
    let result = *i;
    return Ok(result)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    return Ok(s)
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if *item > *largest {
            largest = item;
        }
    }

    largest
}


fn main() {
    {
        let a = vec![1, 3, 5, 100, 20];
        let b = largest(&a);
        println!("b is {}", *b);
    }
    // {
    //     let a = analyzer::analyzer::get_constant();
    //     println!("Value: {}", a);
    // }
    // {
    //     use std::collections::HashMap;
    //
    //     let text = "hello world wonderful world";
    //
    //     let mut map = HashMap::new();
    //
    //     for word in text.split_whitespace() {
    //         let count = map.entry(word).or_insert(0);
    //         *count += 1;
    //     }
    //
    //     println!("{:?}", map);
    //
    // }

    {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            },
        };
    }
    // {
    //     let a: u32 = 10;
    //     match testFunction(&a) {
    //         Ok(b) => {
    //             println!("b is {}", b);
    //         }
    //         Err(e) => {
    //             println!("e is {}", e);
    //         }
    //     }
    // }

    // {
    //     let hello = "Здравствуйте";
    //     let helloStr = hello.to_string();
    //     // let answer = &hello[0];
    //
    //     let s = &hello[0..4];
    //     let v = &hello[0..4];
    //
    //     println!("s is \"{}\", hello len is {}", s, hello.len());
    //     println!("v is \"{}\", hello str len is {}", s, helloStr.chars().count());
    //
    //     let ss: String = helloStr.chars().take(4).collect();
    //     println!("ss is \"{}\", ss len is {}", ss, ss.chars().count());
    //
    //     let ss1: String = helloStr.chars().skip(1).take(4).collect();
    //     println!("ss1 is \"{}\", ss1 len is {}", ss1, ss1.chars().count());
    //
    // }

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        // scores.insert(String::from("Blue"), 30);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        let a = scores.get(&team_name);
        match a {
            None => {}
            Some(v) => {
                println!("found {}", v);
            }
        }
    }
    // {
    //     print!("\x1B[2J\x1B[1;1H");
    //     let mut a = 0;
    //     println!("line 1 {}", a);
    //     print!("\x1B[2J\x1B[1;1H");
    //     a = a + 1;
    //     println!("line 1 {}", a)
    // }
    {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {}", result);
        println!("The counter is {}", counter);
    }

    // {
    //     let max = 10;
    //     for number in (1..max) {
    //         println!("{}!", number);
    //     }
    //     println!("LIFTOFF!!!");
    // }

    // {
    //     let mut s = String::from("hello world");
    //
    //     let word = first_word(&s);
    //
    //     s.clear(); // error!
    //
    //     println!("the first word is: {}", word);
    // }
    {
        let a = "1ва23 4";
        let astring = String::from(a);
        let b = first_word(&astring);
        println!("Result1 : {}", b);

        let mut s = String::from("test String");
        let word = first_word(&s);
        s.clear();

        let a1 = 0;
        let a2 = a1;
        // println!("the first word is: {}", word);
    }

    {
        let t1 = (String::from("test"), 10);
        // let t1 = (20, 10);
        let t2 = t1.clone();
        let v1 = t1.0;
        let v2 = t1.1;
        let a: usize = 0;
    }
    {
        let x: i8 = 5;
        let y: Option<i8> = None;

        // match y {
        //     Some(v) => {
        //         let sum = x + v;
        //         println!("Sum is {}", sum);
        //     },
        //     None => {}
        // }

        if let Some(v) = y {
            let sum = x + v;
            println!("Sum is {}", sum);
        }
    }

    let h = human::Human { name: String::from("Anton"), age: 34 };
    let mut a1: aaa::TestObject = aaa::TestObject::new(1);
    let mut a2: aaa::TestObject = aaa::TestObject::new_simple();

    let mut a: i32 = 1;
    let y = &mut a;
    *y = 10;


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
    let Rectangle {
        top_left: Point { x: ref x1, y: ref y1 },
        bottom_right: Point { x: ref x2, y: ref y2 }
    } = rect;
    // *x1 = 1.0; // wtf
    println!("rect area: {}", rect_area(&rect));
    println!("rect area: {}", rect.area());

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number is: {}", secret_number);

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("result string {}", s);
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];
        v.push(3);
        println!("length is {}", v.len());
        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }

    }

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Ohh!!");

        // let mut guessParsed: u32 = 0;
        // let fff: Result<u32, ParseIntError> = guess.trim().parse::<u32>();
        // match fff {
        //     Ok(value) => {
        //         guessParsed = value;
        //         println!("Found value: {}", value)
        //         return;
        //     }
        //     Err(error) => {
        //         println!("Found error {}", error)
        //     }
        // }


        // let mut guessParsed: u32 = 0;
        // let result = panic::catch_unwind(|| {
        //     let fff = guess.trim().parse().expect("Please type a number!");
        //     guessParsed = fff
        // });

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };


        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}