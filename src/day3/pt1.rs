use std::time::Instant;

use crate::OutputStruct;

pub fn pt1() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day3/input.txt").expect("Open failed");

    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    let iter = string.split("\n");
    let mut sum = 0;
    iter.into_iter().for_each(|s| {
        if !s.is_empty() {
            let cmp1 = &s[..s.chars().count() / 2];
            let cmp2 = &s[s.chars().count() / 2..];
            for c in cmp1.chars() {
                if cmp2.contains(c) {
                    if c as i32 > 96 {
                        sum += c as i32 - 96;
                    }else {
                        sum += c as i32 - 38;
                    }
                    break;
                }
            }
        }
    });

    output.calcTime = clock.elapsed().as_micros();
    output.answer = sum.to_string();

    return output;
}