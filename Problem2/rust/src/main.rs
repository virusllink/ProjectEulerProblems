use std::time::{Instant};

fn main() {
    let timer: Instant = Instant::now();

    let mut sum: u32 = 2;
    let mut prev_num: u32 = 1;
    let mut curr_num: u32 = 2;

    while curr_num < 4000000 {
        let sum_of_nums: u32 = curr_num + prev_num;
        if sum_of_nums % 2 == 0 {
            sum += sum_of_nums;
        }

        prev_num = curr_num;
        curr_num = sum_of_nums;
    }

    println!("{}", sum);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}
