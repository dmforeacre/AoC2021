use std::time::Instant;

use crate::OutputStruct;

fn isBlocked(grid: &Vec<Vec<i32>>, x: i32, y: i32) -> bool {
    let mut blockedTop = false;
    let mut blockedBot = false;
    let mut blockedLeft = false;
    let mut blockedRight = false;
    let mut i:i32 = x - 1;
    while !blockedLeft && i >= 0 {
        if grid[i as usize][y as usize] >= grid[x as usize][y as usize] {
            blockedLeft = true;
        }
        i -= 1;
    }
    i = x + 1;
    while !blockedRight && i < grid.len() as i32 {
        if grid[i as usize][y as usize] >= grid[x as usize][y as usize] {
            blockedRight = true;
        }
        i += 1;
    }
    i = y - 1;
    while !blockedTop && i >= 0 {
        if grid[x as usize][i as usize] >= grid[x as usize][y as usize] {
            blockedTop = true;
        }
        i -= 1;
    }
    i = y + 1;
    while !blockedBot && i < grid[0].len() as i32 {
        if grid[x as usize][i as usize] >= grid[x as usize][y as usize] {
            blockedBot = true;
        }
        i += 1;
    }
    return blockedBot && blockedTop && blockedLeft && blockedRight;
}

pub fn pt1() -> OutputStruct::OutputStruct {
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

    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if i == 0 || i == grid.len() - 1 || j == 0 || j == grid[0].len() - 1 {
                count += 1;
            }
            else if !isBlocked(&grid, i as i32, j as i32) {
                count += 1;
            }
        }
    }

    output.calcTime = clock.elapsed().as_micros();
    output.answer = count.to_string();

    return output;
}