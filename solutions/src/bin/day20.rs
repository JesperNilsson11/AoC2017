use std::collections::HashSet;

fn part1() -> i64 {
    let input = include_str!("input20.txt").trim();
    let mut result = 0;

    let mut particles = Vec::new();
    let mut id = 0;
    for line in input.split('\n') {
        let (l, r) = line.split_once('<').unwrap();
        let (l, r) = r.split_once('>').unwrap();
        let p: Vec<i64> = l.split(',').map(|x| x.parse::<i64>().unwrap()).collect();

        let (l, r) = r.split_once('<').unwrap();
        let (l, r) = r.split_once('>').unwrap();
        let v: Vec<i64> = l.split(',').map(|x| x.parse::<i64>().unwrap()).collect();

        let (l, r) = r.split_once('<').unwrap();
        let (l, r) = r.split_once('>').unwrap();
        let a: Vec<i64> = l.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        particles.push((p, v, a, id));
        id += 1;
    }

    particles.sort_by(|a,b| {
        let mut adist = a.2[0].abs() + a.2[1].abs() + a.2[2].abs();
        let mut bdist = b.2[0].abs() + b.2[1].abs() + b.2[2].abs();

        if adist != bdist {
            return adist.cmp(&bdist);
        }

        adist = a.1[0].abs() + a.1[1].abs() + a.1[2].abs();
        bdist = b.1[0].abs() + b.1[1].abs() + b.1[2].abs();
        if adist != bdist {
            return adist.cmp(&bdist);
        }

        adist = a.0[0].abs() + a.0[1].abs() + a.0[2].abs();
        bdist = b.0[0].abs() + b.0[1].abs() + b.0[2].abs();
        return adist.cmp(&bdist);
    });

    result = particles[0].3;

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input20.txt").trim();
    let mut result = 0;

    let mut particles = Vec::new();
    for line in input.split('\n') {
        let (l, r) = line.split_once('<').unwrap();
        let (l, r) = r.split_once('>').unwrap();
        let p: Vec<i64> = l.split(',').map(|x| x.parse::<i64>().unwrap()).collect();

        let (l, r) = r.split_once('<').unwrap();
        let (l, r) = r.split_once('>').unwrap();
        let v: Vec<i64> = l.split(',').map(|x| x.parse::<i64>().unwrap()).collect();

        let (l, r) = r.split_once('<').unwrap();
        let (l, r) = r.split_once('>').unwrap();
        let a: Vec<i64> = l.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        particles.push((p, v, a));
    }
    
    // TODO: better/always correct solution
    for _ in 0..1000 {
        for p in &mut particles {
            for i in 0..3 {
                p.1[i] += p.2[i];
                p.0[i] += p.1[i];
            }
        }
        let mut remove = Vec::new();
        let mut set = HashSet::new();
        for i in 0..particles.len() {
            for j in i+1..particles.len() {
                if particles[i].0 == particles[j].0 {
                    if set.contains(&i) == false {
                        set.insert(i);
                        remove.push(i);
                    }
                    if set.contains(&j) == false {
                        set.insert(j);
                        remove.push(j);
                    }
                }
            }
        }

        remove.sort();
        remove.reverse();
        for r in remove {
            particles.remove(r);
        }
    }

    result = particles.len() as i64;
    return result;
}

fn main() {
    println!("Day 20!");
    println!("{}", part1());
    println!("{}", part2());
}
