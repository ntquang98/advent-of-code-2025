use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
    z: usize,
}

pub type Pair = (usize, usize, usize);

pub struct Group {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Group {}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Point> {
    let points: Vec<_> = input
        .lines()
        .map(|line| {
            let mut pp = line.trim().split(",").map(|s| s.parse::<usize>().unwrap());
            Point {
                x: pp.next().unwrap(),
                y: pp.next().unwrap(),
                z: pp.next().unwrap(),
            }
        })
        .collect();

    points
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<Point>) -> usize {
    let mut heap: BinaryHeap<Pair> = BinaryHeap::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let dx = p1.x.abs_diff(p2.x);
            let dy = p1.y.abs_diff(p2.y);
            let dz = p1.z.abs_diff(p2.z);
            let distance = dx * dx + dy * dy + dz * dz;
            if heap.len() < 1000 {
                heap.push((distance, i, j));
            } else if distance < heap.peek().unwrap().0 {
                heap.pop();
                heap.push((distance, i, j));
            }
        }
    }

    let mut pairs: Vec<Pair> = heap
        .into_iter()
        .map(|(distance, i, j)| (distance, i, j))
        .collect();

    // sort by distance ascending
    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    let mut total = 0;

    total
}

// #[aoc(day8, part2)]
// pub fn solve_part2(input: &Vec<Vec<char>>) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "162,817,812
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
    425,690,689";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 40);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(solve_part2(&input_generator(INPUT)), 40);
    // }
}
