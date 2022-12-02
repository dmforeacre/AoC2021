fn main() {
    let string = std::fs::read_to_string("1_1_input.txt").expect("Open failed");

    let iter = string.split("\n\n");
    let mut vec: Vec<i32> = Vec::new();
    iter.into_iter().for_each(|s| {
        let elf: Vec<i32> = s.trim().split("\n").map(|x| {x.parse::<i32>().unwrap()}).collect();
        vec.push(elf.iter().sum::<i32>());
    });
    vec.sort();
    vec.reverse();
    let mut sum = 0;
    for i in 0..3 {
        sum += vec[i];
    }
    println!("{}", sum);
}