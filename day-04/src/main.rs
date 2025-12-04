const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> usize {
    let chars = INPUT.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    accessible_rolls(&chars).len()
}

fn part_2() -> usize {
    let mut chars = INPUT.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
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

fn accessible_rolls(plan: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut accessible = vec![];
    let height = plan.len();
    for (y, line) in plan.iter().enumerate() {
        let width = line.len();
        for (x, c) in line.iter().enumerate() {
            if c.eq(&'@') {
                let neighbor_rolls = neighbors(x, y, width, height).iter()
                    .filter(|(nx, ny)| plan[*ny][*nx].eq(&'@'))
                    .count();
                if neighbor_rolls < 4 {
                    accessible.push((x, y));
                }
            }
        }
    }
    accessible
}

fn neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut n = vec![];
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx != 0 || dy != 0 {
                let nx = (x as isize + dx).clamp(0, width as isize - 1);
                let ny = (y as isize + dy).clamp(0, height as isize - 1);
                if nx != x as isize || ny != y as isize {
                    n.push((nx as usize, ny as usize));
                }
            }
        }
    }
    n.sort();
    n.dedup();
    n
}