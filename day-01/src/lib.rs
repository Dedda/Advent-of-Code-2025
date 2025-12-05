const INPUT: &str = include_str!("input.txt");

pub fn part_1() -> usize {
    p1(INPUT)
}

fn p1(input: &str) -> usize {
    movements(input).iter().fold((50, 0), |(dial, mut zeroes), amount| {
        let dial = (dial + amount).rem_euclid(100);
        if dial == 0 {
            zeroes += 1
        };
        (dial, zeroes)
    }).1
}

pub fn part_2() -> usize {
    p2(INPUT)
}

fn p2(input: &str) -> usize {
    movements(input).iter().fold((50, 0), |(dial, zeroes), amount| {
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

fn movements(input: &str) -> Vec<i32> {
    input.lines()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse::<i32>().unwrap()))
        .map(|(c, amount)| if c == 'L' { -amount } else { amount })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{p1, p2};

    const INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn part_1() {
        assert_eq!(3, p1(INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(6, p2(INPUT));
    }
}