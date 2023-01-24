use std::time::{Instant};

fn main() {
    let timer: Instant = Instant::now();

    let mut i: u32 = 0;
    loop {
        i += 20;
        for j in 3..21 {
            if i % j != 0 {
                break;
            } else if j == 20 {
                println!("{}", i);
                
                let microseconds: u128 = timer.elapsed().as_micros();
                println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
                return;
            }
        }
    }
}
