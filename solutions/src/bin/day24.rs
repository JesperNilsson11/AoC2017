fn dfs(next: i64, components: Vec<(i64, i64)>) -> i64 {
    let mut result = 0;

    for i in 0..components.len() {
        let c = components[i];
        if c.0 == next {
            let mut newcomps = components.clone();
            newcomps.remove(i);
            let tmp = c.0 + c.1 + dfs(c.1, newcomps);
            if result < tmp {
                result = tmp;
            }
        } else if c.1 == next {
            let mut newcomps = components.clone();
            newcomps.remove(i);
            let tmp = c.0 + c.1 + dfs(c.0, newcomps);
            if result < tmp {
                result = tmp;
            }
        }
    }

    return result;
}

fn dfs2(next: i64, length: i64, components: Vec<(i64, i64)>) -> (i64, i64) {
    let mut result = (0, length);

    for i in 0..components.len() {
        let c = components[i];
        if c.0 == next {
            let mut newcomps = components.clone();
            newcomps.remove(i);
            let mut tmp = dfs2(c.1, length + 1, newcomps);
            tmp.0 += c.0 + c.1;
            if result.1 < tmp.1 || (result.1 == tmp.1 && result.0 < tmp.0) {
                result = tmp;
            }
        } else if c.1 == next {
            let mut newcomps = components.clone();
            newcomps.remove(i);
            let mut tmp = dfs2(c.0, length + 1, newcomps);
            tmp.0 += c.0 + c.1;
            if result.1 < tmp.1 || (result.1 == tmp.1 && result.0 < tmp.0) {
                result = tmp;
            }
        }
    }

    return result;
}

fn part1() -> i64 {
    let input = include_str!("input24.txt").trim();
    let mut result = 0;

    let mut components = Vec::new();
    for line in input.split('\n') {
        let (l, r) = line.split_once('/').unwrap();

        let c1: i64 = l.parse().unwrap();
        let c2: i64 = r.parse().unwrap();
        components.push((c1, c2));
    }

    result = dfs(0, components);
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input24.txt").trim();
    let mut result = 0;

    let mut components = Vec::new();
    for line in input.split('\n') {
        let (l, r) = line.split_once('/').unwrap();

        let c1: i64 = l.parse().unwrap();
        let c2: i64 = r.parse().unwrap();
        components.push((c1, c2));
    }

    result = dfs2(0, 0, components).0;
    

    return result;
}

fn main() {
    println!("Day 24!");
    println!("{}", part1());
    println!("{}", part2());
}
