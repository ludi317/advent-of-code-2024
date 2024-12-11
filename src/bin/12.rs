advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut cost = 0;
    for r in 0..grid.len() {
        for c in 0..grid.len() {
            if grid[r][c].is_ascii_uppercase() {
                let (area, perimeter) = dfs(&mut grid, r, c);
                cost += area * perimeter;
            }
        }
    }

    Some(cost)
}

// up, right, down, left
const DIRECTIONS: [(isize,isize); 4] = [(-1,0),(0,1),(1,0),(0,-1)];
const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
const LEFT: usize = 3;

const DIAG_DIRECTIONS: [(isize,isize); 8] = [(-1,0),(0,1),(1,0),(0,-1), (-1,1), (-1,-1), (1,1), (1,-1)];
const UP_RIGHT: usize = 4;
const UP_LEFT: usize = 5;
const DOWN_RIGHT: usize = 6;
const DOWN_LEFT: usize = 7;



fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize) -> (usize, usize) {
    let orig = grid[r][c];
    grid[r][c] = orig.to_ascii_lowercase();
    let mut area = 1;
    let mut perimeter = 0;
    for dir in DIRECTIONS.iter() {
        let nr = r.wrapping_add_signed(dir.0);
        let nc = c.wrapping_add_signed(dir.1);
        if nr == usize::MAX || nr == grid.len() || nc == usize::MAX || nc == grid[0].len() ||
            (grid[nr][nc] != orig && grid[nr][nc] != orig.to_ascii_lowercase() ){
            perimeter += 1;
            continue
        }
        if grid[nr][nc] != orig {
            continue
        }

        let (partial_area, partial_perimeter) = dfs(grid, nr, nc);
        area += partial_area;
        perimeter += partial_perimeter;
    }
    (area, perimeter)
}

fn dfs2(grid: &mut Vec<Vec<char>>, r: usize, c: usize) -> (usize, usize) {
    let orig = grid[r][c];
    grid[r][c] = orig.to_ascii_lowercase();
    let mut area = 1;
    let mut sides = 0;

    let mut is_outside_region: [bool; 8] = [false, false, false, false,false, false, false, false];
    for (i, dir) in DIAG_DIRECTIONS.iter().enumerate() {
        let nr = r.wrapping_add_signed(dir.0);
        let nc = c.wrapping_add_signed(dir.1);
        if nr == usize::MAX || nr == grid.len() || nc == usize::MAX || nc == grid[0].len() ||
            (grid[nr][nc] != orig && grid[nr][nc] != orig.to_ascii_lowercase() ) {
            is_outside_region[i] = true;
        }
    }
    if is_outside_region[UP] && is_outside_region[LEFT] {
        sides += 1;
    }
    if is_outside_region[UP] && is_outside_region[RIGHT] {
        sides += 1;
    }
    if is_outside_region[DOWN] && is_outside_region[LEFT] {
        sides += 1;
    }
    if is_outside_region[DOWN] && is_outside_region[RIGHT] {
        sides += 1;
    }
    if !is_outside_region[UP] && !is_outside_region[RIGHT] && is_outside_region[UP_RIGHT] {
        sides += 1;
    }
    if !is_outside_region[UP] && !is_outside_region[LEFT] && is_outside_region[UP_LEFT] {
        sides += 1;
    }
    if !is_outside_region[DOWN] && !is_outside_region[RIGHT] && is_outside_region[DOWN_RIGHT] {
        sides += 1;
    }
    if !is_outside_region[DOWN] && !is_outside_region[LEFT] && is_outside_region[DOWN_LEFT] {
        sides += 1;
    }



    for dir in DIRECTIONS.iter() {
        let nr = r.wrapping_add_signed(dir.0);
        let nc = c.wrapping_add_signed(dir.1);
        if nr == usize::MAX || nr == grid.len() || nc == usize::MAX || nc == grid[0].len() ||
            (grid[nr][nc] != orig && grid[nr][nc] != orig.to_ascii_lowercase() ) {
            continue
        }
        if grid[nr][nc] != orig {
            continue
        }
        let (partial_area, partial_sides) = dfs2(grid, nr, nc);
        area += partial_area;
        sides += partial_sides;
    }
    (area, sides)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut cost = 0;
    for r in 0..grid.len() {
        for c in 0..grid.len() {
            if grid[r][c].is_ascii_uppercase() {
                let (area, sides) = dfs2(&mut grid, r, c);
                cost += area * sides;
            }
        }
    }

    Some(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }
}
