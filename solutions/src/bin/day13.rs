use std::collections::HashMap;

fn part1() -> i64 {
    let input = include_str!("input13.txt").trim();
    let mut result = 0;

    let mut hmap = HashMap::new();
    let mut max_layers = 0;
    for line in input.split('\n') {
        let (l, r) = line.split_once(": ").unwrap();
        let layer: i64 = l.parse().unwrap();
        let depth: i64 = r.parse().unwrap();

        if layer > max_layers {
            max_layers = layer;
        }
        hmap.insert(layer, (depth, 0, 1));
    }

    for i in 0..=max_layers {
        if hmap.contains_key(&i) {
            let e = hmap.get(&i).unwrap();
            if e.1 == 0 {
                result += i * e.0;
            }
        }

        for (k, v) in &mut hmap {
            v.1 += v.2;
            if v.1 == v.0 - 1 {
                v.2 = -1;
            } else if v.1 == 0 {
                v.2 = 1;
            } else if v.1 < 0 {
                v.1 = 0;
                v.2 = 1;
            }
        }
    }
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input13.txt").trim();
    let mut result = 0;

    let mut nums = Vec::new();
    let mut max_layers = 0;
    for line in input.split('\n') {
        let (l, r) = line.split_once(": ").unwrap();
        let layer: i64 = l.parse().unwrap();
        let depth: i64 = r.parse().unwrap();

        if layer > max_layers {
            max_layers = layer;
        }
        let div = (depth-1) * 2;
        nums.push((layer, div));
    }
    
    loop {
        let mut valid = true;

        for (l, d) in &nums {
            if (result + l) % d == 0 {
                valid = false;
                break;
            }
        }

        if valid {
            break;
        }
        result += 1;
    }

    return result;
}

fn main() {
    println!("Day 13!");
    println!("{}", part1());
    println!("{}", part2());
}
