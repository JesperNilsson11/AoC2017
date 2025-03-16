use std::collections::HashMap;

fn getval(input: &str, registers: &HashMap<String, i64>) -> i64 {
    if input.chars().next().unwrap().is_alphabetic() == false {
        return input.parse().unwrap();
    }

    return *registers.get(input).unwrap();
}

fn part1() -> i64 {
    let input = include_str!("input18.txt").trim();
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
            "snd" => {
                let x = registers.entry(reg).or_default();
                result = *x;
            }
            "add" => {
                let x = *registers.get(&reg).unwrap();
                let y = getval(&program[pc as usize][6..], &registers);
                *registers.get_mut(&reg).unwrap() = x + y;
            }
            "mul" => {
                let x = *registers.get(&reg).unwrap();
                let y = getval(&program[pc as usize][6..], &registers);
                *registers.get_mut(&reg).unwrap() = x * y;
            }
            "mod" => {
                let x = *registers.get(&reg).unwrap();
                let y = getval(&program[pc as usize][6..], &registers);
                *registers.get_mut(&reg).unwrap() = x % y;
            }
            "rcv" => {
                if *registers.get(&reg).unwrap() != 0 {
                    break;
                }
            }
            "jgz" => {
                if *registers.get(&reg).unwrap() > 0 {
                    let y = getval(&program[pc as usize][6..], &registers);
                    pc += y;
                } else {
                    pc += 1;
                }
            }
            _ => println!("Warning"),
        }

        if op != "jgz" {
            pc += 1;
        }
    }

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input18.txt").trim();
    let mut result = 0;
    let program: Vec<&str> = input.split('\n').collect();

    let mut pc: i64 = 0;
    let mut pc2: i64 = 0;
    let mut registers: HashMap<String, i64> = HashMap::new();
    let mut registers2: HashMap<String, i64> = HashMap::new();
    for c in 'a'..'z' {
        registers.insert(c.to_string(), 0);
        registers2.insert(c.to_string(), 0);
    }

    registers.insert("p".to_string(), 0);
    registers2.insert("p".to_string(), 1);

    let mut queue = Vec::new();
    let mut queue2 = Vec::new();

    let mut deadlock: i64 = 0;
    while deadlock != 2 {
        deadlock = 0;
        if pc >= 0 && pc < program.len() as i64 {
            let op = &program[pc as usize][0..3];
            let reg = (&program[pc as usize][4..5]).to_string();
            match op {
                "set" => {
                    let y = getval(&program[pc as usize][6..], &registers);
                    *registers.get_mut(&reg).unwrap() = y;
                }
                "snd" => {
                    let x = getval(&reg, &registers);
                    queue.push(x);
                }
                "add" => {
                    let x = *registers.get(&reg).unwrap();
                    let y = getval(&program[pc as usize][6..], &registers);
                    *registers.get_mut(&reg).unwrap() = x + y;
                }
                "mul" => {
                    let x = *registers.get(&reg).unwrap();
                    let y = getval(&program[pc as usize][6..], &registers);
                    *registers.get_mut(&reg).unwrap() = x * y;
                }
                "mod" => {
                    let x = *registers.get(&reg).unwrap();
                    let y = getval(&program[pc as usize][6..], &registers);
                    *registers.get_mut(&reg).unwrap() = x % y;
                }
                "rcv" => {
                    if queue2.len() > 0 {
                        let q = queue2.remove(0);
                        *registers.get_mut(&reg).unwrap() = q;
                    } else {
                        pc -= 1;
                        deadlock += 1;
                    }
                }
                "jgz" => {
                    let val = getval(&reg, &registers);
                    if val > 0 {
                        let y = getval(&program[pc as usize][6..], &registers);
                        pc += y;
                    } else {
                        pc += 1;
                    }
                }
                _ => println!("Warning"),
            }

            if op != "jgz" {
                pc += 1;
            }
        } else {
            deadlock += 1;
        }

        if pc2 >= 0 && pc2 < program.len() as i64 {
            let op = &program[pc2 as usize][0..3];
            let reg = (&program[pc2 as usize][4..5]).to_string();
            match op {
                "set" => {
                    let y = getval(&program[pc2 as usize][6..], &registers2);
                    *registers2.get_mut(&reg).unwrap() = y;
                }
                "snd" => {
                    let x = getval(&reg, &registers2);
                    queue2.push(x);
                    result += 1;
                }
                "add" => {
                    let x = *registers2.get(&reg).unwrap();
                    let y = getval(&program[pc2 as usize][6..], &registers2);
                    *registers2.get_mut(&reg).unwrap() = x + y;
                }
                "mul" => {
                    let x = *registers2.get(&reg).unwrap();
                    let y = getval(&program[pc2 as usize][6..], &registers2);
                    *registers2.get_mut(&reg).unwrap() = x * y;
                }
                "mod" => {
                    let x = *registers2.get(&reg).unwrap();
                    let y = getval(&program[pc2 as usize][6..], &registers2);
                    *registers2.get_mut(&reg).unwrap() = x % y;
                }
                "rcv" => {
                    if queue.len() > 0 {
                        let q = queue.remove(0);
                        *registers2.get_mut(&reg).unwrap() = q;
                    } else {
                        pc2 -= 1;
                        deadlock += 1;
                    }
                }
                "jgz" => {
                    let val = getval(&reg, &registers2);
                    if val > 0 {
                        let y = getval(&program[pc2 as usize][6..], &registers2);
                        pc2 += y;
                    } else {
                        pc2 += 1;
                    }
                }
                _ => println!("Warning"),
            }

            if op != "jgz" {
                pc2 += 1;
            }
        } else {
            deadlock += 1;
        }
    }

    return result;
}

fn main() {
    println!("Day 18!");
    println!("{}", part1());
    println!("{}", part2());
}
