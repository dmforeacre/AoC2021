use std::time::Instant;

use crate::OutputStruct;

pub fn pt1() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day6/input.txt").expect("Open failed");
    
    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    let chars: Vec<char> = string.chars().collect();
    let mut index = 0;
    while (countChars(chars[index..(index+4)].to_vec(), chars[index]) > 1 ||
        countChars(chars[index..(index+4)].to_vec(), chars[index+1]) > 1 ||
        countChars(chars[index..(index+4)].to_vec(), chars[index+2]) > 1) &&
        index < chars.len() - 4 {
        index += 1;
    }

    output.calcTime = clock.elapsed().as_micros();
    output.answer = (index + 4).to_string();

    return output;
}

fn countChars(arr: Vec<char>, c: char) -> usize
{
    return arr.iter().filter(|x| x.eq_ignore_ascii_case(&c)).count();
}