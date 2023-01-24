use std::time::{Instant};

fn main() {
    let timer: Instant = Instant::now();

    let mut sum_of_squares: u64 = 1;
    for i in 2..101 {
        sum_of_squares += (i as u64).pow(2);
    }

    let mut square_of_sums: u64 = 0;
    for i in 1..101 {
        square_of_sums += i;
    }
    square_of_sums = square_of_sums.pow(2);

    println!("{}", square_of_sums - sum_of_squares);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}