use advent_of_code::get_nums;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let count = input
        .lines()
        .filter(|line| {
            let nums = get_nums(line);
            let first_diff = nums[0] - nums[1];

            let valid_range = match first_diff {
                1..=3 => 1..=3,
                -3..=-1 => -3..=-1,
                _ => return false,
            };

            nums[1..]
                .windows(2)
                .map(|window| window[0] - window[1])
                .all(|x| valid_range.contains(&x))
        })
        .count();

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut skipped_nums: Vec<isize> = vec![0isize; 7];
    let count = input
        .lines()
        .filter(|line| {
            let nums = get_nums(line);
            for skip_idx in 0..nums.len() {
                for i in 0..skip_idx {
                    skipped_nums[i] = nums[i];
                }
                for i in skip_idx + 1..nums.len() {
                    skipped_nums[i - 1] = nums[i]
                }
                let first_diff = skipped_nums[0] - skipped_nums[1];

                let valid_range = match first_diff {
                    1..=3 => 1..=3,
                    -3..=-1 => -3..=-1,
                    _ => continue,
                };
                if skipped_nums[1..nums.len() - 1]
                    .windows(2)
                    .map(|window| window[0] - window[1])
                    .all(|x| valid_range.contains(&x))
                {
                    return true;
                }
            }
            false
        })
        .count();

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
