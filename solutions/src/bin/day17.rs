fn part1() -> i64 {
    let mut result = 0;

    let mut list = vec![0];
    let mut pos = 0;
    for i in 1..=2017 {
        pos = (pos + 324 + 1) % list.len();
        list.insert(pos, i);
    }

    for i in 0..list.len() {
        if list[i] == 2017 {
            result = list[i + 1];
            break;
        }
    }
    
    return result;
}

fn part2() -> i64 {
    let mut result = 0;

    let mut pos = 0;
    for i in 1..=50000000 {
        pos = (pos+324) % i;

        if pos == 0 {
            result = i;
        }
        pos += 1;
    }

    return result;
}

fn main() {
    println!("Day 17!");
    println!("{}", part1());
    println!("{}", part2());
}
