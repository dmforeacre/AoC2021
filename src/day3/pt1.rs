pub fn pt1() {
    let string = std::fs::read_to_string("src/day3/day3.txt").expect("Open failed");
    let iter = string.split("\n");
    let mut sum = 0;
    iter.into_iter().for_each(|s| {
        if !s.is_empty() {
            let cmp1 = &s[..s.chars().count() / 2];
            let cmp2 = &s[s.chars().count() / 2..];
            for c in cmp1.chars() {
                if cmp2.contains(c) {
                    if c as u32 > 96 {
                        sum += c as u32 - 96;
                    }else {
                        sum += c as u32 - 38;
                    }
                    break;
                }
            }
        }
    });
    println!("{}", sum);
}