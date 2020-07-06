pub fn functions()
{
    let mut z = 1;
    increase(&mut z);
    println!("z = {}", z);

    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("p = {}", p);

    let p1 = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 6.0 };
    let mylline = Line {start: p1, end: p2 };

    println!("length of line is {}", mylline.length())
}

fn increase(x: &mut i32)
{
    *x += 1;
}

fn product(x: i32, y: i32) -> i32
{
    x * y //NOTE the lack of a semicolon -> this tells the function to return
}

struct Point
{
    x: f64,
    y: f64
}

struct Line
{
    start: Point,
    end: Point
}

impl Line
{
    fn length(&self) -> f64
    {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

pub fn closures()
{
    let sh = say_hello;
    sh();

    let plus_one_func = |x:i32| -> i32 { x + 1 }; // Note no semicolon after x + 1
    let a = 6;
    println!("{} + 1 = {}", a, plus_one_func(a));

    let mut two = 2;
    { // put func in scope 
    let plus_two = |x|
    {
        let mut z = x;
        z += two;
        z // returns
    };
    println!("{} + 2 = {}", 3, plus_two(3));
    }

    let borrow_two = &mut two;

    // T: by value
    // T&
    // &mut 
    let plus_three = |x:&mut i32| *x +=3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);
}

fn say_hello() { println!("hello"); }

pub fn higher_order_functions()
{
    let limit = 500;
    let mut sum = 0;

    for i in 0..
    {
        let isq = i*i;

        if isq > limit {break;}
        else if is_even(isq) { sum += isq;}
    }

    println!("loop sum = {}", sum);

    let sum2 = (0..).map(|x| x*x)
                    .take_while(|&x| x < limit)
                    .filter(|x| is_even(*x))
                    .fold(0, |summ,x| summ+x);

    println!("sum2 = {}", sum2);
}

fn is_even (x: u32) -> bool { x % 2 == 0 }

pub fn traits()
{
    let h = Human {name:"John"};
    h.talk();

    let h2 = Human::create("John");
    h2.talk();

    let c = Cat {name:"Mittens"};
    c.talk();

    let c2 = Cat::create("Mittens");
    c2.talk();

    let a:Human = Animal::create("Sam"); // casting an Animal to Human
    a.talk();

    let v = vec![1,2,3];
    println!("sum = {}", v.sum());
}

trait Animal
{
    fn create(name: &'static str) -> Self; // static function

    fn name(&self) -> &'static str;

    fn talk(&self)
    {
        println!("{} cannot talk", self.name());
    }
}

struct Human 
{
    name: &'static str
}

struct Cat
{
    name: &'static str
}

impl Animal for Human
{
    fn create(name:&'static str) -> Human
    {
        Human {name: name}
    }
    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat 
{
    fn create(name:&'static str) -> Cat
    {
        Cat {name: name}
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says meow", self.name());
    }
}

trait Summable<T>
{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut result:i32 = 0;
        for x in self { result += *x; }
        return result
    }
}