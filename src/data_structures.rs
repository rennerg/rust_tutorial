#![allow(dead_code)]

use std::mem;

struct Line {
    start: Point,
    end: Point
}

struct Point {
    x: f64,
    y: f64
}

enum Color{
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8), // tuple
    Cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8} // struct
}

pub fn structures()
{
    let p = Point {x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);
}

pub fn enums()
{
    let c:Color = Color::Red;

    match c
    {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Rgb(0,0,0)
        | Color::Cmyk{cyan:_, magenta:_, yellow:_, black:255} => println!("black"),
        Color::Rgb(r,g,b) => println!("rgb({},{},{})", r, g, b),
        _ => println!("something else")
    }
}

pub fn option()
{
    // Option<T>
    let x = 3.0;
    let y = 2.0;
    let result:Option<f64> = if y != 0.0 { Some(x/y) } else { None };

    println!("{:?}", result);

    match result
    {
        Some(t) => println!("{}/{} = {}", x, y, t),
        None => println!("cannot divide {} by {}", x, y)
    }

    // if let / while let
    if let Some(z) = result { println!("z = {}", z); } // if destructuring is Some(z)
}

pub fn arrays()
{
    let mut a:[i32;5] = [1,2,3,4,5];
    let mut b = [1,2,3,4,5];

    println!("a has {} elements, first is {}", a.len(), a[0]);

    if a == [1,2,3,4,5]
    {
        println!("{:?}", a);
    }

    let d = [1; 10];
    for i in 0..d.len() 
    {
        println!("{}", d[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&d));

    let matrix:[[f32;3];2] = 
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", matrix);

    for i in 0..matrix.len()
    {
        for j in 0..matrix[i].len()
        {
            println!("{}", matrix[i][j]);
        }
    }
}

pub fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(1);
    a.push(1);

    println!("a = {:?}", a);

    a.push(1);

    println!("a = {:?}", a);

    println!("a = {:?}", a);

    let indx:usize = 0;

    println!("a = {:?}", a[indx]);

    // indexing into vector
    match a.get(6) // Returns Option
    {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }

    for x in &a { println!("{}", x); }

    a.push(77);
    let last_elem = a.pop(); // Option
    println!("last element is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop() // fails when a.pop returns None and breaks out of loop
    {
        println!("{}", x);
    }
}

pub fn slices()
{
    let mut data = [1,2,4,5,6,7];

    use_slice(&mut data[1..4]);
    use_slice(&mut data);

    println!("{:?}", data);
}

fn use_slice(slice: &mut[i32])
{
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

pub fn strings()
{
    // 2 different string types
    // static string
    let s:&'static str = "Hello there!"; // &str = string slice
    // s = "abc"; --> not allowed
    // let h = s[0]; --> also not allowed

    for c in s.chars().rev()
    {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0)
    {
        println!("first letter is {}", first_char);
    }

    // heap allocated
    // String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    // &str <> String
    let u:&str = &letters;

    // concatenation
    // String + str
    let z = letters + "abc";

    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));
}

pub fn tuples()
{
    let x = 43;
    let y = 4;
    let sp = sum_and_product(x,y);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    //destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("combined = {:?}", combined);
    println!("last element = {}", (combined.1).1);

    let tuple = (true, 42.0, -1i8);

    let single_elem_tuple = (42,);
}

fn sum_and_product(x:i32, y:i32) -> (i32, i32)
{
    return (x+y, x*y);
}

pub fn generics()
{
    let a:GenericPoint<i32> = GenericPoint {x: 0, y: 0};
    let b:GenericPoint<f64> = GenericPoint {x: 1.2, y: 3.4};
}

struct GenericPoint<T>
{
    x: T,
    y: T
}