use std::time::Instant;

use crate::OutputStruct;

pub fn pt2() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day10/input.txt").expect("Open failed");
    let mut instructions: Vec<(i32, i32)> = Vec::new();
    for line in string.trim().split("\n").collect::<Vec<&str>>() {
        let mut t = 1;
        let mut num = 0;
        if line.trim() != "noop" {
            let tuple = line.split_once(" ").unwrap();
            t = 2;
            num = tuple.1.trim().parse::<i32>().unwrap();
        }
        instructions.push((t, num));
    }
    instructions.reverse();

    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    let mut reg = 1;
    let mut cycle = 1;
    let mut out = "                     ║                   ║                   ║                ║\n\n".to_string();

    let mut inst: (i32, i32) = (1, 0);

    while cycle <= 240 {
        inst.0 -= 1;

        if inst.0 == 0 {
            reg += inst.1;

            if instructions.len() > 0 {
                inst = instructions.pop().unwrap();
            }
        }

        if (cycle - 1) % 40 >= reg - 1 && (cycle -1) % 40 <= reg + 1 {
            out = out + "█";
        }
        else {
            out = out + " ";
        }

        if cycle % 40 == 0 {
            out = out + "\n";
        }

        cycle += 1;
    }

    output.calcTime = clock.elapsed().as_micros();
    output.answer = out + "\n║                 ║                   ";

    return output;
}