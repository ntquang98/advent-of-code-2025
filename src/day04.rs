#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

const DIR: [(i32, i32); 8] = [
    (0, 1),   // up
    (1, 1),   // right-up
    (1, 0),   // right
    (1, -1),  // right-down
    (0, -1),  // down
    (-1, -1), // left-down
    (-1, 0),  // left
    (-1, 1),  // left-up
];

fn check_point(map: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    if map[y as usize][x as usize] != '@' {
        return false;
    }

    let mut neighbors = 0;

    for (dx, dy) in DIR {
        let nx = x + dx;
        let ny = y + dy;

        // out of map
        if nx < 0 || nx as usize >= map[0].len() || ny < 0 || ny as usize >= map.len() {
            continue;
        }

        // is roll of paper
        if map[ny as usize][nx as usize] == '@' {
            neighbors += 1;
        }
    }

    neighbors < 4
}

fn walk_matrix(map: &Vec<Vec<char>>) -> (i32, Vec<(i32, i32)>) {
    let mut total = 0;
    let mut visited = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if check_point(map, x as i32, y as i32) {
                total += 1;
                visited.push((x as i32, y as i32));
            }
        }
    }

    (total, visited)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> i32 {
    let (total, _) = walk_matrix(input);
    total
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    let mut map = input.clone();

    loop {
        let (nums, visited) = walk_matrix(&map);
        if nums == 0 {
            break;
        }

        for (x, y) in visited {
            map[y as usize][x as usize] = '.';
        }

        total += nums;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 43);
    }
}
