#[derive(Debug, Clone)]
pub struct Range {
    start: usize,
    end: usize,
}

pub struct Database {
    valid_ranges: Vec<Range>,
    ids: Vec<usize>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Database {
    let mut valid_ranges = Vec::new();
    let mut ids = Vec::new();
    let mut read_range = true;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            read_range = false;
            continue;
        }

        if read_range {
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse().unwrap();
            let end = parts[1].parse().unwrap();
            valid_ranges.push(Range { start, end });
        } else {
            ids.push(line.parse().unwrap());
        }
    }

    Database { valid_ranges, ids }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Database) -> i32 {
    let mut total = 0;

    for id in &input.ids {
        for range in &input.valid_ranges {
            if *id >= range.start && *id <= range.end {
                total += 1;
                break;
            }
        }
    }

    total
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Database) -> usize {
    let mut total = 0;
    let mut valid_ranges = input.valid_ranges.clone();
    valid_ranges.sort_by_key(|r| r.start);

    let mut last_end = 0;
    for range in valid_ranges {
        // because we sorted the ranges by start
        // we can assume that the next range will start after the last start
        // possible situation
        // 1. a - b not overlapped
        // 2. a - b overlapped - b end after a
        // 3. a - b overlapped - a completely overlaps b

        // a completely overlaps b
        if range.start < last_end && range.end <= last_end {
            continue;
        }

        // a - b not overlapped
        if range.start > last_end {
            total += range.end - range.start + 1;
        }

        // a - b overlapped - b end after a
        if range.start <= last_end && range.end > last_end {
            total += range.end - last_end;
        }

        last_end = range.end;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 14);
    }

    const INPUT2: &str = "3-5
    10-14
    12-14
    12-14
    14-20

    1
    5
    8
    11
    17
    32";

    #[test]
    fn test_part2_new() {
        assert_eq!(solve_part2(&input_generator(INPUT2)), 14);
    }
}
