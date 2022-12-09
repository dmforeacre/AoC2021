use std::time::Instant;

use crate::OutputStruct;

pub fn pt2() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day3/day3.txt").expect("Open failed");

    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    let iter: Vec<&str> = string.split("\n").collect();
    let mut sum = 0;
    let mut i = 0;
    while i < iter.len() {
        if !iter[i].is_empty() {
            for c in iter[i].chars() {                
                if iter[i+1].contains(c) && iter[i+2].contains(c) {
                    if c as i32 > 96 {
                        sum += c as i32 - 96;
                    }else {
                        sum += c as i32 - 38;
                    }
                    break;
                }
            }
        }
        i += 3;
    }

    output.calcTime = clock.elapsed().as_micros();
    output.answer = sum.to_string();

    return output;
}