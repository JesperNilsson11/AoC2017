use std::collections::HashMap;

fn flipx(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut newmap = map.clone();

    for y in 0..newmap.len() {
        let w = newmap[y].len();
        for x in 0..w / 2 {
            let tmp = newmap[y][x];
            newmap[y][x] = newmap[y][w - 1 - x];
            newmap[y][w - 1 - x] = tmp;
        }
    }

    return newmap;
}

fn flipy(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut newmap = map.clone();

    for x in 0..newmap[0].len() {
        let h = newmap.len();
        for y in 0..h / 2 {
            let tmp = newmap[y][x];
            newmap[y][x] = newmap[h - 1 - y][x];
            newmap[h - 1 - y][x] = tmp;
        }
    }

    return newmap;
}

fn rotate(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut newmap = map.clone();

    let h = newmap.len();
    let w = newmap[0].len();
    for y in 0..newmap.len() {
        for x in 0..newmap[y].len() {
            newmap[y][x] = map[x][h - 1 - y];
        }
    }

    return newmap;
}

fn part1() -> i64 {
    let input = include_str!("input21.txt").trim();
    let mut result = 0;

    let mut patterns = HashMap::new();
    for line in input.split('\n') {
        let mut pat = Vec::new();
        let mut curr_row = Vec::new();

        let (l, r) = line.split_once("=> ").unwrap();
        for c in l.chars() {
            if c == '/' {
                pat.push(curr_row);
                curr_row = Vec::new();
            } else if c == ' ' {
                pat.push(curr_row);
                break;
            } else {
                curr_row.push(c);
            }
        }

        let mut newpat = Vec::new();
        let mut curr_row = Vec::new();
        for c in r.chars() {
            if c == '/' {
                newpat.push(curr_row);
                curr_row = Vec::new();
            } else {
                curr_row.push(c);
            }
        }
        newpat.push(curr_row);

        let r = rotate(&pat);
        patterns.insert(flipx(&r), newpat.clone());
        patterns.insert(flipy(&r), newpat.clone());
        patterns.insert(r.clone(), newpat.clone());

        let r = rotate(&r);
        patterns.insert(flipx(&r), newpat.clone());
        patterns.insert(flipy(&r), newpat.clone());
        patterns.insert(r.clone(), newpat.clone());

        let r = rotate(&r);
        patterns.insert(flipx(&r), newpat.clone());
        patterns.insert(flipy(&r), newpat.clone());
        patterns.insert(r.clone(), newpat.clone());

        patterns.insert(flipx(&pat), newpat.clone());
        patterns.insert(flipy(&pat), newpat.clone());
        patterns.insert(pat, newpat);
    }

    let mut image = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];

    for _ in 0..5 {
        let mut newimage = Vec::new();

        if image.len() % 2 == 0 {
            let mut y = 0;
            while y < image.len() {
                let mut x = 0;
                let mut row1 = Vec::new();
                let mut row2 = Vec::new();
                let mut row3 = Vec::new();

                while x < image[y].len() {
                    let pattern = vec![
                        vec![image[y][x], image[y][x + 1]],
                        vec![image[y + 1][x], image[y + 1][x + 1]],
                    ];

                    let newpat = patterns.get(&pattern).unwrap();
                    row1.push(newpat[0][0]);
                    row1.push(newpat[0][1]);
                    row1.push(newpat[0][2]);

                    row2.push(newpat[1][0]);
                    row2.push(newpat[1][1]);
                    row2.push(newpat[1][2]);

                    row3.push(newpat[2][0]);
                    row3.push(newpat[2][1]);
                    row3.push(newpat[2][2]);
                    x += 2;
                }

                newimage.push(row1);
                newimage.push(row2);
                newimage.push(row3);

                y += 2;
            }
        } else if image.len() % 3 == 0 {
            let mut y = 0;
            while y < image.len() {
                let mut x = 0;
                let mut row1 = Vec::new();
                let mut row2 = Vec::new();
                let mut row3 = Vec::new();
                let mut row4 = Vec::new();

                while x < image[y].len() {
                    let pattern = vec![
                        vec![image[y + 0][x], image[y + 0][x + 1], image[y + 0][x + 2]],
                        vec![image[y + 1][x], image[y + 1][x + 1], image[y + 1][x + 2]],
                        vec![image[y + 2][x], image[y + 2][x + 1], image[y + 2][x + 2]]
                    ];

                    let newpat = patterns.get(&pattern).unwrap();
                    row1.push(newpat[0][0]);
                    row1.push(newpat[0][1]);
                    row1.push(newpat[0][2]);
                    row1.push(newpat[0][3]);

                    row2.push(newpat[1][0]);
                    row2.push(newpat[1][1]);
                    row2.push(newpat[1][2]);
                    row2.push(newpat[1][3]);

                    row3.push(newpat[2][0]);
                    row3.push(newpat[2][1]);
                    row3.push(newpat[2][2]);
                    row3.push(newpat[2][3]);

                    row4.push(newpat[3][0]);
                    row4.push(newpat[3][1]);
                    row4.push(newpat[3][2]);
                    row4.push(newpat[3][3]);
                    x += 3;
                }

                newimage.push(row1);
                newimage.push(row2);
                newimage.push(row3);
                newimage.push(row4);

                y += 3;
            }
        } else {
            println!("Warning image size");
            break;
        }

        image = newimage;
    }

    for row in image {
        for c in row {
            if c == '#' {
                result += 1;
            }
        }
    }

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input21.txt").trim();
    let mut result = 0;

    let mut patterns = HashMap::new();
    for line in input.split('\n') {
        let mut pat = Vec::new();
        let mut curr_row = Vec::new();

        let (l, r) = line.split_once("=> ").unwrap();
        for c in l.chars() {
            if c == '/' {
                pat.push(curr_row);
                curr_row = Vec::new();
            } else if c == ' ' {
                pat.push(curr_row);
                break;
            } else {
                curr_row.push(c);
            }
        }

        let mut newpat = Vec::new();
        let mut curr_row = Vec::new();
        for c in r.chars() {
            if c == '/' {
                newpat.push(curr_row);
                curr_row = Vec::new();
            } else {
                curr_row.push(c);
            }
        }
        newpat.push(curr_row);

        let r = rotate(&pat);
        patterns.insert(flipx(&r), newpat.clone());
        patterns.insert(flipy(&r), newpat.clone());
        patterns.insert(r.clone(), newpat.clone());

        let r = rotate(&r);
        patterns.insert(flipx(&r), newpat.clone());
        patterns.insert(flipy(&r), newpat.clone());
        patterns.insert(r.clone(), newpat.clone());

        let r = rotate(&r);
        patterns.insert(flipx(&r), newpat.clone());
        patterns.insert(flipy(&r), newpat.clone());
        patterns.insert(r.clone(), newpat.clone());

        patterns.insert(flipx(&pat), newpat.clone());
        patterns.insert(flipy(&pat), newpat.clone());
        patterns.insert(pat, newpat);
    }

    let mut image = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];

    for _ in 0..18 {
        let mut newimage = Vec::new();

        if image.len() % 2 == 0 {
            let mut y = 0;
            while y < image.len() {
                let mut x = 0;
                let mut row1 = Vec::new();
                let mut row2 = Vec::new();
                let mut row3 = Vec::new();

                while x < image[y].len() {
                    let pattern = vec![
                        vec![image[y][x], image[y][x + 1]],
                        vec![image[y + 1][x], image[y + 1][x + 1]],
                    ];

                    let newpat = patterns.get(&pattern).unwrap();
                    row1.push(newpat[0][0]);
                    row1.push(newpat[0][1]);
                    row1.push(newpat[0][2]);

                    row2.push(newpat[1][0]);
                    row2.push(newpat[1][1]);
                    row2.push(newpat[1][2]);

                    row3.push(newpat[2][0]);
                    row3.push(newpat[2][1]);
                    row3.push(newpat[2][2]);
                    x += 2;
                }

                newimage.push(row1);
                newimage.push(row2);
                newimage.push(row3);

                y += 2;
            }
        } else if image.len() % 3 == 0 {
            let mut y = 0;
            while y < image.len() {
                let mut x = 0;
                let mut row1 = Vec::new();
                let mut row2 = Vec::new();
                let mut row3 = Vec::new();
                let mut row4 = Vec::new();

                while x < image[y].len() {
                    let pattern = vec![
                        vec![image[y + 0][x], image[y + 0][x + 1], image[y + 0][x + 2]],
                        vec![image[y + 1][x], image[y + 1][x + 1], image[y + 1][x + 2]],
                        vec![image[y + 2][x], image[y + 2][x + 1], image[y + 2][x + 2]]
                    ];

                    let newpat = patterns.get(&pattern).unwrap();
                    row1.push(newpat[0][0]);
                    row1.push(newpat[0][1]);
                    row1.push(newpat[0][2]);
                    row1.push(newpat[0][3]);

                    row2.push(newpat[1][0]);
                    row2.push(newpat[1][1]);
                    row2.push(newpat[1][2]);
                    row2.push(newpat[1][3]);

                    row3.push(newpat[2][0]);
                    row3.push(newpat[2][1]);
                    row3.push(newpat[2][2]);
                    row3.push(newpat[2][3]);

                    row4.push(newpat[3][0]);
                    row4.push(newpat[3][1]);
                    row4.push(newpat[3][2]);
                    row4.push(newpat[3][3]);
                    x += 3;
                }

                newimage.push(row1);
                newimage.push(row2);
                newimage.push(row3);
                newimage.push(row4);

                y += 3;
            }
        } else {
            println!("Warning image size");
            break;
        }

        image = newimage;
    }

    for row in image {
        for c in row {
            if c == '#' {
                result += 1;
            }
        }
    }

    return result;
}

fn main() {
    println!("Day 21!");
    println!("{}", part1());
    println!("{}", part2());
}
