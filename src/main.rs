#![allow(non_snake_case)]
use std::env;
extern crate itertools;

mod OutputStruct;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut results = Vec::<OutputStruct::OutputStruct>::new();

    println!("╔═════════════════╦════════════════[ Advent of Code 2022 ]═══════════════════════════════════════╗");
    println!("║     Problem     ║        Answer       |       Input       |     Calculation   |     Total      ║");
    println!("╠═════════════════╬═════════════════════╦═══════════════════╦═══════════════════╦════════════════╣");

    if args[1] == "all" {
        for i in 1..6 {
            for j in 1..3 {
                results = func_map(results, i, j);
            }
        }
    }
    else {
        results = func_map(results, args[1].parse::<u32>().unwrap(), args[2].parse::<u32>().unwrap())
    }
    println!("╚═════════════════╬═════════════════════╬═══════════════════╬═══════════════════╬════════════════╣");
    let mut iTotal = 0;
    let mut cTotal = 0;
    let mut tTotal = 0;
    let size = results.len() as u128;
    for r in results {
        iTotal += r.parseTime;
        cTotal += r.calcTime;
        tTotal += r.parseTime + r.calcTime;
    }
    println!("                  ║          Totals:    ║{:>17}  ║{:>17}  ║{:>14}  ║", iTotal, cTotal, tTotal);
    println!("                  ║          Averages:  ║{:>17}  ║{:>17}  ║{:>14}  ║", iTotal / size, cTotal / size, tTotal / size);
    println!("                  ╚═════════════════════╩═══════════════════╩═══════════════════╩════════════════╝");
}

#[allow(unused_assignments)]
fn func_map(mut results: Vec::<OutputStruct::OutputStruct> ,day: u32, part: u32) -> Vec<OutputStruct::OutputStruct> {
    //println!("Starting {day}-{part}...");

    let mut output = OutputStruct::new();
    match (day, part) {
        (1, 1) => output = day1::pt1::pt1(),
        (1, 2) => output = day1::pt2::pt2(),
        (2, 1) => output = day2::pt1::pt1(),
        (2, 2) => output = day2::pt2::pt2(),
        (3, 1) => output = day3::pt1::pt1(),
        (3, 2) => output = day3::pt2::pt2(),
        (4, 1) => output = day4::pt1::pt1(),
        (4, 2) => output = day4::pt2::pt2(),
        (5, 1) => output = day5::pt1::pt1(),
        (5, 2) => output = day5::pt2::pt2(),
        (6, 1) => output = day6::pt1::pt1(),
        (6, 2) => output = day6::pt2::pt2(),
        (7, 1) => output = day7::pt1::pt1(),
        (7, 2) => output = day7::pt2::pt2(),
        (8, 1) => output = day8::pt1::pt1(),
        (8, 2) => output = day8::pt2::pt2(),
        (9, 1) => output = day9::pt1::pt1(),
        (9, 2) => output = day9::pt2::pt2(),
        _ => println!("Problem not found")
    }

    //println!("  -->  Answer: {}", output.answer);

    //println!("Completed {day}-{part} in => Parse ({:?})  Solution ({:?}) microseconds", output.parseTime, output.calcTime); //clock.elapsed().as_micros());
    //println!("║     Problem     ║        Answer       |       Input       |     Calculation   |     Total      ║");  
    println!("║       {:>2}-{:>1}      ║{:>19}  ║{:>17}  ║{:>17}  ║{:>14}  ║", day, part,
        output.answer, output.parseTime, output.calcTime, output.parseTime + output.calcTime);
    results.push(output);

    return results;
}
