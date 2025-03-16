use std::collections::HashMap;

fn part1() -> i64 {
    let input = include_str!("input25.txt").trim();
    let mut result = 0;
    let mut states = HashMap::new();
    
    let (l, r) = input.split_once("\n\n").unwrap();
    let mut curr_state = &l[15..16];
    let dig = &l[54..];
    let diagnostics: i64 = dig.split(' ').next().map(|x| x.parse().unwrap()).unwrap();
    for desc in r.split("\n\n") {
        let lines: Vec<&str> = desc.split('\n').collect();
        let s = &lines[0][9..10];
        let write1;
        let write2;
        let dx1;
        let dx2;
        let s1;
        let s2;

        if lines[1] != "  If the current value is 0:" {
            println!("Warning");
        }
        if lines[2] == "    - Write the value 1." {
            write1 = 1;
        } else {
            write1 = 0;
        }
        if lines[3] == "    - Move one slot to the right." {
            dx1 = 1;
        } else {
            dx1 = -1;
        }
        s1 = &lines[4][26..27];

        if lines[5] != "  If the current value is 1:" {
            println!("Warning");
        }
        if lines[6] == "    - Write the value 1." {
            write2 = 1;
        } else {
            write2 = 0;
        }
        if lines[7] == "    - Move one slot to the right." {
            dx2 = 1;
        } else {
            dx2 = -1;
        }
        s2 = &lines[8][26..27];

        println!("State: {}, if 0 ({} {} {}), if 1 ({} {} {})", s, write1, dx1, s1, write2, dx2, s2);
        states.insert(s, (write1, dx1, s1, write2, dx2, s2));
    }
    
    let mut mem = HashMap::new();
    let mut pos = 0;
    for _ in 0..diagnostics {
        let e = mem.entry(pos).or_insert(0);
        let state = states.get(curr_state).unwrap();

        if *e == 0 {
            *e = state.0;
            pos += state.1;
            curr_state = state.2;
        } else {
            *e = state.3;
            pos += state.4;
            curr_state = state.5;
        }
    }

    for (k, v) in mem {
        if v == 1 {
            result += 1;
        }
    }

    return result;
}

fn main() {
    println!("Day 25!");
    println!("{}", part1());
}
