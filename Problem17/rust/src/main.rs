use std::{time::{Instant}};

fn main() {
    let timer: Instant = Instant::now();
    
    let mut map: Vec<u32> = vec![0; 1001];
    map[0] = 0;
    map[1] = 3;
    map[2] = 3;
    map[3] = 5;
    map[4] = 4;
    map[5] = 4;
    map[6] = 3;
    map[7] = 5;
    map[8] = 5;
    map[9] = 4;
    map[10] = 3;
    map[11] = 6;
    map[12] = 6;
    map[13] = 8;
    map[14] = 8;
    map[15] = 7;
    map[16] = 7;
    map[17] = 9;
    map[18] = 8;
    map[19] = 8;
    map[20] = 6;
    map[30] = 6;
    map[40] = 5;
    map[50] = 5;
    map[60] = 5;
    map[70] = 7;
    map[80] = 6;
    map[90] = 6;

    for i in 21..100 {
        let tens_digit: usize = (i/10 as usize) * 10;
        let ones_digit: usize = i - tens_digit;
        map[i] = map[tens_digit] + map[ones_digit];
    }

    for i in 100..1000 {
        let hundreds_digit: usize = (i/100 as usize) * 100;
        let tens_ones_digits: usize = i - hundreds_digit;

        if tens_ones_digits == 0 {
            map[i] = map[hundreds_digit / 100] + 7;
        } else {
            map[i] = map[hundreds_digit / 100] + 10 + map[tens_ones_digits];
        }
    }

    map[1000] = 11;

    println!("{}", map.into_iter().sum::<u32>());

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}
