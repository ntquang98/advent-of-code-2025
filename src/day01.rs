pub enum Dir {
    Left,
    Right,
}

const STARTING_POSITION: isize = 50;
const NUMS: isize = 100;
const MAX_POSITION: isize = 99;
const MIN_POSITION: isize = 0;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<(Dir, isize, usize)> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.trim().split_at(1);
            (
                match l.chars().next() {
                    Some('L') => Dir::Left,
                    Some('R') => Dir::Right,
                    _ => panic!("Invalid direction"),
                },
                r.parse::<isize>().unwrap() % NUMS as isize,
                r.parse::<usize>().unwrap() / NUMS as usize,
            )
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<(Dir, isize, usize)>) -> usize {
    let mut dial = STARTING_POSITION;
    let mut count = 0;

    for (dir, steps, _) in input {
        match dir {
            Dir::Left => dial -= steps,
            Dir::Right => dial += steps,
        }

        if dial > MAX_POSITION {
            dial -= 100;
        } else if dial < MIN_POSITION {
            dial += 100;
        }

        if dial == 0 {
            count += 1;
        }
    }

    count
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<(Dir, isize, usize)>) -> usize {
    let mut dial = STARTING_POSITION;
    let mut count = 0;

    for (dir, steps, spins) in input {
        let mut should_add_spins = false;
        let dial_start = dial;

        match dir {
            Dir::Left => dial -= steps,
            Dir::Right => dial += steps,
        }

        if dial > MAX_POSITION {
            dial -= NUMS;
            should_add_spins = true;
        } else if dial < MIN_POSITION {
            dial += NUMS;
            if dial_start != MIN_POSITION {
                should_add_spins = true;
            }
        }

        if spins > &0 {
            count += spins;
        }

        if dial == MIN_POSITION || should_add_spins {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 3);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 6);
    }
}
