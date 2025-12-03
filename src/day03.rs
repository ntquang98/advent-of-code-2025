#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.trim().to_string()).collect()
}

fn get_batteries(bank_line: String) -> u32 {
    let mut largest = 0;

    let digits: Vec<u32> = bank_line.chars().filter_map(|c| c.to_digit(10)).collect();

    for i in 0..digits.len() {
        for j in i + 1..digits.len() {
            let value = digits[i] * 10 + digits[j];
            if value > largest {
                largest = value;
            }
        }
    }

    largest
}

fn max_joltage_k(bank: String, k: usize) -> String {
    let digits: Vec<u8> = bank.bytes().map(|b| b - b'0').collect();
    let mut stack: Vec<u8> = Vec::with_capacity(k);
    let mut drop = digits.len().saturating_sub(k);

    for &d in &digits {
        while drop > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
            stack.pop();
            drop -= 1;
        }

        stack.push(d);
    }

    stack.truncate(k);

    stack.into_iter().map(|d| (d + b'0') as char).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<String>) -> u32 {
    let mut total = 0;

    for line in input {
        let batteries = get_batteries(line.clone());
        total += batteries;
    }

    total
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<String>) -> usize {
    let mut total = 0;

    for line in input {
        let max_joltage = max_joltage_k(line.clone(), 12);
        let max_joltage = max_joltage.parse::<usize>().unwrap();
        total += max_joltage;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
    811111111111119
    234234234234278
    818181911112111";

    #[test]
    fn test_get_batteries() {
        assert_eq!(get_batteries("987654321111111".to_string()), 98);
        assert_eq!(get_batteries("811111111111119".to_string()), 89);
        assert_eq!(get_batteries("234234234234278".to_string()), 78);
        assert_eq!(get_batteries("818181911112111".to_string()), 92);
    }

    #[test]
    fn test_max_joltage_k() {
        assert_eq!(
            max_joltage_k("987654321111111".to_string(), 12),
            "987654321111"
        );
        assert_eq!(
            max_joltage_k("811111111111119".to_string(), 12),
            "811111111119"
        );
        assert_eq!(
            max_joltage_k("234234234234278".to_string(), 12),
            "434234234278"
        );
        assert_eq!(
            max_joltage_k("818181911112111".to_string(), 12),
            "888911112111"
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 3121910778619);
    }
}
