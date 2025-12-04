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
            .map(|i| (i, i.to_string()))
            .filter(|(_, s)| is_invalid_2(s)))            
        .map(|(i, _)| i).sum()
}

fn is_invalid_2(s: &str) -> bool {
    let s_len = s.len();
    (1..=s_len/2)
        .filter(|e_len| s_len.is_multiple_of(*e_len))            
        .any(|e_len| 
            (1..s_len/e_len).all(|n| s[..e_len] == s[n*e_len..(n+1)*e_len])
        )
}

fn ranges() -> Vec<(usize, usize)> {
    INPUT.trim().split(',').map(|i| {
        let mut parts = i.split('-');
        (parts.next().unwrap().parse::<usize>().unwrap(), parts.next().unwrap().parse::<usize>().unwrap())
    }).collect()
}
