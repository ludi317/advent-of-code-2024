use std::collections::{HashMap, HashSet};
use advent_of_code::get_nums_usize;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let mut before: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut part1 = true;
    let mut sum = 0;
    for line in input.lines() {
        if part1 {
            let nums = get_nums_usize(line);
            if nums.len() == 0 {
                part1 = false;
                continue
            }
            before.entry(nums[1]).or_insert(HashSet::new()).insert(nums[0]);
        } else {
            let nums = get_nums_usize(line);
            if is_valid(&nums, &before) {
                sum += nums[nums.len()/2]
            }
        }
    }
    Some(sum)
}

fn is_valid(nums: &Vec<usize>, before: &HashMap<usize, HashSet<usize>> ) -> bool {
    let mut forbidden: HashSet<usize> = HashSet::new();
    for num in nums {
        if forbidden.contains(&num) {
            return false
        }
        if let Some(befores) = before.get(num) {
            forbidden.extend(befores);
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut before: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut part1 = true;
    let mut sum = 0;
    for line in input.lines() {
        if part1 {
            let nums = get_nums_usize(line);
            if nums.len() == 0 {
                part1 = false;
                continue
            }
            before.entry(nums[1]).or_insert(HashSet::new()).insert(nums[0]);
        } else {
            let nums = get_nums_usize(line);
            let (middle, valid) = reorder_and_get_middle(&nums, &before);
            if !valid {
                sum += middle;
            }

        }
    }
    Some(sum)
}

fn reorder_and_get_middle(nums: &Vec<usize>, before: &HashMap<usize, HashSet<usize>>) -> (usize, bool) {
    let mut num_set: HashSet<usize> = HashSet::new();
    num_set.extend(nums);
    let mut valid = true;
    let mid_idx = nums.len() / 2;
    for (i, n) in nums.iter().enumerate() {
        // get length of intersection of before.get(n) and num_set
        if let Some(befores) = before.get(n) {
            let count_before = befores.intersection(&num_set).count();
            if i != count_before {
                valid = false
            }
            if count_before == mid_idx {
                return (*n, valid);
            }
        }
    }
    (0, valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
