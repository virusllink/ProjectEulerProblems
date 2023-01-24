use std::{time::{Instant}};
use num::BigUint;

fn main() {
    let timer: Instant = Instant::now();

    let hundred_factorial: BigUint = (1 as usize ..=100 as usize).product();
    println!("{}", hundred_factorial.to_string().chars().map(|x| x.to_digit(10).unwrap()).sum::<u32>());

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}