fn part1() -> i64 {
    let input = include_str!("input5.txt").trim();
    let mut result = 0;

    let mut list: Vec<i64> = input.split('\n').map(|x| x.parse().unwrap()).collect();
    let mut pc: i64 = 0;

    while pc >= 0 && pc < list.len() as i64 {
        result += 1;
        let jmp = list[pc as usize];
        list[pc as usize] += 1;
        pc += jmp;
    }
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input5.txt").trim();
    let mut result = 0;

    let mut list: Vec<i64> = input.split('\n').map(|x| x.parse().unwrap()).collect();
    let mut pc: i64 = 0;

    while pc >= 0 && pc < list.len() as i64 {
        result += 1;
        let jmp = list[pc as usize];
        let mut delta = 1;
        if jmp >= 3 {
            delta = -1;
        }
        list[pc as usize] += delta;
        pc += jmp;
    }
    

    return result;
}

fn main() {
    println!("Day 5!");
    println!("{}", part1());
    println!("{}", part2());
}
