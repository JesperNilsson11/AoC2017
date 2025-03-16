use std::collections::{HashMap, HashSet};

fn part1() -> i64 {
    let input = include_str!("input19.txt");
    let mut result = 0;

    let mut hmap = HashMap::new();
    let mut y = 0;
    let mut pos = (0, 0);
    for line in input.split('\n') {
        let mut x = 0;
        for c in line.chars() {
            if c != ' ' {
                //println!("{}", c);

                if y == 0 {
                    pos.0 = x;
                }
                hmap.insert((x, y), c);
            }
            x += 1;
        }
        y += 1;
    }

    let mut dx = 0;
    let mut dy = 1;
    let mut ans = String::new();
    let mut visited = HashSet::new();
    visited.insert(pos);
    loop {
        let nx = pos.0 + dx;
        let ny = pos.1 + dy;

        if hmap.contains_key(&(nx, ny)) {
            let c = *hmap.get(&(nx, ny)).unwrap();
            if c.is_alphabetic() {
                ans.push(c);
            }
            pos = (nx, ny);
            visited.insert(pos);
        } else {
            let mut possible = 0;
            for (ndx, ndy) in [(1,0), (-1,0), (0,1), (0,-1)] {
                let nx = pos.0 + ndx;
                let ny = pos.1 + ndy;

                if hmap.contains_key(&(nx, ny)) && visited.contains(&(nx, ny)) == false {
                    possible += 1;
                    dx = ndx;
                    dy = ndy;
                }
            }

            if possible == 0 {
                break;
            }
            if possible > 1 {
                println!("warning");
            }
            visited.insert(pos);
        }
    }

    println!("{}", ans);
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input19.txt");
    let mut result = 0;

    let mut hmap = HashMap::new();
    let mut y = 0;
    let mut pos = (0, 0);
    for line in input.split('\n') {
        let mut x = 0;
        for c in line.chars() {
            if c != ' ' {
                //println!("{}", c);

                if y == 0 {
                    pos.0 = x;
                }
                hmap.insert((x, y), c);
            }
            x += 1;
        }
        y += 1;
    }

    let mut dx = 0;
    let mut dy = 1;
    let mut ans = String::new();
    let mut visited = HashSet::new();
    visited.insert(pos);
    result = 1;
    loop {
        let nx = pos.0 + dx;
        let ny = pos.1 + dy;

        if hmap.contains_key(&(nx, ny)) {
            let c = *hmap.get(&(nx, ny)).unwrap();
            if c.is_alphabetic() {
                ans.push(c);
            }
            pos = (nx, ny);
            visited.insert(pos);
            result += 1;
        } else {
            let mut possible = 0;
            for (ndx, ndy) in [(1,0), (-1,0), (0,1), (0,-1)] {
                let nx = pos.0 + ndx;
                let ny = pos.1 + ndy;

                if hmap.contains_key(&(nx, ny)) && visited.contains(&(nx, ny)) == false {
                    possible += 1;
                    dx = ndx;
                    dy = ndy;
                }
            }

            if possible == 0 {
                break;
            }
            if possible > 1 {
                println!("warning");
            }
            visited.insert(pos);
        }
    }

    println!("{}", ans);
    
    return result;
}

fn main() {
    println!("Day 19!");
    println!("{}", part1());
    println!("{}", part2());
}
