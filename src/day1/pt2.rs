use std::time::Instant;

use crate::OutputStruct;

pub fn pt2() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day1/day1.txt").expect("Open failed");

    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    let iter = string.split("\n\n");
    let mut vec: Vec<i32> = Vec::new();
    iter.into_iter().for_each(|s| {
        let elf: Vec<i32> = s.trim().split("\n").map(|x| {x.parse::<i32>().unwrap()}).collect();
        vec.push(elf.iter().sum::<i32>());
    });
    vec.sort();
    vec.reverse();
    let mut sum = 0;
    for i in 0..3 {
        sum += vec[i];
    }

    output.calcTime = clock.elapsed().as_micros();
    output.answer = sum.to_string();

    return output;
}