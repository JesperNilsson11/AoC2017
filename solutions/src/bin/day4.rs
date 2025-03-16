use std::collections::HashMap;

fn part1() -> i64 {
    let input = include_str!("input4.txt").trim();
    let mut result = 0;

    for line in input.split('\n') {
        let words: Vec<&str> = line.split_whitespace().collect();

        let mut valid = true;
        for i in 0..words.len() {
            for j in i+1..words.len() {
                if words[i] == words[j] {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            result += 1;
        }
    }
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input4.txt").trim();
    let mut result = 0;

    for line in input.split('\n') {
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut hashes = Vec::new();
        for word in words {
            let mut hmap: HashMap<char, i64> = HashMap::new();
            for c in word.chars() {
                *hmap.entry(c).or_default() += 1;
            }
            hashes.push(hmap);
        }

        let mut valid = true;
        for i in 0..hashes.len() {
            for j in i+1..hashes.len() {
                if hashes[i].len() != hashes[j].len() {
                    continue;
                }
                valid = false;
                for (k,&v) in &hashes[i] {
                    let mut v2: i64 = -1;
                    if hashes[j].contains_key(k) {
                        v2 = *hashes[j].get(k).unwrap();
                    }

                    if v != v2 {
                        valid = true;
                        break;
                    }
                }
                if valid == false {
                    break;
                }
            }
            if valid == false {
                break;
            }
        }
        if valid {
            result += 1;
        }
    }
    
    return result;
}

fn main() {
    println!("Day 4!");
    println!("{}", part1());
    println!("{}", part2());
}
