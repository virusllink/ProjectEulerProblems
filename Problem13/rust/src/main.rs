use std::time::{Instant};
use std::fs;
use num::BigUint;

fn main() {
    let timer: Instant = Instant::now();

    let binding: String = fs::read_to_string("values.txt").expect("");
    let content: Vec<&str> = binding.lines().collect::<Vec<&str>>();

    let sum: BigUint = content.into_iter().map(|x| x.parse::<BigUint>().unwrap()).sum();

    println!("{}", sum.to_string().split_at(10).0);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}