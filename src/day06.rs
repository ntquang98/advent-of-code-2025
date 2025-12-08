#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn calc_block_p1(matrix: &Vec<Vec<char>>, op: char, start_col: usize, end_col: usize) -> usize {
    let mut total = 0;

    for i in 0..matrix.len() - 1 {
        let mut current_num = Vec::new();
        for j in start_col..end_col {
            if matrix[i][j] != ' ' {
                current_num.push(matrix[i][j]);
            }
        }

        let num = current_num
            .iter()
            .collect::<String>()
            .trim()
            .parse::<usize>()
            .unwrap();

        total = match op {
            '+' => total + num,
            '*' => {
                if total == 0 {
                    num
                } else {
                    total * num
                }
            }
            _ => total,
        };
    }

    total
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> usize {
    let mut total = 0;

    let op_row: usize = input.len() - 1;
    let mut start_col = 0;
    let mut current_op = input[op_row][0];
    let mut next_op: char;
    for i in 1..input[op_row].len() {
        if input[op_row][i] == '+' || input[op_row][i] == '*' {
            next_op = input[op_row][i];
            total += calc_block_p1(&input, current_op, start_col, i);
            start_col = i;
            current_op = next_op;
        }
    }

    total += calc_block_p1(&input, current_op, start_col, input[0].len());

    total
}

fn calc_block_p2(matrix: &Vec<Vec<char>>, op: char, start_col: usize, end_col: usize) -> usize {
    let mut total = 0;

    for j in start_col..end_col {
        let mut current_num = Vec::new();
        for i in 0..matrix.len() - 1 {
            if matrix[i][j] != ' ' {
                current_num.push(matrix[i][j]);
            }
        }

        if current_num.is_empty() {
            continue;
        }

        let num = current_num
            .iter()
            .collect::<String>()
            .trim()
            .parse::<usize>()
            .unwrap();

        total = match op {
            '+' => total + num,
            '*' => {
                if total == 0 {
                    num
                } else {
                    total * num
                }
            }
            _ => total,
        };
    }

    total
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<Vec<char>>) -> usize {
    let mut total = 0;

    let op_row: usize = input.len() - 1;
    let mut start_col = 0;
    let mut current_op = input[op_row][0];
    let mut next_op: char;
    for i in 1..input[op_row].len() {
        if input[op_row][i] == '+' || input[op_row][i] == '*' {
            next_op = input[op_row][i];
            total += calc_block_p2(&input, current_op, start_col, i);
            start_col = i;
            current_op = next_op;
        }
    }

    total += calc_block_p2(&input, current_op, start_col, input[0].len());

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test calc block
    const INPUT_CALC_BLOCK_P1: &str =
        "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_calc_block_p1() {
        assert_eq!(
            calc_block_p1(&input_generator(INPUT_CALC_BLOCK_P1), '*', 0, 3),
            123 * 45 * 6
        );

        assert_eq!(
            calc_block_p1(&input_generator(INPUT_CALC_BLOCK_P1), '+', 4, 8),
            328 + 64 + 98
        );
    }

    #[test]
    fn test_calc_block_p2() {
        assert_eq!(
            calc_block_p2(&input_generator(INPUT_CALC_BLOCK_P1), '*', 0, 3),
            356 * 24 * 1
        );

        assert_eq!(
            calc_block_p2(&input_generator(INPUT_CALC_BLOCK_P1), '+', 4, 8),
            8 + 248 + 369
        );
    }

    const INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   + ";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 3263827);
    }
}
