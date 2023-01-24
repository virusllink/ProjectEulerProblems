use std::time::{Instant};

fn main() {
    let timer: Instant = Instant::now();

    let mut answer: u64 = 1;
    let mut longest_sequence: u64 = 0;

    for number in 2..1000000 as u64 {
        let mut current_sequence_count: u64 = 0;
        let mut current_sequence_number: u64 = number;
        loop {
            if current_sequence_number % 2 == 0 {
                current_sequence_count += 1;
                current_sequence_number /= 2;
            } else {
                current_sequence_count += 2;
                current_sequence_number = (3 * current_sequence_number + 1) / 2;
            }

            if current_sequence_number == 1 {
                break;
            }
        }

        if current_sequence_count > longest_sequence {
            longest_sequence = current_sequence_count;
            answer = number;
        }
    }

    println!("{}", answer);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}