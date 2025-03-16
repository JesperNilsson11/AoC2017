fn part1() -> i64 {
    let input = include_str!("input2.txt").trim();
    let mut result = 0;

    for line in input.split('\n') {
        let nums: Vec<i64> = line.split('\t').map(|x| x.parse().unwrap()).collect();
        let mut min = nums[0];
        let mut max = nums[0];

        for n in nums {
            if n > max {
                max = n;
            }
            if n < min {
                min = n;
            }
        }
        result += max-min;
    }
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input2.txt").trim();
    let mut result = 0;

    for line in input.split('\n') {
        let nums: Vec<i64> = line.split('\t').map(|x| x.parse().unwrap()).collect();

        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                let nom;
                let denom;
                if nums[i] < nums[j] {
                    nom = nums[j];
                    denom = nums[i];
                } else {
                    nom = nums[i];
                    denom = nums[j];
                }

                if nom % denom == 0 {
                    result += nom / denom;
                }
            }
        }
    }
    
    return result;
}

fn main() {
    println!("Day 2!");
    println!("{}", part1());
    println!("{}", part2());
}
