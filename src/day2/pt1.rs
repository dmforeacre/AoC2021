use std::time::Instant;

use crate::OutputStruct;

pub fn pt1() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day2/input.txt").expect("Open failed");

    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    let iter = string.split("\n");
    let mut score = 0;
    iter.into_iter().for_each(|s| {
        if !s.is_empty() {
            let moves: Vec<&str> = s.split(" ").collect();
            match moves[0] {
                "A" => match moves[1] {
                    "X" => score += 4,
                    "Y" => score += 8,
                    _ => score += 3
                },
                "B" => match moves[1] {
                    "X" => score += 1,
                    "Y" => score += 5,
                    _ => score += 9
                },
                _ => match moves[1] {
                    "X" => score += 7,
                    "Y" => score += 2,
                    _ => score += 6
                }
            }
        }
    });

    output.calcTime = clock.elapsed().as_micros();
    output.answer = score.to_string();

    return output;
}