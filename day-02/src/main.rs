const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> usize {
    ranges().into_iter().flat_map(|(first, last)| {
        (first..=last).filter(|id| {
            let s = id.to_string();
            let len = s.len();
            len % 2 == 0 && s[..len/2] == s[len/2..]
        })
    }).sum()
}

fn part_2() -> usize {
    ranges().into_iter()
        .flat_map(|(first, last)| (first..=last)
            .map(|i| format!("{i}"))
            .filter(|s| is_invalid_2(s)))            
        .map(|s| s.parse::<usize>().unwrap()).sum()
}

fn is_invalid_2(s: &str) -> bool {
    let len = s.len();
    (1..=len/2)
        .map(|i| &s[..i])
        .filter(|e| s.match_indices(e).nth(1).is_some())
        .map(|e| s.replace(e, ""))
        .any(|s| s.is_empty())
}

fn ranges() -> Vec<(usize, usize)> {
    INPUT.trim().split(',').map(|i| {
        let mut parts = i.split('-');
        (parts.next().unwrap().parse::<usize>().unwrap(), parts.next().unwrap().parse::<usize>().unwrap())
    }).collect()
}
