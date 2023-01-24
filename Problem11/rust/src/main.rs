use std::time::{Instant};
use std::fs;

fn main() {
    let binding: Vec<u8> = fs::read("grid.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = std::str::from_utf8(&binding).unwrap().split("\n").collect();
    let mut grid: Vec<Vec<&str>> = Vec::new();

    for i in lines {
        grid.push(i.split_whitespace().collect::<Vec<&str>>());
    }

    let timer: Instant = Instant::now();

    let mut largest_product: u32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            println!("y: {}, x: {}", y, x);
            if x < grid[0].len()-3 {
                let horizontal_product: u32 = grid[y][x].parse::<u32>().unwrap() * grid[y][x+1].parse::<u32>().unwrap() * grid[y][x+2].parse::<u32>().unwrap() * grid[y][x+3].parse::<u32>().unwrap();
                if largest_product < horizontal_product { largest_product = horizontal_product; }
            }

            if x < grid[0].len()-3 && y < grid.len()-3 {
                let right_diagonal_product: u32 = grid[y][x].parse::<u32>().unwrap() * grid[y+1][x+1].parse::<u32>().unwrap() * grid[y+2][x+2].parse::<u32>().unwrap() * grid[y+3][x+3].parse::<u32>().unwrap();
                if largest_product < right_diagonal_product { largest_product = right_diagonal_product; }
            }

            if x > 2 && y < grid.len()-3 {
                let left_diagonal_product: u32 = grid[y][x].parse::<u32>().unwrap() * grid[y+1][x-1].parse::<u32>().unwrap() * grid[y+2][x-2].parse::<u32>().unwrap() * grid[y+3][x-3].parse::<u32>().unwrap();
                if largest_product < left_diagonal_product { largest_product = left_diagonal_product; }
            }
            
            if y < grid.len()-3 {
                let vertical_product: u32 = grid[y][x].parse::<u32>().unwrap() * grid[y+1][x].parse::<u32>().unwrap() * grid[y+2][x].parse::<u32>().unwrap() * grid[y+3][x].parse::<u32>().unwrap();
                if largest_product < vertical_product { largest_product = vertical_product; }
            }
        }
    }

    println!("{}", largest_product);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}
