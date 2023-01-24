use std::time::{Instant};

fn main() {
    let timer: Instant = Instant::now();

    let mut prime_count: u64 = 1;

    let mut i: u64 = 1;
    'outer: loop {
        i += 2;

        for j in (3..=((i as f64).sqrt().floor() as u64)).step_by(2) {
            if i % j == 0 {
                continue 'outer;
            }
        }
        prime_count += 1;

        if prime_count == 10001 {
            println!("{}", i);

            let microseconds: u128 = timer.elapsed().as_micros();
            println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));

            return;
        }
    }
}