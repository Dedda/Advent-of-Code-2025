const INPUT: &str = include_str!("input.txt");

type Location<T> = (T, T);

pub fn part_1() -> usize {
    p1(INPUT)
}

fn p1(input: &str) -> usize {
    let chars = char_map(input);
    accessible_rolls(&chars).len()
}

pub fn part_2() -> usize {
    p2(INPUT)
}

fn p2(input: &str) -> usize {
    let mut chars = char_map(input);
    let mut removed = 0;
    loop {
        let for_removal = accessible_rolls(&chars);
        if for_removal.is_empty() {
            break;
        }
        removed += for_removal.len();
        for (x, y) in for_removal {
            chars[y][x] = '.';
        }
    }
    removed
}

fn char_map(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn accessible_rolls(plan: &[Vec<char>]) -> Vec<Location<usize>> {
    plan.iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter()
            .enumerate()
            .filter(|(_, c)| **c == '@')
            .filter(move |(x, _)| is_accessible(*x, y, plan))
            .map(move |(x, _)| (x, y))
        ).collect::<Vec<_>>()
}

fn is_accessible(x: usize, y: usize, plan: &[Vec<char>]) -> bool {    
    neighbors(x, y, plan[0].len(), plan.len()).iter()
                .filter(|(nx, ny)| plan[*ny][*nx].eq(&'@'))
                .count() < 4
}

fn neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<Location<usize>> {
    let mut n = (-1..=1).flat_map(|dy| 
        (-1..=1).map(move |dx| (dx, dy)))
            .filter(|(dx, dy)| *dx != 0 || *dy != 0)
            .map(|(dx, dy)| clamp_location((x as isize + dx, y as isize + dy), width, height))
            .filter(|(nx, ny)| *nx != x || *ny != y)        
            .collect::<Vec<_>>();
    n.sort();
    n.dedup();
    n
}

fn clamp_location((x, y): Location<isize>, width: usize, height: usize) -> Location<usize> {
    (
        x.clamp(0, width as isize - 1) as usize,
        y.clamp(0, height as isize - 1) as usize
    )
}

#[cfg(test)]
mod tests {
    use crate::{p1, p2};

    const INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn part_1() {
        assert_eq!(13, p1(INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(43, p2(INPUT));
    }
}