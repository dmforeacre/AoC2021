pub fn pt1()
{
    let string = std::fs::read_to_string("src/day4/day4.txt").expect("Open failed");
    let iter = string.split("\n");
    let mut count = 0;
    for s in iter {
        let nums: Vec<i32> = s.split(&['-',',']).into_iter().map(|n| {n.parse::<i32>().unwrap()}).collect();
        if (nums[0] >= nums[2] && nums[1] <= nums[3]) || (nums[0] <= nums[2] && nums[1] >= nums[3]) {
            count += 1;
        }
    }
    println!("{}", count);
}