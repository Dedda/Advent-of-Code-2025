const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> usize{
    let mut dial = 50;
    let mut zeroes = 0;
    for amount in movements() {
        dial += amount;
        dial = dial.rem_euclid(100);
        if dial == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

fn part_2() -> usize {
    let mut dial = 50;
    let mut zeroes = 0;
    for amount in movements() {
        match amount.signum() {
            1 => for _ in 0..amount {
                dial += 1;
                dial %= 100;
                if dial == 0 {
                    zeroes += 1;
                }
            },
            -1 => for _ in 0..amount.abs() {
                dial -= 1;
                if dial < 0 {
                    dial += 100;
                }
                if dial == 0 {
                    zeroes += 1;
                }
            },
            _ => {},
        }
    }
    zeroes
}

fn movements() -> Vec<i32> {
    INPUT.lines().map(|line| {
        let mut amount = line[1..].parse::<i32>().expect("input not a number!");
        if line.starts_with('L') {
            amount *= -1;
        }
        amount
    }).collect()
}