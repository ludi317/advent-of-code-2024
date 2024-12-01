use std::collections::HashMap;
use advent_of_code::{get_nums, get_nums_usize};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let nums = get_nums(&line);
        left.push(nums[0]);
        right.push(nums[1]);
    }

    left.sort_unstable();
    right.sort_unstable();
    let sum: usize = left.iter()
        .zip(right.iter())
        .map(|(l,r)| (l-r).abs() as usize)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut left: HashMap<usize, usize> = HashMap::new();
    let mut right: HashMap<usize, usize> = HashMap::new();
    for line in input.lines() {
        let nums = get_nums_usize(&line);
        *left.entry(nums[0]).or_insert(0) += 1;
        *right.entry(nums[1]).or_insert(0) += 1;
    }
    let mut sum: usize = 0;
    for (key, left_value) in left.iter() {
        if let Some(right_value) = right.get(key) {
            sum += key * left_value * right_value
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
