use std::{time::{Instant}, vec};

fn main() {
    _solution_2(2000000);
}

//Solution 1 is entirely my solution
fn _solution_1() {
    let timer: Instant = Instant::now();

    let mut sum_of_primes: u64 = 5;
    //Create a loop between 3 and 2 million
    'outer: for i in (5..2000000 as u64).step_by(2) {
        let upper_limit: u64 = (i as f64).sqrt().ceil() as u64;
        //Loop to check if i is a prime or not
        for j in (3..=upper_limit).step_by(2) {
            if i % j != 0 {
                //If j has reached the upper limit, i is a prime
                if j == upper_limit || j == upper_limit - 1 {
                    sum_of_primes += i;
                }
            } else {
                //Continue the outer loop if i is not a prime
                continue 'outer;
            }
        }
    }

    println!("{}", sum_of_primes);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}

//Solution 2 was an attempt to implement the steps from the problem 10 overview from projecteuler.net
fn _solution_2(n: u64) {
    let timer: Instant = Instant::now();

    //1. Make a list of all numbers from 2 to N
    //Vector starts at 2 and ends at n - 1
    let mut numbers: Vec<bool> = vec![true; n as usize - 1];

    //2. Find the next number p not yet crossed out. This is a prime. ...
    let sqrt_of_n: u64 = (n as f64).sqrt() as u64;
    for p in 2..numbers.len() as u64 {
        //... If it is greater than √N, go to 5
        if p > sqrt_of_n {
            break;
        }

        //3. Cross out all multiples of p which are not yet crossed out.
        for multiples_of_p in (p+p..n).step_by(p as usize) {
            numbers[multiples_of_p as usize - 2] = false;
        }

        //4. Go to 2.
    }

    //5. The numbers not crossed out are the primes not exceeding N.
    let mut sum_of_primes: u64 = 0;

    for i in 0..numbers.len() as u64 {
        if numbers[i as usize] == true {
            sum_of_primes += i + 2;
        }
    }

    println!("{}", sum_of_primes);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}