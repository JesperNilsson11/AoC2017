use std::collections::HashSet;

fn calc_hash(input: String) -> Vec<u8> {
    let mut result = Vec::new();

    let mut list = Vec::new();
    for i in 0..=255 {
        list.push(i);
    }

    let mut skip: usize = 0;
    let mut pos: usize = 0;
    let len = list.len();

    let mut lengths: Vec<u8> = input.bytes().collect();
    lengths.push(17);
    lengths.push(31);
    lengths.push(73);
    lengths.push(47);
    lengths.push(23);

    for _ in 0..64 {
        for &ll in &lengths {
            let l = ll as usize;
            for i in 0..l / 2 {
                let idx1 = (pos + i) % len;
                let idx2 = (pos + l - i - 1) % len;
                let tmp = list[idx1];
                list[idx1] = list[idx2];
                list[idx2] = tmp;
            }

            pos += l + skip;
            pos %= len;
            skip += 1;
        }
    }

    let mut tmp: u8 = 0;
    let mut skip = true;
    for i in 0..=255 {
        if i % 16 == 0 {
            if skip == false {
                result.push(tmp);
            }
            skip = false;
            tmp = list[i];
        } else {
            tmp ^= list[i];
        }
    }
    result.push(tmp);

    return result;
}

fn part1() -> i64 {
    let mut result = 0;

    let basekey = "wenycdww-".to_string();
    for i in 0..128 {
        let key = basekey.clone() + &i.to_string();
        let bits = calc_hash(key);

        for mut b in bits {
            while b > 0 {
                result += (1 & b) as i64;
                b = b >> 1;
            }
        }
    }
    
    return result;
}

fn part2() -> i64 {
    let mut result = 0;

    let basekey = "wenycdww-".to_string();
    let mut active = HashSet::new();
    for i in 0..128 {
        let key = basekey.clone() + &i.to_string();
        let mut bits = calc_hash(key);

        let mut x = 0;
        for b in bits {
            for bb in [128, 64, 32, 16, 8, 4, 2, 1] {
                if bb & b > 0 {
                    active.insert((x, i));
                }
                x += 1;
            }
        }
    }

    let mut visited = HashSet::new();
    for y in 0..128 {
        for x in 0..128 {
            if active.contains(&(x, y)) == false {
                continue;
            }
            if visited.contains(&(x, y)) {
                continue;
            }

            result += 1;
            let mut queue = Vec::new();
            queue.push((x, y));
            visited.insert((x, y));
            while queue.len() > 0 {
                let mut newqueue = Vec::new();
                for q in &queue {
                    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        let nx = q.0 + dx;
                        let ny = q.1 + dy;

                        if nx < 0 || nx >= 128 || ny < 0 || ny >= 128 {
                            continue;
                        }

                        if active.contains(&(nx, ny)) == false {
                            continue;
                        }

                        if visited.contains(&(nx, ny)) {
                            continue;
                        }
                        visited.insert((nx, ny));
                        newqueue.push((nx, ny));
                    }
                }

                queue = newqueue;
            }
        }
    }
    
    return result;
}

fn main() {
    println!("Day 14!");
    println!("{}", part1());
    println!("{}", part2());
}
