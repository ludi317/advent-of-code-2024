use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '0' {
                let mut nines: HashSet<(usize, usize)> = HashSet::new();
                dfs(&grid, r, c, &mut nines);
                sum += nines.len();
            }
        }
    }
    Some(sum)
}


const DIRECTIONS: [(isize,isize); 4] = [(-1,0),(0,1),(1,0),(0,-1)];
fn dfs(grid: &Vec<Vec<char>>, r: usize, c: usize, nines: &mut HashSet<(usize, usize)>) {
    if grid[r][c] == '9' {
        nines.insert((r,c));
        return;
    }
    for dir in DIRECTIONS.iter() {
        let nr = r.wrapping_add_signed(dir.0);
        let nc = c.wrapping_add_signed(dir.1);
        if nr == usize::MAX || nr == grid.len() || nc == usize::MAX || nc == grid[0].len()||
            grid[nr][nc] as u8 != grid[r][c] as u8 + 1 {
            continue;
        }
        dfs(grid, nr, nc, nines)
    }
}

fn dfs2(grid: &Vec<Vec<char>>, r: usize, c: usize) -> usize {
    if grid[r][c] == '9' {
        return 1
    }
    let mut ans = 0;
    for dir in DIRECTIONS.iter() {
        let nr = r.wrapping_add_signed(dir.0);
        let nc = c.wrapping_add_signed(dir.1);
        if nr == usize::MAX || nr == grid.len() || nc == usize::MAX || nc == grid[0].len()||
            grid[nr][nc] as u8 != grid[r][c] as u8 + 1 {
            continue;
        }
        ans += dfs2(grid, nr, nc);
    }
    ans
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '0' {
                sum += dfs2(&grid, r, c);
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
