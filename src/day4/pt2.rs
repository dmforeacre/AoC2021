use std::time::Instant;

use crate::OutputStruct;

pub fn pt2() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day4/input.txt").expect("Open failed");

    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    let iter = string.trim().split("\n");
    let mut count = 0;
    for s in iter {
        let nums: Vec<i32> = s.split(&['-',',']).into_iter().map(|n| {n.parse::<i32>().unwrap()}).collect();
        if !((nums[0] < nums[2] && nums[1] < nums[2]) || (nums[0] > nums[3] && nums[1] > nums[3])) {
            count += 1;
        }
    }

    output.calcTime = clock.elapsed().as_micros();
    output.answer = count.to_string();

    return output;
}