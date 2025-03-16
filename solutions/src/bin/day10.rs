fn part1() -> i64 {
    let input = include_str!("input10.txt").trim();
    let mut result = 0;

    let mut list = Vec::new();
    for i in 0..256 {
        list.push(i);
    }

    let mut skip = 0;
    let mut pos = 0;
    let mut len = list.len() as i64;
    for l in input.split(",").map(|x| x.parse::<i64>().unwrap()) {
        for i in 0..l / 2 {
            let idx1 = ((pos + i) % len) as usize;
            let idx2 = ((pos + l - i - 1) % len) as usize;
            let tmp = list[idx1];
            list[idx1] = list[idx2];
            list[idx2] = tmp;
        }

        pos += l + skip;
        pos %= len;
        skip += 1;
    }

    result = list[0] * list[1];

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input10.txt").trim();
    let mut result = 0;

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
                print!("{:02x}", tmp);
            }
            skip = false;
            tmp = list[i];
        } else {
            tmp ^= list[i];
        }
    }
    print!("{:02x}", tmp);
    println!("");

    return result;
}

fn main() {
    println!("Day 10!");
    println!("{}", part1());
    println!("{}", part2());
}
