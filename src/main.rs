use std::mem;
mod stack_and_heap;
mod data_structures;
mod pattern_matching;
mod functions;

const MEANING_OF_LIFE:i8 = 42;
static mut NOT_VERY_SAFE:i32 = 123;

fn main() {
    fundamental_data_types();

    operators();

    scopes_and_shadowing();

    stack_and_heap::stackNheap();

    stack_and_heap::stack_and_heap();

    data_structures::structures();

    data_structures::enums();

    data_structures::option();

    data_structures::arrays();

    data_structures::slices();

    data_structures::strings();

    data_structures::tuples();

    pattern_matching::pattern_matching();

    functions::functions();

    functions::closures();

    functions::higher_order_functions();

    functions::traits();
}

fn fundamental_data_types()
{
    // println!("Hello, world!");
    let a:u8 = 123; // 8 bits
    println!("a = {}", a);

    let mut b:i8 = 0;
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 12345; // 32-bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit os",
            z, size_of_z, size_of_z * 8);

    let d:char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}

fn operators()
{
    let mut a = 2 + 3 * 4;
    println!("{}", a);
    a += 1;
    a %= 2;
    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
}

fn scopes_and_shadowing()
{
    let a = 123;

    {
        let b = 345;
        println!("{}", b);
        let a = 77;
        println!("{}", a);
    }
    println!("{}", a);
}

fn global_var_access()
{
    println!("{}", MEANING_OF_LIFE);

    unsafe
    {
        println!("{}", NOT_VERY_SAFE);
    }
}

fn control_flow()
{
    let temp = 35;
    if temp > 30 && true || false
    {
        println!("hot today...hot yesterday");
    }
    else if temp < 10
    {
        println!("cold out...");
    }
    else
    {
        println!("pass the moonshine");
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);
}

fn while_and_loop()
{
    let mut x = 1;
    while x < 1000
    {
        x *= 2;

        if x == 64 {continue;}

        println!("x = {}", x);
    }

    let mut y = 1;
    loop
    {
        y *= 2;
        println!("y is {}", y);
        if y == 1 << 10 {break;}
    }

    for x in 1..11
    {
        println!("x is {}", x);
    }

    for (index,y) in (30..41).enumerate()
    {
        println!("{}: {}", index, y);
    }
}

fn match_statement()
{
    let country_code = 7;
    let country = match country_code
    {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1...999 => "Unknown", // "..." includes the last value, 999
        _ => "Invalid"
    };

    println!("the country with code {} is {}", country_code, country);
}

