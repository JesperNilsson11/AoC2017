use std::collections::HashMap;

fn part1() -> i64 {
    let input = include_str!("input8.txt").trim();
    let mut result = 0;

    let mut registers: HashMap<&str, i64> = HashMap::new();
    for line in input.split('\n') {
        let words: Vec<&str> = line.split_whitespace().collect();
        let reg = words[0];
        let op = words[1];
        let num: i64 = words[2].parse().unwrap();
        let cmpreg = words[4];
        let cmp = words[5];
        let cmpnum: i64 = words[6].parse().unwrap();

        let e = registers.entry(cmpreg).or_default();
        let mut do_op = false;
        match cmp {
            "==" => do_op = *e == cmpnum,
            "!=" => do_op = *e != cmpnum,
            "<=" => do_op = *e <= cmpnum,
            ">=" => do_op = *e >= cmpnum,
            "<" => do_op = *e < cmpnum,
            ">" => do_op = *e > cmpnum,
            _ => ()
        }

        if do_op {
            if op == "inc" {
                *registers.entry(reg).or_default() += num;
            } else {
                *registers.entry(reg).or_default() -= num;
            }
        }
    }

    for &v in registers.values() {
        if v > result {
            result = v;
        }
    }
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input8.txt").trim();
    let mut result = 0;

    let mut registers: HashMap<&str, i64> = HashMap::new();
    for line in input.split('\n') {
        let words: Vec<&str> = line.split_whitespace().collect();
        let reg = words[0];
        let op = words[1];
        let num: i64 = words[2].parse().unwrap();
        let cmpreg = words[4];
        let cmp = words[5];
        let cmpnum: i64 = words[6].parse().unwrap();

        let e = registers.entry(cmpreg).or_default();
        let mut do_op = false;
        match cmp {
            "==" => do_op = *e == cmpnum,
            "!=" => do_op = *e != cmpnum,
            "<=" => do_op = *e <= cmpnum,
            ">=" => do_op = *e >= cmpnum,
            "<" => do_op = *e < cmpnum,
            ">" => do_op = *e > cmpnum,
            _ => ()
        }

        if do_op {
            let e = registers.entry(reg).or_default();
            if op == "inc" {
                *e += num;
            } else {
                *e -= num;
            }

            if *e > result {
                result = *e;
            }
        }
    }
    

    return result;
}

fn main() {
    println!("Day 8!");
    println!("{}", part1());
    println!("{}", part2());
}
