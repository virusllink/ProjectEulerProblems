use std::time::{Instant};

fn main() {
    let timer: Instant = Instant::now();

    let mut palindrome: u32 = 0;

    for i in (100..1000).rev() {
        for j in (100..1000).rev() {
            let sum: u32 = i * j;
            let split_string1: String = sum.to_string().split_at(3).0.to_string();
            let split_string2: String = sum.to_string().split_at(3).1.to_string().chars().rev().collect();

            if split_string1 == split_string2 && palindrome < sum {
                palindrome = sum;
            }
        }
    }

    println!("{}", palindrome);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}