use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    const DIRECTIONS: [(i32,i32); 4] = [(-1,0),(0,1),(1,0),(0,-1)];
    let mut r: i32 = -1;
    let mut c: i32 = -1;
    'outer:for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '^' {
                r = row as i32;
                c = col as i32;
                break 'outer
            }
        }
    }
    let mut dir_idx = 0;
    let mut pos: HashSet<(i32, i32)> = HashSet::new();
    loop {
        pos.insert((r, c));
        let nr = r + DIRECTIONS[dir_idx].0;
        let nc = c + DIRECTIONS[dir_idx].1;
        if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 {
            break
        }
        if grid[nr as usize][nc as usize] == '#' {
            dir_idx = (dir_idx + 1)% DIRECTIONS.len();
        } else {
            r = nr;
            c = nc;
        }
    }

    Some(pos.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sr: i32 = -1;
    let mut sc: i32 = -1;
    'outer: for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '^' {
                sr = row as i32;
                sc = col as i32;
                break 'outer
            }
        }
    }
    let mut count = 0;
    for rr in 0..grid.len() {
        for cc in 0..grid[0].len() {
            if grid[rr][cc] == '.' {
                grid[rr][cc] = '#'
            } else {
                continue
            }
            let mut r = sr;
            let mut c = sc;
            const DIRECTIONS: [(i32,i32); 4] = [(-1,0),(0,1),(1,0),(0,-1)];

            let mut dir_idx = 0;
            let mut repeats = 0;
            let mut pos: HashSet<(i32, i32)> = HashSet::new();
            loop {
                let old_len = pos.len();
                pos.insert((r, c));
                let new_len = pos.len();
                if old_len == new_len {
                    repeats += 1;
                } else {
                    repeats = 0;
                }
                if repeats == 100 {
                    count += 1;
                    grid[rr][cc] = '.';
                    break
                }

                let nr = r + DIRECTIONS[dir_idx].0;
                let nc = c + DIRECTIONS[dir_idx].1;
                if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 {
                    grid[rr][cc] = '.';
                    break
                }
                if grid[nr as usize][nc as usize] == '#' {
                    dir_idx = (dir_idx + 1)% DIRECTIONS.len();
                } else {
                    r = nr;
                    c = nc;
                }
            }
        }
    }


    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
