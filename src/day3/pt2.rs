pub fn pt2() {
    let string = std::fs::read_to_string("src/day3/day3.txt").expect("Open failed");
    let iter: Vec<&str> = string.split("\n").collect();
    let mut sum = 0;
    let mut i = 0;
    while i < iter.len() {
        if !iter[i].is_empty() {
            for c in iter[i].chars() {                
                if iter[i+1].contains(c) && iter[i+2].contains(c) {
                    if c as u32 > 96 {
                        sum += c as u32 - 96;
                    }else {
                        sum += c as u32 - 38;
                    }
                    break;
                }
            }
        }
        i += 3;
    }
    println!("{}", sum);
}