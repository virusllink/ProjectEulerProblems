use std::{time::{Instant}};

fn main() {
    let timer: Instant = Instant::now();

    let mut sum_of_amicable_number: u64 = 0;

    for number in 3..10000 as u32 {
        //Get the divisors of number
        let proper_divisors_sum1: u32 = find_divisors(number).into_iter().sum();
        //Get divisors of proper_divisor_sum
        let proper_divisors_sum2: u32 = find_divisors(proper_divisors_sum1).into_iter().sum();

        if number == proper_divisors_sum2 && proper_divisors_sum1 != proper_divisors_sum2 {
            sum_of_amicable_number += proper_divisors_sum1 as u64;
        }
    }

    println!("{}", sum_of_amicable_number);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}

fn find_divisors(number: u32) -> Vec<u32> {
    let mut divisors: Vec<u32> = Vec::new();
    for i in 1..(number as f64).sqrt() as u32 {
        if number % i == 0 {
            divisors.push(i);
            if i != number / i {
                divisors.push(number / i);
            }
        }
    }
    divisors.sort();
    if divisors.len() != 0 && divisors[divisors.len()-1] == number {
        divisors.remove(divisors.len()-1);
    }
    return divisors;
}