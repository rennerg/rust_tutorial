#![allow(dead_code)]

use std::mem;

struct Point 
{
    x: f64,
    y: f64
}

fn origin() -> Point
{
    Point{x: 0.0, y: 0.0}
}

pub fn stackNheap()
{
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("p3 is {}", p3.x);
}

pub fn stack_and_heap()
{
    let y = Box::new(10); // puts value 10 on Heap, puts y variable on Stack (y points to Heap address)
    println!("*y = {}", *y);
    println!("y = {}", y);
}