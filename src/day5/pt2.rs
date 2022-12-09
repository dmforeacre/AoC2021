use std::time::Instant;

use crate::OutputStruct;

pub fn pt2() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day5/input.txt").expect("Open failed");

    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    // Split text into stack and moves
    let data: Vec<&str> = string.trim().split("\n\n").collect();
    //println!("{:#?}", data);

    // Data[0] contains the initial stack configuration
    let stacks: Vec<&str> = data[0].split("\n").collect();
    
    // Create vectors representing each stack
    let numStacks = stacks[0].len();
    let mut stackVectors: Vec<Vec<char>> = Vec::new();
    for _i in 0..(numStacks / 4 + 1){        
        let stack: Vec<char> = Vec::new();
        stackVectors.push(stack);
    }
    // Fill each vector with characters from the file
    for i in 0..stackVectors.len() - 1 {
        let string: Vec<char> = stacks[i].chars().collect();
        let mut j = 1;
        while j < stacks[i].len(){
            if string[j] != ' '{
                stackVectors[j / 4].push(string[j]);
            }
            j += 4;
        }
    }

    // Reverse the filled vectors
    for i in 0..stackVectors.len() {
        stackVectors[i].reverse();
        //println!("{:#?}", stackVectors[i]);
    }

    // Read each move and adjust stacks accordingly
    for s in data[1].split("\n"){
        let tokens: Vec<&str> = s.trim().split(" ").collect();
        let mut crates: Vec<char> = Vec::new();
        for _i in 0..(tokens[1].parse::<usize>().unwrap()){
            let c: char = stackVectors[tokens[3].parse::<usize>().unwrap() - 1].pop().unwrap();
            //println!("Moving {} at {} to {}", c, tokens[3].parse::<usize>().unwrap(), tokens[5].parse::<usize>().unwrap());
            crates.push(c);
        }
        crates.reverse();
        for c in crates {
            stackVectors[tokens[5].parse::<usize>().unwrap() - 1].push(c);
        }
        //let mut line = String::new();
        //std::io::stdin().read_line(&mut line);
    }

    // Build list of crates
    let mut crates: Vec<char> = Vec::new();
    for mut s in stackVectors{
        //println!("{:#?}", s);
        let c = s.pop();
        crates.push(c.unwrap());
    }
    let mut answer = String::new();
    for c in crates{
        answer.push_str(&c.to_string());
    }

    output.calcTime = clock.elapsed().as_micros();
    output.answer = answer;

    return output;
}