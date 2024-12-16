use std::collections::{HashMap, HashSet};
use pheap::PairingHeap;

advent_of_code::solution!(16);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    r: usize,
    c: usize,
    d_idx: usize,
}
const DIRECTIONS: [(isize,isize); 4] = [(-1,0),(0,1),(1,0),(0,-1)];
// north, east, south, west
const EAST: usize = 1;

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut heap: PairingHeap<State, usize> = PairingHeap::new();
    let mut costs: HashMap<State, usize> = HashMap::new();
    let sr = grid.len() - 2;
    let sc = 1;
    assert_eq!(grid[sr][sc], 'S');
    heap.insert(State{r:sr, c:sc, d_idx: EAST}, 0);
    costs.insert(State{r:sr, c:sc, d_idx: EAST}, 0);
    while let Some((state, cost)) = heap.delete_min() {
        if grid[state.r][state.c] == 'E' {
            return Some(cost)
        }

        for d_ix in [state.d_idx, (state.d_idx+3)%4, (state.d_idx + 1)%4] {
            let nr = state.r.wrapping_add_signed(DIRECTIONS[d_ix].0);
            let nc = state.c.wrapping_add_signed(DIRECTIONS[d_ix].1);
            if grid[nr][nc] != '#' {
                let new_cost = cost + 1 + if d_ix != state.d_idx { 1000 } else { 0 };
                let new_state = State{r:nr, c:nc, d_idx: d_ix};
                if !costs.contains_key(&new_state) || costs[&new_state] > new_cost {
                    heap.insert(new_state, new_cost);
                    costs.insert(new_state, new_cost);
                }
            }
        }

    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut heap: PairingHeap<State, usize> = PairingHeap::new();
    let mut costs: HashMap<State, usize> = HashMap::new();
    let mut min_cost = 0;
    let mut pos: HashSet<(usize, usize)> = HashSet::new();
    let mut paths: HashMap<State, HashSet<(usize, usize)>> = HashMap::new();
    let sr = grid.len() - 2;
    let sc = 1;
    assert_eq!(grid[sr][sc], 'S');
    let state = State{r:sr, c:sc, d_idx: EAST};
    heap.insert(state, 0);
    costs.insert(state, 0);
    paths.insert(state, HashSet::from([(sr, sc)]));

    // let mut grid_debug = grid.clone();
    while let Some((state, cost)) = heap.delete_min() {
        if grid[state.r][state.c] == 'E' {
            if min_cost == 0 || min_cost == cost {
                min_cost = cost;
                pos.extend(&paths[&state]);
                // println!("{cost}");
                continue
            } else {
                // for (r,c) in pos.iter() {
                //     grid_debug[*r][*c] = 'O';
                // }
                // for g in &grid_debug {
                //     println!("{:?}", g.iter().collect::<String>());
                // }
                return Some(pos.len())
            }

        }


        for d_ix in [state.d_idx, (state.d_idx+3)%4, (state.d_idx + 1)%4] {
            let nr = state.r.wrapping_add_signed(DIRECTIONS[d_ix].0);
            let nc = state.c.wrapping_add_signed(DIRECTIONS[d_ix].1);
            if grid[nr][nc] != '#' {
                let new_cost = cost + 1 + if d_ix != state.d_idx { 1000 } else { 0 };
                let new_state = State{r:nr, c:nc, d_idx: d_ix};
                if !costs.contains_key(&new_state) || costs[&new_state] > new_cost {
                    heap.insert(new_state, new_cost);
                    costs.insert(new_state, new_cost);
                    // overwrite paths with new_path = state + (nr, nc)
                    let mut new_path = paths[&state].clone();
                    new_path.insert((nr,nc));
                    paths.insert(new_state, new_path);
                } else if costs[&new_state] == new_cost {
                    // an equally good path -- add new path to paths
                    let state_paths = paths[&state].clone();
                    paths.get_mut(&new_state).unwrap().extend(state_paths);
                    paths.get_mut(&new_state).unwrap().insert((nr, nc));
                }
            }
        }

    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(11048));
    }


    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(64));
    }

}
