fn calc(x: i64, mut y: i64) -> i64 {
    let mut result = x.abs();
    for _ in 0..x.abs() {
        if y > 0 {
            y -= 1;
        } else {
            y += 1;
        }
    }
    result += (y.abs()+1) / 2;

    return result;
}

fn part1() -> i64 {
    let input = include_str!("input11.txt").trim();
    let mut result = 0;

    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for dir in input.split(',') {
        match dir {
            "n" => y += 2,
            "nw" => {
                y += 1; x -= 1},
            "ne" => {
                y += 1;
                x += 1;
            },
            "s" => y -= 2,
            "sw" => {
                y -= 1;
                x -= 1;
            },
            "se" => {
                y -= 1;
                x += 1;
            },
            _ => println!("Warning")
        }
    }

    result = calc(x, y);
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input11.txt").trim();
    let mut result = 0;

    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for dir in input.split(',') {
        match dir {
            "n" => y += 2,
            "nw" => {
                y += 1; x -= 1},
            "ne" => {
                y += 1;
                x += 1;
            },
            "s" => y -= 2,
            "sw" => {
                y -= 1;
                x -= 1;
            },
            "se" => {
                y -= 1;
                x += 1;
            },
            _ => println!("Warning")
        }

        let tmp = calc(x, y);
        if tmp > result {
            result = tmp;
        }
    }

    return result;
}

fn main() {
    println!("Day 11!");
    println!("{}", part1());
    println!("{}", part2());
}
