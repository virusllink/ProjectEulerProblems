use std::{time::{Instant}};
use std::fs;

fn main() {
    let timer: Instant = Instant::now();

    let binding: Vec<u8> = fs::read("values.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = std::str::from_utf8(&binding).unwrap().split("\n").collect();
    let mut triangle: Vec<Vec<u32>> = lines.into_iter().map(|x| x.split_whitespace().collect::<Vec<&str>>().into_iter().map(|y| y.to_string().parse::<u32>().unwrap()).collect()).collect::<Vec<Vec<u32>>>();

    for y in (0..triangle.len() - 1).rev() {
        for x in 0..triangle[y].len() {
            triangle[y][x] = if triangle[y+1][x+1] < triangle[y+1][x] {triangle[y][x] + triangle[y+1][x]} else { triangle[y][x] + triangle[y+1][x+1]};
        }
    }

    println!("{}", triangle[0][0]);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}