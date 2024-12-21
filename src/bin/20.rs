use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 2)
}

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];


// dist is r X c of dists from [start, end]

fn bfs(grid: &Vec<Vec<char>>, idx: usize, dist: &mut Vec<Vec<Vec<usize>>>, origin: (usize, usize)) {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    q.push_back(origin);
    visited.insert(origin);
    let mut levels = 0;
    while q.len() > 0 {
        let count = q.len();
        for _ in 0..count {
            let cur = q.pop_front().unwrap();
            dist[cur.0][cur.1][idx] = levels;
            for dir in DIRECTIONS {
                let nr = cur.0.wrapping_add_signed(dir.0);
                let nc = cur.1.wrapping_add_signed(dir.1);
                if visited.contains(&(nr, nc)) {
                    continue
                }
                if grid[nr][nc] != '#' {
                    q.push_back((nr, nc));
                    visited.insert((nr, nc));
                }
            }
        }
        levels += 1;

    }

}

fn solve(input: &str, cheat_len: isize) -> Option<usize> {
    const SAVED_THRESHOLD: usize = 100;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let unreachable = grid.len() * grid[0].len();
    let mut dist: Vec<Vec<Vec<usize>>> = vec![vec![vec![unreachable; 2]; grid[0].len()]; grid.len()];

    // find start and end positions
    let mut start = (0,0);
    let mut end = (0,0);
    'outer: for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'S' {
                start = (r,c);
            } else if grid[r][c] == 'E' {
                end = (r,c);
                break 'outer;
            }
        }
    }

    // populate distances from start
    bfs(&grid, 0, &mut dist, start);
    // populates distances from end
    bfs(&grid, 1, &mut dist, end);

    let base = dist[end.0][end.1][0];

    let mut freqs: HashMap<usize, usize> = HashMap::new();
    let mut tot_count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] != '#' {
                let r = r as isize;
                let c = c as isize;
                for rr in (r - cheat_len).max(0)..= (r + cheat_len).min((grid.len() - 1) as isize) {
                    for cc in (c - cheat_len).max(0)..= (c + cheat_len).min((grid[0].len() - 1) as isize) {
                        let r = r as usize;
                        let c = c as usize;
                        let rr = rr as usize;
                        let cc = cc as usize;
                        if grid[rr][cc] == '#' {continue} // must land on track
                        let l1_dist = rr.abs_diff(r) + cc.abs_diff(c);
                        if l1_dist <= cheat_len as usize {
                            // dist from start + cheat path + dist to end
                            let cheat_cost = dist[r][c][0] + l1_dist + dist[rr][cc][1];
                            if cheat_cost < base {
                                let saved = base - cheat_cost;
                                if saved >= SAVED_THRESHOLD {
                                    tot_count += 1;
                                    *freqs.entry(saved).or_insert(0) += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Some(tot_count)
}


pub fn part_two(input: &str) -> Option<usize> {
 solve(input, 20)
}

