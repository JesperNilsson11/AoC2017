fn part1() -> i64 {
    let input = include_str!("input9.txt").trim();
    let mut result = 0;

    let mut skip = false;
    let mut left = 0;
    let mut garbage = false;
    for c in input.chars() {
        if skip {
            skip = false;
            continue;
        }

        if c == '<' {
            garbage = true;
        }
        if garbage {
            if c == '!' {
                skip = true;
            }
            if c == '>' {
                garbage = false;
            }
        } else {
            if c == '{' {
                left += 1;
                result += left;
            }
            if c == '}' {
                left -= 1;
                if left < 0 {
                    println!("Warning");
                }
            }
        }
    }

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input9.txt").trim();
    let mut result = 0;

    let mut skip = false;
    let mut left = 0;
    let mut garbage = false;
    for c in input.chars() {
        if skip {
            skip = false;
            continue;
        }

        if garbage {
            if c == '!' {
                skip = true;
            } else if c == '>' {
                garbage = false;
            } else {
                result += 1;
            }
        } else {
            if c == '<' {
                garbage = true;
            }
            if c == '{' {
                left += 1;
            }
            if c == '}' {
                left -= 1;
                if left < 0 {
                    println!("Warning");
                }
            }
        }
    }

    return result;
}

fn main() {
    println!("Day 9!");
    println!("{}", part1());
    println!("{}", part2());
}
