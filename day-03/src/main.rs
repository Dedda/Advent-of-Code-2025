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
    INPUT.lines()
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect())
        .collect()
}

fn strongest_n_ordered(n: usize, bank: &[usize]) -> usize {    
    (1..=n).fold((0, bank), |(mut acc, mut remaining), power| {
        let power = n - power;        
        let max = remaining[..remaining.len() - power].iter().max().unwrap();
        remaining = &remaining[remaining.iter().position(|n| n == max).unwrap()+1..];
        acc += max * 10_usize.pow(power as u32);
        (acc, remaining)
    }).0    
}
