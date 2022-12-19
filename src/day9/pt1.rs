use std::time::Instant;

use crate::OutputStruct;

fn updateTail(visited: &mut Vec<(i32,i32)>, count: i32, h: Vec<i32>, t: &mut Vec<i32>) -> i32 {
    let diff = (h[0] - t[0], h[1] - t[1]);
    //println!("H: {:?} T: {:?} diff: {:?}", h, t, diff);
    if diff.0.abs() > 1 {
        t[0] += diff.0.abs() / diff.0;
        if h[1] > t[1] {
            t[1] += 1;
        }
        else if h[1] < t[1] {
            t[1] -= 1;
        }
    }
    if diff.1.abs() > 1 {
        t[1] += diff.1.abs() / diff.1;
        if h[0] > t[0] {
            t[0] += 1;
        }
        else if h[0] < t[0] {
            t[0] -= 1;
        }
    }

    if visited.iter().find(|&&x| x == (t[0],t[1])) == None {
        visited.push((t[0],t[1]));
        return count + 1;
    }

    return count;
}

pub fn pt1() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day9/input.txt").expect("Open failed");
    
    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    let mut headPos = vec![0;2];
    let mut tailPos = vec![0;2];

    let mut visited: Vec<(i32,i32)> = Vec::new();
    let mut count = 0;

    for line in string.trim().split("\n") {
        let tokens: Vec<&str> = line.trim().split(" ").collect();
        match tokens[0] {
            "U" => {
                for _ in 0..tokens[1].parse::<i32>().unwrap() {
                    headPos[1] += 1;
                    count = updateTail(&mut visited, count, headPos.clone(), &mut tailPos);
                }
                //println!("Up");
            },
            "D" => {
                for _ in 0..tokens[1].parse::<i32>().unwrap() {
                    headPos[1] -= 1;
                    count = updateTail(&mut visited, count, headPos.clone(), &mut tailPos);
                }
                //println!("Down");
            },
            "L" => {
                for _ in 0..tokens[1].parse::<i32>().unwrap() {
                    headPos[0] -= 1;
                    count = updateTail(&mut visited, count, headPos.clone(), &mut tailPos);
                }
                //println!("Left");
            },
            "R" => {
                for _ in 0..tokens[1].parse::<i32>().unwrap() {
                    headPos[0] += 1;
                    count = updateTail(&mut visited, count, headPos.clone(), &mut tailPos);
                }
                //println!("Right");
            },
            _ => break,
        }
    }

    //println!("{:?} {:?}", headPos, tailPos);

    output.calcTime = clock.elapsed().as_micros();
    output.answer = count.to_string();

    return output;
}