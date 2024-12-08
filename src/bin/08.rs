use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antenna: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] != '.' {
                antenna.entry(grid[r][c]).or_insert(Vec::new()).push((r as i32, c as i32));
            }
        }
    }
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (k, v) in antenna {
        let locs = v;
        for i in 0..locs.len() {
            for j in i+1..locs.len() {
                let dx = locs[j].0 - locs[i].0;
                let dy = locs[j].1 - locs[i].1;
                let nr1 = locs[j].0 + dx;
                let nc1 = locs[j].1 + dy;
                if is_in_bounds(&grid, nr1, nc1) {
                    antinodes.insert((nr1, nc1));
                }
                let nr2 = locs[i].0 - dx;
                let nc2 = locs[i].1 - dy;
                if is_in_bounds(&grid, nr2, nc2) {
                    antinodes.insert((nr2, nc2));
                }
            }
        }
    }
    Some(antinodes.len())
}

fn is_in_bounds(grid: &Vec<Vec<char>>, r: i32, c: i32) -> bool {
    r >= 0 && r < grid.len() as i32 && c >= 0 && c < grid[0].len() as i32
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antenna: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] != '.' {
                antenna.entry(grid[r][c]).or_insert(Vec::new()).push((r as i32, c as i32));
            }
        }
    }
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (k, v) in antenna {
        let locs = v;
        for i in 0..locs.len() {
            for j in i+1..locs.len() {
                antinodes.insert((locs[i].0, locs[i].1));
                antinodes.insert((locs[j].0, locs[j].1));
                let dx = locs[j].0 - locs[i].0;
                let dy = locs[j].1 - locs[i].1;
                let mut nr1 = locs[j].0 + dx;
                let mut nc1 = locs[j].1 + dy;
                while is_in_bounds(&grid, nr1, nc1) {
                    antinodes.insert((nr1, nc1));
                    nr1 += dx;
                    nc1 += dy;
                }
                let mut nr2 = locs[i].0 - dx;
                let mut nc2 = locs[i].1 - dy;
                while is_in_bounds(&grid, nr2, nc2) {
                    antinodes.insert((nr2, nc2));
                    nr2 -= dx;
                    nc2 -= dy;
                }
            }
        }
    }
    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
