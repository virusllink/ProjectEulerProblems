use std::{time::{Instant}};
use std::fs;

fn main() {
    let timer: Instant = Instant::now();

    let binding: String = fs::read_to_string("names.txt").expect("Should have been able to read file");
    let mut content: Vec<String> = binding.split("\",\"").into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
    content.sort();

    let mut answer: u64 = 0;

    for i in 0..content.len() {
        answer += content[i].chars().map(|x| (x as u8 - 64).to_string().parse::<u64>().unwrap()).sum::<u64>() * (i + 1) as u64;
    }

    println!("{}", answer);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}
