use std::collections::{HashMap, HashSet};

fn part1() -> i64 {
    let input = include_str!("input22.txt").trim();
    let mut result = 0;

    let mut infected = HashSet::new();
    let mut y = 0;
    let mut x = 0;

    for line in input.split('\n') {
        x = 0;
        for c in line.chars() {
            if c == '#' {
                infected.insert((x, y));
            }
            x += 1;
        }
        y += 1;
    }

    x /= 2;
    y /= 2;
    let mut dx = 0;
    let mut dy = -1;

    for _ in 0..10000 {
        if infected.contains(&(x, y)) {
            let tmp = dx;
            dx = -dy;
            dy = tmp;

            infected.remove(&(x, y));
        } else {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            infected.insert((x, y));
            result += 1;
        }

        x += dx;
        y += dy;
    }

    //for k in infected {
    //    println!("({}, {})", k.0, k.1);
    //}

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input22.txt").trim();
    let mut result = 0;

    let mut infected = HashMap::new();
    let mut y = 0;
    let mut x = 0;

    for line in input.split('\n') {
        x = 0;
        for c in line.chars() {
            if c == '#' {
                infected.insert((x, y), 2);
            }
            x += 1;
        }
        y += 1;
    }

    x /= 2;
    y /= 2;
    let mut dx = 0;
    let mut dy = -1;

    for _ in 0..10000000 {
        let e = infected.entry((x, y)).or_insert(0);

        if *e == 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
        } else if *e == 1 {
            result += 1;
        } else if *e == 2 {
            let tmp = dx;
            dx = -dy;
            dy = tmp;
        } else if *e == 3 {
            dx = -dx;
            dy = -dy;
        }

        *e = (*e + 1) % 4;

        x += dx;
        y += dy;
    }

    return result;
}

fn main() {
    println!("Day 22!");
    println!("{}", part1());
    println!("{}", part2());
}
