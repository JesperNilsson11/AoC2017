use std::collections::HashMap;

fn part1() -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut moves_left = 325489 - 1;
    let mut dist = 1;
    while moves_left > 0 {
        let mut curr_dist = dist;
        if curr_dist > moves_left {
            curr_dist = moves_left;
        }
        for _ in 0..curr_dist {
            x += 1;
            moves_left -= 1;
        }

        curr_dist = dist;
        if curr_dist > moves_left {
            curr_dist = moves_left;
        }
        for _ in 0..curr_dist {
            y -= 1;
            moves_left -= 1;
        }

        dist += 1;
        curr_dist = dist;
        if curr_dist > moves_left {
            curr_dist = moves_left;
        }
        for _ in 0..curr_dist {
            x -= 1;
            moves_left -= 1;
        }

        curr_dist = dist;
        if curr_dist > moves_left {
            curr_dist = moves_left;
        }
        for _ in 0..curr_dist {
            y += 1;
            moves_left -= 1;
        }

        dist += 1;
    }
    println!("{} {}", x, y);
    return x.abs() + y.abs();
}

fn calc(x: i64, y: i64, hmap: &mut HashMap<(i64, i64), i64>) -> i64 {
    let mut res = 0;

    for (dx, dy) in [(1,0),(-1,0),(-1,-1),(1,1), (1,-1), (-1,1), (0,1), (0,-1)] {
        let nx = x + dx;
        let ny = y + dy;

        if hmap.contains_key(&(nx, ny)) {
            res += hmap.get(&(nx, ny)).unwrap();
        }
    }

    hmap.insert((x, y), res);

    return res;
}

fn part2() -> i64 {
    let mut result = 0;
    let mut hmap = HashMap::new();
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut moves_left = 325489 - 1;
    let mut dist = 1;
    hmap.insert((x, y), 1);
    while result <= 325489 {
        let mut curr_dist = dist;
        if curr_dist > moves_left {
            curr_dist = moves_left;
        }
        for _ in 0..curr_dist {
            x += 1;
            moves_left -= 1;
            result = calc(x, y, &mut hmap);
            if result > 325489 {
                moves_left = 0;
                break;
            }
        }

        curr_dist = dist;
        if curr_dist > moves_left {
            curr_dist = moves_left;
        }
        for _ in 0..curr_dist {
            y -= 1;
            moves_left -= 1;
            result = calc(x, y, &mut hmap);
            if result > 325489 {
                moves_left = 0;
                break;
            }
        }

        dist += 1;
        curr_dist = dist;
        if curr_dist > moves_left {
            curr_dist = moves_left;
        }
        for _ in 0..curr_dist {
            x -= 1;
            moves_left -= 1;
            result = calc(x, y, &mut hmap);
            if result > 325489 {
                moves_left = 0;
                break;
            }
        }

        curr_dist = dist;
        if curr_dist > moves_left {
            curr_dist = moves_left;
        }
        for _ in 0..curr_dist {
            y += 1;
            moves_left -= 1;
            result = calc(x, y, &mut hmap);
            if result > 325489 {
                moves_left = 0;
                break;
            }
        }

        dist += 1;
    }

    return result;
}

fn main() {
    println!("Day 3!");
    println!("{}", part1());
    println!("{}", part2());
}
