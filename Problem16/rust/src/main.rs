use std::time::{Instant};
use num::{BigUint, bigint::{ToBigUint}};

fn main() {
    let timer: Instant = Instant::now();

    println!("{}", BigUint::pow(&2.to_biguint().unwrap(), 1000).to_string().chars().map(|x| x.to_digit(10).unwrap()).sum::<u32>());

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}
