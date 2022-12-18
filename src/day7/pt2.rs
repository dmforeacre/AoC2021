use std::time::Instant;

use crate::OutputStruct;

pub fn pt2() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    /*let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day7/input.txt").expect("Open failed");
    
    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    output.calcTime = clock.elapsed().as_micros();*/
    output.answer = "Can't do in Rust".to_string();

    return output;
}