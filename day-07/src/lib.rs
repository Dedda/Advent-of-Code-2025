const INPUT: &str = include_str!("input.txt");

pub fn part_1() -> usize {
    p1(INPUT)
}

fn p1(input: &str) -> usize {
    let (beam, lines) = parse_manifold(input);
    let mut beams = vec![beam];
    let mut splits = 0;
    for (y, line) in lines.iter().enumerate() {
        if y % 2 == 0 {
            continue;
        }
        let mut new_beams = vec![];
        for beam in beams.into_iter() {
            if line[beam] {
                splits += 1;
                new_beams.push(beam - 1);
                new_beams.push(beam + 1);
            } else {
                new_beams.push(beam);
            }
        }
        // new_beams.sort();
        new_beams.dedup();
        beams = new_beams;
    }
    splits
}

pub fn part_2() -> usize {
    p2(INPUT)
}

fn p2(input: &str) -> usize {
    let (beam, lines) = parse_manifold(input);
    let width = lines[0].len();
    let mut beams = vec![0; width];
    beams[beam] = 1;
    for (y, line) in lines.iter().enumerate() {
        if y % 2 == 0 {
            continue;
        }
        for (idx, amount) in beams.clone().iter().enumerate() {
            if line[idx] {
                beams[idx] = 0;
                beams[idx - 1] += amount;
                beams[idx + 1] += amount;
            }
        }
    }
    beams.iter().sum()
}

fn parse_manifold(input: &str) -> (usize, Vec<Vec<bool>>) {
    let mut lines = input.lines();
    let beam = lines.next().unwrap().find('S').unwrap();
    (beam, lines.map(|line| line.chars().map(|c| c == '^').collect()).collect())
}

#[cfg(test)]
mod tests {
    use crate::{p1, p2};

    const INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    pub fn part_1() {
        assert_eq!(21, p1(INPUT));
    }

    #[test]
    pub fn part_2() {
        assert_eq!(40, p2(INPUT));
    }
}