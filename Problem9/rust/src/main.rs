use std::time::{Instant};

fn main() {
    let timer: Instant = Instant::now();

    for c in 335..997 as i64 {
        let c_squared: i64 = c.pow(2);
        for a in 2..333 as i64 {
            if a.pow(2) + (1000 - c - a).pow(2) == c_squared {
                println!("{}", a * (1000 - c - a) * c);

                let microseconds: u128 = timer.elapsed().as_micros();
                println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));

                return;
            }
        }
    }
}
