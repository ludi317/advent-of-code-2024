use std::collections::{HashMap, HashSet};
use advent_of_code::get_nums;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    // let width = 11;
    let width = 101;
    // let height = 7;
    let height = 103;
    let num_seconds = 100;
    let mut quad_counts = [[0usize;2]; 2];
    for line in input.lines() {
        let nums = get_nums(line);

        let x = (((num_seconds * nums[2] + nums[0]) % width) + width)%width;
        let y = (((num_seconds * nums[3] + nums[1]) % height) + height)%height;
        // println!("{:?}", nums);
        // println!("{x}, {y}");
        let i1 = if x < width / 2 {
            0
        } else if x > width / 2 {
            1
        } else {
            continue
        };
        let i2 = if y < height / 2 {
            0
        } else if y > height / 2 {
            1
        } else {
            continue
        };
        quad_counts[i1][i2] += 1;
        // println!("{:?}", quad_counts);
    }

    Some(quad_counts[0][0] * quad_counts[1][0]*quad_counts[0][1]*quad_counts[1][1])
}

pub fn part_two(input: &str) -> Option<isize> {
    let width = 101;
    let height = 103;
    for i in 7132..=7132{
    // for i in 0..1_000_000 {
        let mut pos: HashSet<(isize, isize)> = HashSet::new();
        let mut unique = true;
        let mut grid: Vec<Vec<char>> = vec![vec![' '; 103]; 101];
        for line in input.lines() {
            let nums = get_nums(line);

            let x = (((i * nums[2] + nums[0]) % width) + width) % width;
            let y = (((i * nums[3] + nums[1]) % height) + height) % height;
            if pos.contains(&(x, y)) {
                unique = false;
                continue
            } else {
                pos.insert((x,y));
            }
            grid[x as usize][y as usize] = '.';
        }
        if unique {
            println!("{i}");
            for g in grid {
                println!("{:?}", String::from_iter(g));
            }
            return  None;
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
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
