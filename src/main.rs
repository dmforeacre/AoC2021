use std::time::Instant;
use itertools::Itertools;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let mut clock = Instant::now();

    for i in 1..5 {
        for j in 1..3 {
            funcMap(clock, i, j);
        }
    }    
}

fn funcMap(mut clock: Instant, day: u32, part: u32) {
    clock = Instant::now();
    println!("Starting {day}-{part}...");

    match (day, part) {
        (1, 1) => day1::pt1::pt1(),
        (1, 2) => day1::pt2::pt2(),
        (2, 1) => day2::pt1::pt1(),
        (2, 2) => day2::pt2::pt2(),
        (3, 1) => day3::pt1::pt1(),
        (3, 2) => day3::pt2::pt2(),
        (4, 1) => day4::pt1::pt1(),
        (4, 2) => day4::pt2::pt2(),
        _ => println!("Problem not found")
    }

    println!("Completed {day}-{part} in: {} microseconds", clock.elapsed().as_micros());   
}
