const INPUT: &str = include_str!("input.txt");

pub fn part_1() -> usize {
    p1(INPUT)
}

fn p1(input: &str) -> usize {
    banks(input).iter().map(|bank| strongest_n_ordered(2, bank)).sum()
}

pub fn part_2() -> usize {
    p2(INPUT)
}

fn p2(input: &str) -> usize {
    banks(input).iter().map(|bank| strongest_n_ordered(12, bank)).sum()
}

fn banks(input: &str) -> Vec<Vec<usize>> {
    input.lines()
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

#[cfg(test)]
mod tests {
    use crate::{p1, p2};

    const INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn part_1() {
        assert_eq!(357, p1(INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(3121910778619, p2(INPUT));
    }
}