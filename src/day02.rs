pub struct Range {
    start: usize,
    end: usize,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Range> {
    input
        .split(",")
        .map(|pattern| {
            let (s, e) = pattern.trim().split_once('-').unwrap();
            let start = s.parse::<usize>().expect(&format!("Can't parse {}", s));
            let end = e.parse::<usize>().expect(&format!("Can't parse {}", e));
            Range { start, end }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Range>) -> usize {
    fn is_valid_id(id: usize) -> bool {
        let id_str = id.to_string();
        if id_str.len() % 2 != 0 {
            return true;
        }

        let (first, second) = id_str.split_at(id_str.len() / 2);
        first != second
    }

    let mut total = 0;
    for range in input {
        for id in range.start..=range.end {
            if !is_valid_id(id) {
                total += id;
            }
        }
    }
    total
}

fn is_repeated_substring(s: &str) -> bool {
    // A string consists of the same pattern repeated multiple times
    // if and only if the string is a nontrivial rotation of itself.
    let doubled = [s, s].concat();
    // create nontrivial rotation zone
    doubled[1..doubled.len() - 1]
        // if the string appear inside the nontrivial rotation zone, it is repeated
        .contains(s)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Range>) -> usize {
    let mut total = 0;

    fn is_valid_id(id: usize) -> bool {
        let id_str = id.to_string();
        !is_repeated_substring(&id_str)
    }

    for range in input {
        for id in range.start..=range.end {
            if !is_valid_id(id) {
                total += id;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 4174379265);
    }

    // test is_repeated_substring
    #[test]
    fn test_is_repeated_substring() {
        assert!(is_repeated_substring("1111"));
        assert!(is_repeated_substring("1212"));
        assert!(is_repeated_substring("123123"));
        assert!(is_repeated_substring("12341234"));
        assert!(!is_repeated_substring("123"));
        assert!(!is_repeated_substring("12341233"));
    }
}
