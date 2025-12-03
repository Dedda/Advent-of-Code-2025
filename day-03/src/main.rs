const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> usize {
    banks().iter().map(|bank| strongest_n_ordered(2, bank)).sum()
}

fn part_2() -> usize {
    banks().iter().map(|bank| strongest_n_ordered(12, bank)).sum()
}

fn banks() -> Vec<Vec<usize>> {
    INPUT.lines().map(|line| line.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>())
            .collect()
}

fn strongest_n_ordered(n: usize, bank: &[usize]) -> usize {
    let mut acc = 0;
    let mut remaining = bank;
    for power in 1..n+1 {
        let power = n - power;
        let slice = &remaining[..remaining.len() - power];
        let max = slice.iter().max().unwrap();
        remaining = &remaining[remaining.iter().position(|n| n == max).unwrap()+1..];
        acc += max * 10_usize.pow(power as u32);
    }
    acc
}