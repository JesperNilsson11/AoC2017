fn part1() -> i64 {
    let input = include_str!("input1.txt").trim();
    let mut result = 0;

    let nums: Vec<char> = input.chars().collect();

    if nums[0] == nums[nums.len()-1] {
        result += nums[0].to_digit(10).unwrap() as i64;
    }
    for i in 1..nums.len() {
        if nums[i-1] == nums[i] {
            result += nums[i].to_digit(10).unwrap() as i64;
        }
    }
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input1.txt").trim();
    let mut result = 0;
    let nums: Vec<char> = input.chars().collect();
    
    let jump = nums.len() / 2;
    for i in 0..nums.len() {
        if nums[i] == nums[(i+jump) % (nums.len())] {
            result += nums[i].to_digit(10).unwrap() as i64;
        }
    }

    return result;
}

fn main() {
    println!("Day 1!");
    println!("{}", part1());
    println!("{}", part2());
}
