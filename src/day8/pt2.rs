use std::time::Instant;

use crate::OutputStruct;

fn getScore(grid: &Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
    let mut blockedTop = false;
    let mut blockedBot = false;
    let mut blockedLeft = false;
    let mut blockedRight = false;
    let mut counts: Vec<i32> = vec![0;4];
    let mut i:i32 = x - 1;
    while !blockedLeft && i >= 0 {
        if grid[i as usize][y as usize] >= grid[x as usize][y as usize] {
            blockedLeft = true;
        }
        counts[0] += 1;
        i -= 1;
    }
    i = x + 1;
    while !blockedRight && i < grid.len() as i32 {
        if grid[i as usize][y as usize] >= grid[x as usize][y as usize] {
            blockedRight = true;
        }
        counts[1] += 1;
        i += 1;
    }
    i = y - 1;
    while !blockedTop && i >= 0 {
        if grid[x as usize][i as usize] >= grid[x as usize][y as usize] {
            blockedTop = true;
        }        
        counts[2] += 1;
        i -= 1;
    }
    i = y + 1;
    while !blockedBot && i < grid[0].len() as i32 {
        if grid[x as usize][i as usize] >= grid[x as usize][y as usize] {
            blockedBot = true;
        }
        counts[3] += 1;
        i += 1;
    }
    return counts[0] * counts[1] * counts[2] * counts[3];
}

pub fn pt2() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day8/input.txt").expect("Open failed");
    
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for s in string.trim().split('\n') {
        let mut newLine: Vec<i32> = Vec::new();
        for c in s.trim().chars() {
            newLine.push(c.to_digit(10).unwrap() as i32);
        }
        grid.push(newLine);
    }

    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    let mut best = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let score = getScore(&grid, i as i32, j as i32);
            if score > best {
                best = score;
            }
        }
    }

    output.calcTime = clock.elapsed().as_micros();
    output.answer = best.to_string();

    return output;
}