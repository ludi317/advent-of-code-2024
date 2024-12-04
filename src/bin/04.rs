use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(4);

#[derive(Debug)]
struct Cell {
    row: i32,
    col: i32,
    ch: char,
    dir: (i32, i32),
}

pub fn part_one(input: &str) -> Option<u32> {
    const DIRECTIONS: [(i32, i32); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut q: VecDeque<Cell> = VecDeque::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'X' {
                q.push_back(Cell { row: r as i32, col: c as i32, ch: 'X', dir: (-2, -2)})
            }
        }
    }
    // bfs
    let mut count = 0;
    while let Some(cur) = q.pop_front() {
        let next_ch = match cur.ch {
            'X' => 'M',
            'M' => 'A',
            'A' => 'S',
            'S' => {
                count += 1;
                continue;
            }
            _ => continue,
        };

        if next_ch == 'M' {
            for &(dr, dc) in DIRECTIONS.iter() {
                let nr: i32 = cur.row + dr;
                let nc: i32 = cur.col + dc;
                if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 || grid[nr as usize][nc as usize] != next_ch
                {
                    continue;
                }
                q.push_back(Cell{row:nr, col:nc, ch: next_ch, dir: (dr, dc)})
            }
        } else {
            let nr: i32 = cur.row + cur.dir.0;
            let nc: i32 = cur.col + cur.dir.1;
            if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 || grid[nr as usize][nc as usize] != next_ch
            {
                continue;
            }
            q.push_back(Cell{row:nr, col:nc, ch: next_ch, dir: cur.dir})
        }

    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut q: VecDeque<Cell> = VecDeque::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'M' {
                q.push_back(Cell { row: r as i32, col: c as i32, ch: 'M', dir: (-2, -2)})
            }
        }
    }
    // bfs
    let mut a_s: HashMap<(i32, i32), usize> = HashMap::new();
    while let Some(cur) = q.pop_front() {
        let next_ch = match cur.ch {
            'M' => 'A',
            'A' => 'S',
            'S' => {
                *a_s.entry((cur.row - cur.dir.0, cur.col - cur.dir.1)).or_insert(0) += 1;
                continue;
            }
            _ => continue,
        };

        if next_ch == 'A' {
            for &(dr, dc) in DIRECTIONS.iter() {
                let nr: i32 = cur.row + dr;
                let nc: i32 = cur.col + dc;
                if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 || grid[nr as usize][nc as usize] != next_ch
                {
                    continue;
                }
                q.push_back(Cell{row:nr, col:nc, ch: next_ch, dir: (dr, dc)})
            }
        } else {
            let nr: i32 = cur.row + cur.dir.0;
            let nc: i32 = cur.col + cur.dir.1;
            if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 || grid[nr as usize][nc as usize] != next_ch
            {
                continue;
            }
            q.push_back(Cell{row:nr, col:nc, ch: next_ch, dir: cur.dir})
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
