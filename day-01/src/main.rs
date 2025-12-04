const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> usize{
    movements().iter().fold((50, 0), |(dial, mut zeroes), amount| {
        let dial = (dial + amount).rem_euclid(100);        
        if dial == 0 {
            zeroes += 1
        };
        (dial, zeroes)
    }).1
}

fn part_2() -> usize {
    movements().iter().fold((50, 0), |(dial, zeroes), amount| {
        (0..amount.abs()).fold((dial, zeroes), |(mut dial, mut zeroes), _| {
            dial += amount.signum();
            dial = dial.rem_euclid(100);
            if dial == 0 {
                zeroes += 1;
            }
            (dial, zeroes)
        })
    }).1
}

fn movements() -> Vec<i32> {
    INPUT.lines()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse::<i32>().unwrap()))
        .map(|(c, amount)| if c == 'L' { -amount } else { amount })
        .collect()
}
