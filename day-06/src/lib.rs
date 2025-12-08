const INPUT: &str = include_str!("input.txt");

pub fn part_1() -> usize {
    p1(INPUT)
}

fn p1(input: &str) -> usize {
    let parsed = parse_p1(input);
    parsed.iter().map(|(nums, o)| {
        match o {
            '+' => nums.iter().sum(),
            '*'=> nums.iter().product(),
            _ => 0,
        }
    }).sum()
}

fn parse_p1(input: &str) -> Vec<(Vec<usize>, char)> {
    let mut grid = input.lines().map(|line| line.split_ascii_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();
    let operands = grid.pop().unwrap().iter().map(|o| o.chars().next().unwrap()).collect::<Vec<_>>();
    operands.into_iter().enumerate().map(|(i, o)| {
        let nums = (0..grid.len()).map(|j| grid[j][i].parse().unwrap()).collect();
        (nums, o)
    }).collect::<Vec<_>>()
}

pub fn part_2() -> usize {
    p2(INPUT)
}

fn p2(input: &str) -> usize {
    let parsed = parse_p2(input);
    parsed.iter().map(|(nums, o)| {
        match o {
            '+' => nums.iter().sum(),
            '*'=> nums.iter().product(),
            _ => 0,
        }
    }).sum()
}

fn parse_p2(input: &str) -> Vec<(Vec<usize>, char)> {
    let mut lines = input.lines().collect::<Vec<_>>();
    let operands = lines.pop().unwrap().split_ascii_whitespace().map(|o| o.chars().next().unwrap()).collect::<Vec<_>>();
    let longest = lines.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap().len();
    let s = (0..longest).map(|i| lines.iter().map(|line| line.chars().nth(i).unwrap_or(' ')).collect::<String>()).collect::<Vec<_>>();
    let mut acc = vec![];
    let mut total = vec![];
    for i in s {
        if i.trim().is_empty() {
            total.push(acc.clone());
            acc.clear();
        } else {
            acc.push(i);
        }
    }
    total.push(acc.clone());

    // let s = s.into_iter().fold((vec![], vec![]), |(mut acc, mut total), i| {
    //     if i.trim().is_empty() {
    //         total.push(acc);
    //         (vec![], total)
    //     } else {
    //         acc.push(i);
    //         (acc, total)
    //     }
    // }).1.iter().map(|n| n.iter().map(|n| n.trim().parse().unwrap()).collect()).collect::<Vec<_>>();
    let s = total.iter().map(|n| n.iter().map(|n| n.trim().parse().unwrap()).collect()).collect::<Vec<_>>();
    // println!("{s:?}");
    s.into_iter().zip(operands).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::{p1, p2};

    const INPUT: &str = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#;

    #[test]
    pub fn part_1() {
        assert_eq!(4277556, p1(INPUT));
    }

    #[test]
    pub fn part_2() {
        assert_eq!(3263827, p2(INPUT));
    }
}