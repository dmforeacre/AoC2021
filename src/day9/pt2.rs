use std::time::Instant;

use crate::OutputStruct;

fn updateTail(h: Vec<i32>, t: &mut Vec<i32>) {
    let diff = (h[0] - t[0], h[1] - t[1]);
    if diff.0.abs() > 1 {
        t[0] += diff.0.abs() / diff.0;
        if diff.1.abs() == 1 {
            t[1] += diff.1.abs() / diff.1;
        }
    }
    if diff.1.abs() > 1 {
        t[1] += diff.1.abs() / diff.1;
        if diff.0.abs() == 1 {
            t[0] += diff.0.abs() / diff.0;
        }
    }
}

pub fn pt2() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day9/input.txt").expect("Open failed");
    
    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    let mut knots = vec![vec![0;2];10];

    let mut visited: Vec<(i32,i32)> = Vec::new();
    let mut count = 0;

    for line in string.trim().split("\n") {
        let tokens: Vec<&str> = line.trim().split(" ").collect();
        match tokens[0] {
            "U" => {
                for _ in 0..tokens[1].parse::<i32>().unwrap() {
                    knots[0][1] += 1;
                    for i in 0..9 {
                        updateTail(knots[i].clone(), &mut knots[i + 1]);
                    }
                    if visited.iter().find(|&&x| x == (knots[9][0],knots[9][1])) == None {
                        visited.push((knots[9][0],knots[9][1]));
                        count += 1;
                    }
                }
            },
            "D" => {
                for _ in 0..tokens[1].parse::<i32>().unwrap() {
                    knots[0][1] -= 1;
                    for i in 0..9 {
                        updateTail(knots[i].clone(), &mut knots[i + 1]);
                    }
                    if visited.iter().find(|&&x| x == (knots[9][0],knots[9][1])) == None {
                        visited.push((knots[9][0],knots[9][1]));
                        count += 1;
                    }
                }
            },
            "L" => {
                for _ in 0..tokens[1].parse::<i32>().unwrap() {
                    knots[0][0] -= 1;
                    for i in 0..9 {
                        updateTail(knots[i].clone(), &mut knots[i + 1]);
                    }
                    if visited.iter().find(|&&x| x == (knots[9][0],knots[9][1])) == None {
                        visited.push((knots[9][0],knots[9][1]));
                        count += 1;
                    }
                }
            },
            "R" => {
                for _ in 0..tokens[1].parse::<i32>().unwrap() {
                    knots[0][0] += 1;
                    for i in 0..9 {
                        updateTail(knots[i].clone(), &mut knots[i + 1]);
                    }
                    if visited.iter().find(|&&x| x == (knots[9][0],knots[9][1])) == None {
                        visited.push((knots[9][0],knots[9][1]));
                        count += 1;
                    }
                }
            },
            _ => break,
        }
    }
    output.calcTime = clock.elapsed().as_micros();
    output.answer = count.to_string();

    return output;
}