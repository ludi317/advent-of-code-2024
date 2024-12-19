advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let patterns: Vec<&str> = lines.next().unwrap().split(", ").collect();
    lines.next();
    let sum = lines.filter(|line| can_make(&patterns, line)).count();
    Some(sum)
}

fn can_make(patterns: &Vec<&str>, design: &str) -> bool {
    let mut dp = vec![false; design.len()+1];
    dp[0] = true;
    for i in 0..design.len() {
        if !dp[i] {
            continue
        }
        for &p in patterns {
            if design[i..].starts_with(p) {
                dp[i+ p.len()] = true
            }
        }
    }
    dp[design.len()]
}

fn num_ways(patterns: &Vec<&str>, design: &str) -> usize {
    let mut dp = vec![0; design.len()+1];
    dp[0] = 1;
    for i in 0..design.len() {
        if dp[i] == 0 {
            continue
        }
        for &p in patterns {
            if design[i..].starts_with(p) {
                dp[i+ p.len()] += dp[i]
            }
        }
    }
    dp[design.len()]
}


pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let patterns: Vec<&str> = lines.next().unwrap().split(", ").collect();
    lines.next();
    let sum = lines.map(|line| num_ways(&patterns, line)).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
