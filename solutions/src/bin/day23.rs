use std::collections::HashMap;

fn getval(input: &str, registers: &HashMap<String, i64>) -> i64 {
    if input.chars().next().unwrap().is_alphabetic() == false {
        return input.parse().unwrap();
    }

    return *registers.get(input).unwrap();
}

fn part1() -> i64 {
    let input = include_str!("input23.txt").trim();
    let mut result = 0;
    let program: Vec<&str> = input.split('\n').collect();

    let mut pc: i64 = 0;
    let mut registers: HashMap<String, i64> = HashMap::new();
    for c in 'a'..'z' {
        registers.insert(c.to_string(), 0);
    }

    while pc >= 0 && pc < program.len() as i64 {
        let op = &program[pc as usize][0..3];
        let reg = (&program[pc as usize][4..5]).to_string();

        match op {
            "set" => {
                let y = getval(&program[pc as usize][6..], &registers);
                *registers.get_mut(&reg).unwrap() = y;
            }
            "sub" => {
                let x = *registers.get(&reg).unwrap();
                let y = getval(&program[pc as usize][6..], &registers);
                *registers.get_mut(&reg).unwrap() = x - y;
            }
            "mul" => {
                let x = *registers.get(&reg).unwrap();
                let y = getval(&program[pc as usize][6..], &registers);
                *registers.get_mut(&reg).unwrap() = x * y;
                result += 1;
            }
            "jnz" => {
                let val = getval(&reg, &registers);
                if val != 0 {
                    let y = getval(&program[pc as usize][6..], &registers);
                    pc += y;
                } else {
                    pc += 1;
                }
            }
            _ => println!("Warning"),
        }

        if op != "jnz" {
            pc += 1;
        }
    }

    return result;
}

fn part2() -> i64 {
    let mut result = 0;
    
    let mut a: i64 = 1;

    let mut b: i64 = 84;
    let mut c: i64 = b;

    let mut d: i64 = 0;
    let mut e: i64 = 0;
    let mut f: i64 = 0;
    let mut g: i64 = 0;
    let mut h: i64 = 0;

    if a != 0 {
        b *= 100;
        b -= -100000;
        c = b;
        c -= -17000;
    }

    loop {
        f = 1;
        d = 2;

        loop {
            e = 2;

            /*
            loop {
                g = d;
                g *= e;
                g -= b;
                if g == 0 {
                    f = 0
                }
                e -= -1;
                g = e;
                g -= b;

                if g == 0 {
                    break;
                }
            }
            */
            if b % d == 0 {
                f = 0;
            }
            e = b;
            if d * (e-1) == b {
                f = 0;
            }

            d -= -1;
            g = d;
            g -= b;
            if g == 0 {
                break;
            }
        }
        if f == 0 {
            h -= -1
        }
        g = b;
        g -= c;
        if g == 0 {
            break;
        }
        b -= -17;
    }

    result = h;

    return result;
}

fn main() {
    println!("Day 23!");
    println!("{}", part1());
    println!("{}", part2());
}
