use std::time::{Instant};

fn main() {
    let timer: Instant = Instant::now();

    let mut triangle_number: u64 = 0;

    let mut i: u64 = 0;
    loop {
        i += 1;
        triangle_number += i;

        let sqrt_of_triangle_number: u64 = (triangle_number as f64).sqrt() as u64;
        let mut count: i32 = 0;
        for i in 1..sqrt_of_triangle_number {
            if triangle_number % i == 0 {
                count += 2;
            }
        }
        if triangle_number == sqrt_of_triangle_number * sqrt_of_triangle_number {
            count -= 1;
        }
        if count > 500 {
            println!("{}", triangle_number);

            let microseconds: u128 = timer.elapsed().as_micros();
            println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));

            return;
        }
    }
}
