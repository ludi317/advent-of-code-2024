use std::collections::HashMap;
use advent_of_code::get_nums_usize;

advent_of_code::solution!(11);

fn count_split(v: usize, remaining: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(&cached) = memo.get(&(v, remaining)) {
        return cached;
    }

    let result = if remaining == 0 {
        1
    } else if v == 0 {
        count_split(1, remaining - 1, memo)
    } else {
        let num_digs = v.ilog10() + 1;
        if num_digs % 2 == 0 {
            let pow = 10usize.pow(num_digs / 2);
            let left = v / pow;
            let right = v % pow;
            count_split(left, remaining - 1, memo) + count_split(right, remaining - 1, memo)
        } else {
            count_split(v * 2024, remaining - 1, memo)
        }
    };

    memo.insert((v, remaining), result);
    result
}


pub fn part_one(input: &str) -> Option<usize> {
    let nums = get_nums_usize(input);
    let rem = 25;
    let mut memo: HashMap<_,_> = HashMap::new();
    Some(nums.iter().map(|&n| count_split(n, rem, &mut memo)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let nums = get_nums_usize(input);
    let rem = 75;
    let mut memo: HashMap<_,_> = HashMap::new();
    Some(nums.iter().map(|&n| count_split(n, rem, &mut memo)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }

}
