extern crate rand;
use rand::Rng;

pub fn consuming_crates()
{
    let mut rng = rand::thread_rng();
    let b:f64 = rng.gen();
    println!("random num is {}", b);
}