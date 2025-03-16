use std::collections::HashSet;

fn part1() -> i64 {
    let input = include_str!("input12.txt").trim();
    let mut result = 0;

    let mut sets: Vec<HashSet<i64>> = Vec::new();

    for line in input.split('\n') {
        let (l, r) = line.split_once(" <-> ").unwrap();
        let id: i64 = l.parse().unwrap();
        let mut idxs = Vec::new();

        for num in r.split(", ").map(|x| x.parse::<i64>().unwrap()) {
            let mut idx: i64 = -1;
            for i in 0..sets.len() {
                if sets[i].contains(&num) {
                    idx = i as i64;
                    break;
                }
            }

            if idx == -1 {
                sets.push(HashSet::new());
                idx = sets.len() as i64 - 1; 
                sets[idx as usize].insert(num);
            }

            if idxs.contains(&(idx as usize)) == false {
                idxs.push(idx as usize);
            }
        }

        let mut newset = HashSet::new();
        newset.insert(id);
        for &idx in &idxs {
            for &n in &sets[idx] {
                newset.insert(n);
            }
        }

        idxs.sort();
        idxs.reverse();
        for idx in idxs {
            sets.remove(idx);
        }
        sets.push(newset);
    }

    for i in 0..sets.len() {
        if sets[i].contains(&0) {
            result = sets[i].len() as i64;
            break;
        }
    }
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input12.txt").trim();
    let mut result = 0;
    let mut sets: Vec<HashSet<i64>> = Vec::new();

    for line in input.split('\n') {
        let (l, r) = line.split_once(" <-> ").unwrap();
        let id: i64 = l.parse().unwrap();
        let mut idxs = Vec::new();

        for num in r.split(", ").map(|x| x.parse::<i64>().unwrap()) {
            let mut idx: i64 = -1;
            for i in 0..sets.len() {
                if sets[i].contains(&num) {
                    idx = i as i64;
                    break;
                }
            }

            if idx == -1 {
                sets.push(HashSet::new());
                idx = sets.len() as i64 - 1; 
                sets[idx as usize].insert(num);
            }

            if idxs.contains(&(idx as usize)) == false {
                idxs.push(idx as usize);
            }
        }

        let mut newset = HashSet::new();
        newset.insert(id);
        for &idx in &idxs {
            for &n in &sets[idx] {
                newset.insert(n);
            }
        }

        idxs.sort();
        idxs.reverse();
        for idx in idxs {
            sets.remove(idx);
        }
        sets.push(newset);
    }

    result = sets.len() as i64;

    return result;
}

fn main() {
    println!("Day 12!");
    println!("{}", part1());
    println!("{}", part2());
}
