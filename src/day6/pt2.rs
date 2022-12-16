use std::time::Instant;

use crate::OutputStruct;

pub fn pt2() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day6/input.txt").expect("Open failed");
    
    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    let chars: Vec<char> = string.chars().collect();
    let mut index = 13;
    let mut found = false;
    while !found && index < chars.len() {
        let mut size = 0;
        while isUnique(chars[(index - 13)..(index - 13 + size)].to_vec(), chars[index - 13 + size]) && size < 14 {
            //print!(" - Starting search at {} to {} -- searching: {}", index - 13, index - 13 + size, chars[index - 12 + size]);
            size += 1;
        }
        if size == 14 {
            found = true;
        }
        else {
            index += 1;
        }
    }
    /*if found {
        println!("Found sequence: {}-{}", (index-13), index);
    }
    else {
        println!("Sequence not found");
    }*/

    output.calcTime = clock.elapsed().as_micros();
    // Add 1 to account for 0 index
    output.answer = (index + 1).to_string();

    return output;
}

fn isUnique(arr: Vec<char>, next: char) -> bool {
    //print!(" Looking for {} ", next);
    if arr.contains(&next){
        //print!(" Found repetition\n");
        return false;
    }
    //print!("\n");
    return true;
}