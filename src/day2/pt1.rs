pub fn pt1() {
    let string = std::fs::read_to_string("src/day2/day2.txt").expect("Open failed");
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
    println!("{}", score);
}