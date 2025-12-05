const INPUT: &str = include_str!("input.txt");

pub fn part_1() -> usize {
    p1(INPUT)
}

fn p1(input: &str) -> usize {
    ranges(input).into_iter().flat_map(|(first, last)| {
        (first..=last).filter(|id| {
            let s = id.to_string();
            let len = s.len();
            len % 2 == 0 && s[..len/2] == s[len/2..]
        })
    }).sum()
}

pub fn part_2() -> usize {
    p2(INPUT)
}

fn p2(input: &str) -> usize {
    ranges(input).into_iter()
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

fn ranges(input: &str) -> Vec<(usize, usize)> {
    input.trim().split(',').map(|i| {
        let mut parts = i.split('-');
        (parts.next().unwrap().parse::<usize>().unwrap(), parts.next().unwrap().parse::<usize>().unwrap())
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::{p1, p2};

    const INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn part_1() {
        assert_eq!(1227775554, p1(INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(4174379265, p2(INPUT));
    }
}