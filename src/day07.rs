use std::collections::HashSet;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> usize {
    let mut total = 0;

    let mut set: HashSet<usize> = HashSet::new();
    for i in 0..input.len() {
        if set.is_empty() {
            for j in 0..input[i].len() {
                if input[i][j] == 'S' {
                    set.insert(j);
                    break;
                }
            }
        } else {
            let mut new_set = HashSet::new();
            for j in set.iter().cloned() {
                if input[i][j] == '.' {
                    new_set.insert(j);
                    continue;
                }

                if input[i][j] == '^' {
                    total += 1;
                    if j > 0 {
                        new_set.insert(j - 1);
                    }

                    if j + 1 < input[i].len() {
                        new_set.insert(j + 1);
                    }
                }
            }
            set = new_set;
        }
    }

    total
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<Vec<char>>) -> usize {
    let mut total = 0;

    let mut active_cols: HashSet<usize> = HashSet::new();
    let mut times = Vec::new();
    for i in 0..input[0].len() {
        if input[0][i] == 'S' {
            times.push(1);
            active_cols.insert(i);
        } else {
            times.push(0);
        }
    }

    for i in 1..input.len() {
        let mut new_cols = HashSet::new();
        for j in active_cols.iter().cloned() {
            if input[i][j] == '.' {
                new_cols.insert(j);
                continue;
            }

            if input[i][j] == '^' {
                if j > 0 {
                    new_cols.insert(j - 1);
                    times[j - 1] += times[j];
                }

                if j + 1 < input[i].len() {
                    new_cols.insert(j + 1);
                    times[j + 1] += times[j];
                }
                times[j] = 0;
            }
        }

        active_cols = new_cols;
    }

    for time in times {
        total += time;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".......S.......
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
    ...............";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 40);
    }
}
