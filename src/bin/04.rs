use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    const DIRECTIONS: [(i32, i32); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'X' {
                for dir in DIRECTIONS.iter() {
                    if is_valid(r as i32, c as i32, &grid, dir, "MAS") {
                        count += 1;
                    }
                }
            }
        }
    }
    Some(count)
}

fn is_valid(r: i32, c: i32, grid: &Vec<Vec<char>>, dir: &(i32, i32), to_match: &str) -> bool {
    let (dr, dc) = dir;
    for i in 1..=to_match.len() as i32 {
        let nr = r + i * dr;
        let nc = c + i * dc;

        if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 {
            return false
        }

        if grid[nr as usize][nc as usize] != to_match.as_bytes()[i as usize - 1] as char {
            return false
        }
    }
    true
}


pub fn part_two(input: &str) -> Option<usize> {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let mut a_s: HashMap<(i32, i32), usize> = HashMap::new();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'M' {
                for dir in DIRECTIONS.iter() {
                    if is_valid(r as i32, c as i32, &grid, dir, "AS") {
                        *a_s.entry((r as i32 + dir.0, c as i32 + dir.1)).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    let count = a_s.iter().filter(|(_, &v)| v == 2).count();
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
