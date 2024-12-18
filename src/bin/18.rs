use std::collections::{HashSet, VecDeque};
use advent_of_code::get_nums_usize;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    let num_rows =  71; // 7
    let num_col = num_rows;
    let mut grid: Vec<Vec<char>> = vec![vec!['.' ;num_col]; num_rows];
    let mut count =  1024; // 12
    for line in input.lines() {
        let nums = get_nums_usize(line);
        grid[nums[1]][nums[0]] = '#';
        count -= 1;
        if count == 0 {
            break
        }
    }

    bfs(&grid, num_rows, num_col)
}

pub fn part_two(input: &str) -> Option<String> {
    let num_rows =  71; // 7
    let num_col = num_rows;
    let mut s = 1024; // 12
    let mut e = 3450;
    while s < e {
        let m = (s + e)/2;
        let mut count = m;
        let mut grid: Vec<Vec<char>> = vec![vec!['.' ;num_col]; num_rows];
        for line in input.lines() {
            let nums = get_nums_usize(line);
            grid[nums[1]][nums[0]] = '#';
            count -= 1;
            if count == 0 {
                break
            }
        }
        if bfs(&grid, num_rows, num_col).is_none() {
            e = m;
        } else {
            s = m + 1;
        }
    }

    for line in input.lines() {
        s -= 1;
        if s == 0 {
            return Some(line.to_string());
        }
    }

    None
}

fn bfs(grid: &Vec<Vec<char>>, num_rows: usize, num_col: usize) -> Option<usize> {
    const DIRECTIONS: [(isize,isize); 4] = [(-1,0),(0,1),(1,0),(0,-1)];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((0,0));
    let mut level = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::from([(0,0)]);
    while q.len() > 0 {
        let count = q.len();
        for _ in 0..count {
            let (r, c) = q.pop_front().unwrap();
            if r == num_rows - 1 && c == num_col - 1 {
                return Some(level)
            }
            for dir in DIRECTIONS {
                let nr = r.wrapping_add_signed(dir.0);
                let nc = c.wrapping_add_signed(dir.1);
                if nr == usize::MAX || nr == num_rows || nc == usize::MAX || nc == num_col ||
                    visited.contains(&(nr, nc)) || grid[nr][nc] == '#' {
                    continue
                }
                q.push_back((nr,nc));
                visited.insert((nr, nc));
            }
        }
        level += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(22));
    // }
    //
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some("6,1".to_owned()));
    // }
}
