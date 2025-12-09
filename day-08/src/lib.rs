use std::{str::FromStr, vec};

use util::ExpectParseNext;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct JunctionBox(isize, isize, isize);

impl FromStr for JunctionBox {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        Ok(Self(parts.e_parse_next(), parts.e_parse_next(), parts.e_parse_next()))
    }
}

impl JunctionBox {
    fn raw_distance_to(&self, other: &JunctionBox) -> isize {
        (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)
    }
}

#[derive(Clone)]
struct Network(Vec<usize>);

impl Network {
    fn contains(&self, idx: usize) -> bool {
        self.0.contains(&idx)
    }

    fn merge(&mut self, other: &mut Network) {
        self.0.append(&mut other.0);
        self.0.sort();
        self.0.dedup();
    }
}

pub fn part_1() -> usize {
    p1(INPUT, 1000)
}

fn p1(input: &str, n: usize) -> usize {
    let boxes = input.lines().map(|line| line.parse().unwrap()).collect::<Vec<JunctionBox>>();
    let closest = closest_n(&boxes, n);
    let mut networks: Vec<Network> = vec![];
    for (_, a, b) in closest {
        let pos_a = networks.iter().position(|n| n.contains(a));
        let pos_b = networks.iter().position(|n| n.contains(b));        
        if let Some(pos_a) = pos_a && let Some(pos_b) = pos_b {
            if pos_a == pos_b {
                continue;
            }
            let mut n1 = networks.remove(pos_a.max(pos_b));
            let n2 = networks.get_mut(pos_a.min(pos_b)).unwrap();
            n2.merge(&mut n1);
        } else if let Some(pos_a) = pos_a {
            let n = networks.get_mut(pos_a).unwrap();
            n.0.push(b);
        } else if let Some(pos_b) = pos_b {
            let n = networks.get_mut(pos_b).unwrap();
            n.0.push(a);
        } else {
            networks.push(Network(vec![a, b]));
        }
    }
    networks.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    networks[..3].iter().map(|n| n.0.len()).product()
}

fn closest_n(boxes: &[JunctionBox], n: usize) -> Vec<(isize, usize, usize)> {
    let mut closest = vec![];
    let mut max = (isize::MAX, 0, 0);
    for (idx_a, box_a) in boxes.iter().enumerate() {
        for (idx_b, box_b) in boxes.iter().enumerate().skip(idx_a) {            
            if idx_a == idx_b {
                continue;
            }
            let len = closest.len();
            let dist = box_a.raw_distance_to(box_b);
            if len < n {
                closest.push((dist, idx_a, idx_b));                
                if len == n {
                    closest.sort();
                    max = *closest.last().unwrap();
                }
            } else if dist < max.0 {
                closest.pop();
                closest.push((dist, idx_a, idx_b));
                closest.sort_by(|a, b| a.0.cmp(&b.0));
                max = *closest.last().unwrap();
            }
        }
    }
    closest
}

pub fn part_2() -> usize {
    p2(INPUT)
}

fn p2(input: &str) -> usize {
    let boxes = input.lines().map(|line| line.parse().unwrap()).collect::<Vec<JunctionBox>>();
    let (a, b) = last_connection(&boxes);
    (boxes[a].0 * boxes[b].0) as usize
}

fn last_connection(boxes: &[JunctionBox]) -> (usize, usize) {
    let len = boxes.len();
    let mut all_closest = closest_n(boxes, 1_000_000);
    all_closest.sort_by(|a, b| a.0.cmp(&b.0));
    let mut networks: Vec<Network> = vec![];
    for (_, a, b) in all_closest {
        let net_a = networks.iter().position(|n| n.contains(a));
        let net_b = networks.iter().position(|n| n.contains(b));        
        if let Some(net_a) = net_a && let Some(net_b) = net_b {
            if net_a == net_b {
                continue;
            }
            let mut n1 = networks.remove(net_a.max(net_b));
            let n2 = networks.get_mut(net_a.min(net_b)).unwrap();
            n2.merge(&mut n1);
            if n2.0.len() == len {
                return (a, b);
            }
        } else if let Some(net_a) = net_a {
            let n = networks.get_mut(net_a).unwrap();
            n.0.push(b);
            if n.0.len() == len {
                return (a, b);
            }
        } else if let Some(net_b) = net_b {
            let n = networks.get_mut(net_b).unwrap();
            n.0.push(a);
            if n.0.len() == len {
                return (a, b);
            }
        } else {
            networks.push(Network(vec![a, b]));
        }
    }
    networks.sort_by(|a, b| b.0.len().cmp(&a.0.len()));    
    (0, 0)
}

#[cfg(test)]
mod tests {
    use crate::{p1, p2};


    const INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    pub fn part_1() {
        assert_eq!(40, p1(INPUT, 10));
    }

    #[test]
    pub fn part_2() {
        assert_eq!(25272, p2(INPUT));
    }
}