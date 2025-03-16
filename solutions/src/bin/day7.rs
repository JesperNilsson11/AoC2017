use std::collections::{HashMap, HashSet};

fn part1() -> i64 {
    let input = include_str!("input7.txt").trim();
    let mut result = 0;
    let mut hmap: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut child_nodes = HashSet::new();

    for line in input.split('\n') {
        let (l, r) = line.split_once(' ').unwrap();
        let mut tmp = Vec::new();
        
        if let Some(c) = r.split_once("-> ") {
            for child in c.1.split(", ") {
                tmp.push(child);
                child_nodes.insert(child);
            }
        }
        hmap.insert(l, tmp);
    }

    for k in hmap.keys() {
        if child_nodes.contains(k) == false {
            println!("root: {}", k);
        }
    }
    
    return result;
}

fn calc(node: &str, hmap: &HashMap<&str, (i64, Vec<&str>)>) -> i64 {
    let e = hmap.get(node).unwrap();
    let mut res = e.0;

    let mut weights = Vec::new();
    for c in &e.1 {
        let tmp = calc(c, hmap);
        res += tmp;
        weights.push(tmp);
    }
    if weights.len() == 2 {
        if weights[0] != weights[1] {
            println!("wrong: {} or {}", e.1[0], e.1[1]);
        }
    }

    for i in 0..weights.len() {
        let cmp = weights[i];
        let cmp1 = weights[(i+weights.len()-1) % weights.len()];
        let cmp2 = weights[(i+1) % weights.len()];
        if  cmp1 != cmp && cmp2 != cmp {
            println!("wrong {} {} {}", e.1[i], cmp, cmp1 - cmp + hmap.get(e.1[i]).unwrap().0);
        }
    }

    return res;
}

fn part2() -> i64 {
    let input = include_str!("input7.txt").trim();
    let mut result = 0;

    let mut hmap: HashMap<&str, (i64, Vec<&str>)> = HashMap::new();
    let mut child_nodes = HashSet::new();

    for line in input.split('\n') {
        let (l, r) = line.split_once(' ').unwrap();
        let mut tmp = Vec::new();

        let idx = r.find(')').unwrap();
        let weight: i64 = r[1..idx].parse::<i64>().unwrap();
        
        if let Some(c) = r.split_once("-> ") {
            for child in c.1.split(", ") {
                tmp.push(child);
                child_nodes.insert(child);
            }
        }
        hmap.insert(l, (weight, tmp));
    }

    for k in hmap.keys() {
        if child_nodes.contains(k) == false {
            println!("root: {}", k);
            result = calc(k, &hmap);
        }
    }
    

    return result;
}

fn main() {
    println!("Day 7!");
    println!("{}", part1());
    println!("{}", part2());
}
