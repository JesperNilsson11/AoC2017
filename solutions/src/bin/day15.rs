fn part1() -> i64 {
    let mut result = 0;
    let mut A: i64 = 618;
    let mut B: i64 = 814;

    for _ in 0..40000000 {
        A = (A * 16807) % 2147483647;
        B = (B * 48271) % 2147483647;

        let a = A & ((1 << 16) - 1);
        let b = B & ((1 << 16) - 1);
        if a == b {
            result += 1;
        }
    }
    
    return result;
}

fn part2() -> i64 {
    let mut result = 0;
    let mut A: i64 = 618;
    let mut B: i64 = 814;

    for _ in 0..5000000 {
        A = (A * 16807) % 2147483647;
        while A % 4 > 0 {
            A = (A * 16807) % 2147483647;
        }
        B = (B * 48271) % 2147483647;
        while B % 8 > 0 {
            B = (B * 48271) % 2147483647;
        }

        let a = A & ((1 << 16) - 1);
        let b = B & ((1 << 16) - 1);
        if a == b {
            result += 1;
        }
    }
    
    return result;
}

fn main() {
    println!("Day 15!");
    println!("{}", part1());
    println!("{}", part2());
}
