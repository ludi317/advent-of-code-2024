use std::collections::HashSet;
use advent_of_code::get_nums_usize;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    for line in input.lines() {
        let nums = get_nums_usize(line);

        let mut h: HashSet<usize> = HashSet::new();
        h.insert(nums[1]);
        for i in 2..nums.len() {
            let mut hh:  HashSet<usize> = HashSet::new();
            for &v in h.iter() {
                hh.insert(v + nums[i]);
                hh.insert(v * nums[i]);
            }
            h = hh;
        }
        if h.contains(&nums[0]) {
            sum += nums[0];
        }


    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum = 0;
    for line in input.lines() {
        let nums = get_nums_usize(line);

        let mut h: HashSet<usize> = HashSet::new();
        h.insert(nums[1]);
        for i in 2..nums.len() {
            let mut hh:  HashSet<usize> = HashSet::new();
            for &v in h.iter() {
                hh.insert(v + nums[i]);
                hh.insert(v * nums[i]);
                let digits = num_digits(nums[i]);
                let mut vv = v;
                for i in 0..digits {
                    vv *= 10;
                }
                hh.insert(vv + nums[i]);
            }
            h = hh;
        }
        if h.contains(&nums[0]) {
            sum += nums[0];
        }


    }
    Some(sum)
}

fn num_digits(mut x: usize) -> usize {
    let mut digits = 0;
    while x > 0 {
        x /= 10;
        digits += 1;
    }
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
