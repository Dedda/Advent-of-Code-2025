use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone)]
struct FreshRange(usize, usize);

impl FromStr for FreshRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-').map(|x| x.parse::<usize>().unwrap());
        Ok(Self(parts.next().unwrap(), parts.next().unwrap()))
    }
}

impl FreshRange {
    fn contains(&self, x: usize) -> bool {
        self.0 <= x && x <= self.1
    }

    fn overlaps(&self, range: &FreshRange) -> bool {
        !(self.0 > range.1 || range.0 > self.1)
    }

    fn merge(&self, other: &FreshRange) -> FreshRange {
        FreshRange(self.0.min(other.0), self.1.max(other.1))
    }
}

pub fn part_1() -> usize {
    p1(INPUT)
}

fn p1(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let ranges = parse_ranges(parts.next().unwrap());
    let ingredients = parts.next().unwrap().lines().map(|x| x.parse::<usize>().unwrap());
    ingredients.filter(|x| ranges.iter().any(|r| r.contains(*x))).count()
}

pub fn part_2() -> usize {
    p2(INPUT)
}

fn p2(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let mut ranges = parse_ranges(parts.next().unwrap());
    ranges.sort();
    let ranges = ranges.into_iter().fold(Vec::<FreshRange>::new(), |mut acc: Vec<_>, r| {
        if let Some(last) = acc.last() && last.overlaps(&r) {
            let last = acc.pop().unwrap();
            acc.push(r.merge(&last));
            acc
        } else {
            acc.push(r.clone());
            acc
        }
    });
    ranges.iter().map(|r| r.1-r.0+1).sum::<usize>()
}

fn parse_ranges(input: &str) -> Vec<FreshRange> {
    input.lines().map(|x| x.parse::<FreshRange>().unwrap()).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::{p1, p2};

    const INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

    #[test]
    fn part_1() {
        assert_eq!(3, p1(INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(14, p2(INPUT));
    }
}
