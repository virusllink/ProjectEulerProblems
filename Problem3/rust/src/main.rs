use std::time::{Instant};

fn main() {
    let timer: Instant = Instant::now();

    let mut num: u64 = 600851475143;
    let mut largest_prime_factor: u64 = 1;

    while num % 2 == 0 {
        largest_prime_factor = 2;
        num /= 2;
    }

    for i in (3..((num as f64).sqrt().floor() as u64)).step_by(2) {
        while num % i == 0 {
            largest_prime_factor = i;
            num /= i;
        }
    }

    println!("{}", largest_prime_factor);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}
