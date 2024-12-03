use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum: usize = re
        .captures_iter(input)
        .map(|cap| {
            let x: usize = cap[1].parse().unwrap();
            let y: usize = cap[2].parse().unwrap();
            x * y
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let search = vec!["don't()", "do()"];
    let mut i = 0;
    let mut start = 0;
    let mut trimmed_input = String::new();
    // println!("{input}");
    while start < input.len() {
        match input[start..].find(search[i]) {
            Some(index) => {
                if i == 0 {
                    // push everything before "don't()"
                    trimmed_input.push_str(&input[start..start + index]);
                }
                start += index + search[i].len();
            }
            None => {
                if i == 0 { // don't()
                    trimmed_input.push_str(&input[start..]);
                }
                break
            }
        }
        i = (i + 1) % 2
    }

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum: usize = re
        .captures_iter(&*trimmed_input)
        .map(|cap| {
            let x: usize = cap[1].parse().unwrap();
            let y: usize = cap[2].parse().unwrap();
            x * y
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
