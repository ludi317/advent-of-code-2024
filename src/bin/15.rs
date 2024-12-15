use std::collections::HashSet;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    let mut part1 = true;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut dirs: Vec<char> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            part1 = false;
            continue
        }
        if part1 {
            grid.push(line.chars().collect());
        } else {
            dirs.extend(line.chars());
        }

    }

    for g in &grid {
        println!("{:?}", g);
    }
    println!("{:?}", dirs);
    println!();
    let mut r = 0;
    let mut c = 0;
    for rr in 0..grid.len() {
        for cc in 0..grid[0].len() {
            if grid[rr][cc] == '@' {
                r = rr;
                c = cc;
                grid[rr][cc] = '.';
            }
        }
    }

    'outer: for d in dirs {
        grid[r][c] = '@';
        println!("{d}");
        for g in &grid {
            println!("{:?}", g);
        }
        grid[r][c] = '.';
        let dir =
        match d {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1,0),
            '<' => (0,-1),
            _ => panic!("invalid direction")
        };
        let mut box_count = 0;
        for m in 1..100 {
            let nr = r.wrapping_add_signed(m * dir.0);
            let nc = c.wrapping_add_signed(m * dir.1);
            // hit a wall before going out of bounds
            match grid[nr][nc] {
                '.' => {
                    r = r.wrapping_add_signed(dir.0);
                    c = c.wrapping_add_signed(dir.1);

                    if box_count > 0 {
                        grid[r][c] = '.';
                        grid[nr][nc] = 'O';
                    }

                    continue 'outer;
                },
                'O' => {
                    box_count+= 1;
                    continue;
                },
                '#' => {
                    continue 'outer;
                },
                _ => {
                    panic!()
                }
            }
        }
    }

    let mut sum = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'O' {
                sum += 100 * r + c;
            }
        }

    }

    Some(sum)

}

pub fn part_two(input: &str) -> Option<usize> {

    let mut part1 = true;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut dirs: Vec<char> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            part1 = false;
            continue
        }
        if part1 {
            let mut exp_line: Vec<char> = Vec::new();
            for ch in line.chars() {
                match ch {
                    '#' => {
                        exp_line.extend("##".chars());
                    },
                    'O' => {
                        exp_line.extend("[]".chars());
                    },
                    '.' => {
                        exp_line.extend("..".chars());
                    },
                    '@' => {
                        exp_line.extend("@.".chars());
                    }
                    _ => panic!()
                }
            }
            grid.push(exp_line);
        } else {
            dirs.extend(line.chars());
        }

    }

    for g in &grid {
        println!("{:?}", g);
    }
    println!("{:?}", dirs);
    println!();
    let mut r = 0;
    let mut c = 0;
    for rr in 0..grid.len() {
        for cc in 0..grid[0].len() {
            if grid[rr][cc] == '@' {
                r = rr;
                c = cc;
                grid[rr][cc] = '.';
            }
        }
    }

    'outer: for d in dirs {
        grid[r][c] = '@';
        println!("{d}");
        for g in &grid {
            println!("{:?}", g.iter().collect::<String>());
        }
        grid[r][c] = '.';
        let dir =
            match d {
                '^' => (-1, 0),
                '>' => (0, 1),
                'v' => (1,0),
                '<' => (0,-1),
                _ => panic!("invalid direction")
            };
        if d == '>' || d == '<' {
            let mut box_side_count = 0;
            for m in 1..100 {
                let nr = r.wrapping_add_signed(m * dir.0);
                let nc = c.wrapping_add_signed(m * dir.1);
                // hit a wall before going out of bounds
                match grid[nr][nc] {
                    '.' => {
                        r = r.wrapping_add_signed(dir.0);
                        c = c.wrapping_add_signed(dir.1);


                        let mut rr = nr;
                        let mut cc = nc;
                        for _ in 0..box_side_count {
                            let prev_r = rr.wrapping_add_signed(-dir.0);
                            let prev_c = cc.wrapping_add_signed(-dir.1);
                            match grid[prev_r][prev_c] {
                                '[' => grid[rr][cc] = '[',
                                ']' => grid[rr][cc] = ']',
                                _ => unreachable!()
                            }
                            rr = prev_r;
                            cc = prev_c;
                        }
                        if box_side_count > 0 {
                            grid[r][c] = '.';
                        }

                        continue 'outer;
                    },
                    '[' => {
                        box_side_count += 1;
                        continue;
                    },
                    ']' => {
                        box_side_count += 1;
                        continue;
                    }
                    '#' => {
                        continue 'outer;
                    },
                    _ => {
                        unreachable!()
                    }
                }
            }
        } else {
            // up or down
                let nr = r.wrapping_add_signed(dir.0);
                let nc = c.wrapping_add_signed(dir.1);
                // hit a wall before going out of bounds
                match grid[nr][nc] {
                    '.' => {
                        r = nr;
                        c = nc;
                    },
                    '[' | ']' => {
                        let mut box_pos = HashSet::new();
                        // if there is space,
                        if dfs(&grid, nr, nc, dir, &mut box_pos) {
                            // erase all the boxes
                            for theBox in &box_pos {
                                grid[theBox.0][theBox.1] = '.';
                            }
                            // then move them one unit
                            for theBox in box_pos {
                                let moved_r = theBox.0.wrapping_add_signed(dir.0);
                                let moved_c = theBox.1.wrapping_add_signed(dir.1);
                                grid[moved_r][moved_c] = theBox.2;
                            }

                            // and move the robot
                            r = nr;
                            c = nc;
                        }

                    },
                    '#' => {
                       // no op
                    },
                    _ => {
                        unreachable!()
                    }
            }

        }
    }
    for g in &grid {
        println!("{:?}", g.iter().collect::<String>());
    }

    let mut sum = 0;
    for r in 0..grid.len() {
        let mut c = 0;
        while c < grid[0].len() {
            if grid[r][c] == '[' {
                sum += 100 * r + c;
                c += 1;
            }
            c += 1;
        }
    }

    Some(sum)
}

fn dfs(grid: &Vec<Vec<char>>, r: usize, c: usize, dir: (isize, isize), box_set: &mut HashSet<(usize, usize, char)>) -> bool {
    assert!(dir == (1,0) || dir == (-1,0));
    assert!(grid[r][c] == '[' || grid[r][c] == ']');
    if box_set.contains(&(r, c, grid[r][c])) {
        return true
    }
    box_set.insert((r, c, grid[r][c]));
    let mut ans = true;
    if grid[r][c] == '[' {
        ans &= dfs(grid, r, c+1, dir, box_set);
    } else {
        ans &= dfs(grid, r, c-1, dir, box_set);
    }

    let nr = r.wrapping_add_signed(dir.0);
    let nc = c.wrapping_add_signed(dir.1);

    // check the space above
    if grid[nr][nc] == '#' {
        return false;
    }
    if grid[nr][nc] == '[' || grid[nr][nc] == ']' {
        ans &= dfs(grid, nr, nc, dir, box_set)
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(9021));
    }
}
