use std::collections::HashMap;

fn part1() -> i64 {
    let input = include_str!("input16.txt").trim();
    let mut result = 0;
    let mut programs = Vec::new();
    for c in 'a'..='p' {
        programs.push(c);
    }

    for mv in input.split(',') {
        let m = &mv[0..1];

        match m {
            "s" => {
                let num: i64 = (&mv[1..]).parse().unwrap();
                for _ in 0..num {
                    let p = programs.pop().unwrap();
                    programs.insert(0, p);
                }
            }
            "x" => {
                let (s1, s2) = (&mv[1..]).split_once('/').unwrap();
                let n1 = s1.parse::<usize>().unwrap();
                let n2 = s2.parse::<usize>().unwrap();

                let tmp = programs[n1];
                programs[n1] = programs[n2];
                programs[n2] = tmp;
            }
            "p" => {
                let (s1, s2) = (&mv[1..]).split_once('/').unwrap();
                let c1: char = s1.chars().collect::<Vec<char>>()[0];
                let c2: char = s2.chars().collect::<Vec<char>>()[0];
                for i in 0..programs.len() {
                    if programs[i] == c1 {
                        programs[i] = c2;
                    } else if programs[i] == c2 {
                        programs[i] = c1;
                    }
                }
            }
            _ => println!("Warning"),
        }
    }

    for p in programs {
        print!("{}", p);
    }
    println!("");

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input16.txt").trim();
    let mut result = 0;
    let mut programs = Vec::new();
    for c in 'a'..='p' {
        programs.push(c);
    }

    let mut hmap = HashMap::new();
    let mut moves: Vec<&str> = input.split(',').collect();

    let ITERS = 1000000000;
    let mut itr = 0;
    while itr < ITERS {
        for mv in &moves {
            let m = &mv[0..1];

            match m {
                "s" => {
                    let num: i64 = (&mv[1..]).parse().unwrap();
                    for _ in 0..num {
                        let p = programs.pop().unwrap();
                        programs.insert(0, p);
                    }
                }
                "x" => {
                    let (s1, s2) = (&mv[1..]).split_once('/').unwrap();
                    let n1 = s1.parse::<usize>().unwrap();
                    let n2 = s2.parse::<usize>().unwrap();

                    let tmp = programs[n1];
                    programs[n1] = programs[n2];
                    programs[n2] = tmp;
                }
                "p" => {
                    let (s1, s2) = (&mv[1..]).split_once('/').unwrap();
                    let c1: char = s1.chars().collect::<Vec<char>>()[0];
                    let c2: char = s2.chars().collect::<Vec<char>>()[0];
                    for i in 0..programs.len() {
                        if programs[i] == c1 {
                            programs[i] = c2;
                        } else if programs[i] == c2 {
                            programs[i] = c1;
                        }
                    }
                }
                _ => println!("Warning"),
            }
        }

        let mut porder = String::new();
        for j in 0..programs.len() {
            porder.push(programs[j]);
        }

        if hmap.contains_key(&porder) {
            let old = hmap.get(&porder).unwrap();
            let diff = itr - old;
            while itr + diff < ITERS {
                itr += diff;
            }
        } else {
            hmap.insert(porder, itr);
        }
        itr += 1;
    }

    for p in programs {
        print!("{}", p);
    }
    println!("");

    return result;
}

fn main() {
    println!("Day 16!");
    println!("{}", part1());
    println!("{}", part2());
}
