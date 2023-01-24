use std::{time::{Instant}};

fn main() {
    let timer: Instant = Instant::now();

    //Number of Sundays on the first of the month in the twentieth century
    let mut answer: u16 = 0;

    let mut year: u16 = 1901;
    //1=January, 2=Febuary, 3=March, 4=April, 5=May, 6=June, 7=July, 8=August, 9=September, 10=October, 11=November, 12=December
    let mut month: u8 = 1;
    //1=Sunday, 2=Monday, 3=Tuesday, 4=Wednesday, 5=Thursday, 6=Friday, 7=Saturday
    let mut day: u8 = 3;

    //Continually loop until the end of the 20th century
    loop {
        match month {
            //These months have 30 days
            4 | 6 | 9 | 11 => {
                day += 2; //30 % 7 == 2
            },
            //These months have 31 days
            1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                day += 3; //31 % 7 == 3
            },
            //Special case in febuary
            2 => {
                //Don't add anything if the current year is not a leap year (28 % 7 == 0)
                if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                    day += 1; //29 % 7 == 1
                }
            },
            //This shouldn't happen
            _ => {
                println!("Something went wrong in the logic");
            }
        }

        //Correct the day
        if day > 7 {
            day -= 7;
        }

        month += 1;
        //Correct the year and month
        if month == 13 {
            month = 1;
            year += 1;
        }

        //Add to answer if the first day is Sunday
        if day == 1 {
            answer += 1;
            println!("Year: {}; Month: {}; Day: {}", year, month, day);
        }

        //Break the loop at the end of the 20th century
        if year == 2000 && month == 12 {
            break;
        }
    }

    println!("{}", answer);

    let microseconds: u128 = timer.elapsed().as_micros();
    println!("Code took {} milliseconds to execute", (microseconds as f64) / (1000 as f64));
}
