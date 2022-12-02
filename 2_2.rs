fn main() {
    let string = std::fs::read_to_string("day2.txt").expect("Open failed");
    let iter = string.split("\n");
    let mut score = 0;
    iter.into_iter().for_each(|s| {
        if !s.is_empty() {
            let moves: Vec<&str> = s.split(" ").collect();
            match moves[0] {
                "A" => match moves[1] {
                    "X" => score += 3,
                    "Y" => score += 4,
                    _ => score += 8
                },
                "B" => match moves[1] {
                    "X" => score += 1,
                    "Y" => score += 5,
                    _ => score += 9
                },
                _ => match moves[1] {
                    "X" => score += 2,
                    "Y" => score += 6,
                    _ => score += 7
                }
            }
        }
    });
    println!("{}", score);
}