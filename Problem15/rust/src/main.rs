use std::time::{Instant};

fn main() {
    let timer: Instant = Instant::now();

    let mut grid: Vec<Vec<u64>> = vec![vec![0; 21]; 21];

    for i in 0..grid.len() {
        grid[i][0] = 1;
        grid[0][i] = 1;
    }

    for x in 1..grid.len() {
        for y in 1..grid.len() {
            grid[x][y] = grid[x - 1][y] + grid[x][y - 1];
        }
    }

    println!("{}", grid[grid.len() - 1][grid.len() - 1]);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}