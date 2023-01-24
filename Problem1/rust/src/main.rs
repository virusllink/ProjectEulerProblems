use std::time::{Instant};

fn main() {
    let timer: Instant = Instant::now();

    let mut sum: u32 = 0;

    for i in 3..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    println!("{}", sum);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}