use std::time::{Instant};
use std::fs;

fn main() {
    let timer: Instant = Instant::now();

    //Read the number from the values.txt file
    let binding: String = fs::read_to_string("values.txt").expect("");
    let contents: String = binding.lines().collect();

    //Create a variable to store the largest factor
    let mut largest_factor: u64 = 0;

    //Create a loop to iterate through the string, stopping 13 characters before the end
    for i in 0..contents.chars().count()-12 {
        //Split the contents string to only contain 13 characters from the i-th character
        let num: String = contents.split_at(i).1.split_at(13).0.to_string();

        //Create a variable to store the current factors of the numbers in num
        let mut current_factor: u64 = 1;
        //Calculate the factors
        for j in num.chars() {
            current_factor *= j.to_digit(10).unwrap() as u64;
        }
        //If the calculated factor is larger than the largest stored factor, then replace the stored current factor with the calculated factor
        if current_factor > largest_factor {
            largest_factor = current_factor;
        }
    }

    //Print the final answer
    println!("{largest_factor}");

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}
