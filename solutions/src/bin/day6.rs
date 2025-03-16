use std::collections::{HashMap, HashSet};

fn part1() -> i64 {
    let input = include_str!("input6.txt").trim();
    let mut result = 0;

    let mut banks: Vec<i64> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut seen = HashSet::new();
    seen.insert(banks.clone());

    loop {
        result += 1;
        let mut idx = 0;
        for i in 0..banks.len() {
            if banks[idx] < banks[i] {
                idx = i;
            }
        }

        let num_banks = banks[idx];
        banks[idx] = 0;
        for _ in 0..num_banks {
            idx += 1;
            idx %= banks.len();
            banks[idx] += 1;
        }

        if seen.contains(&banks) {
            break;
        }
        seen.insert(banks.clone());
    }
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input6.txt").trim();
    let mut result = 0;

    let mut banks: Vec<i64> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut seen = HashMap::new();
    seen.insert(banks.clone(), 0);

    loop {
        result += 1;
        let mut idx = 0;
        for i in 0..banks.len() {
            if banks[idx] < banks[i] {
                idx = i;
            }
        }

        let num_banks = banks[idx];
        banks[idx] = 0;
        for _ in 0..num_banks {
            idx += 1;
            idx %= banks.len();
            banks[idx] += 1;
        }

        if seen.contains_key(&banks) {
            result -= seen.get(&banks).unwrap();
            break;
        }
        seen.insert(banks.clone(), result);
    }
    
    return result;
}

fn main() {
    println!("Day 6!");
    println!("{}", part1());
    println!("{}", part2());
}
