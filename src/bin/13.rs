use advent_of_code::get_nums_f64;
advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
     compute_tokens(input, 0.)
}

pub fn part_two(input: &str) -> Option<usize> {
    compute_tokens(input, 10000000000000.)
}

fn is_close_to_int(x: f64, eps: f64) -> bool {
    let rounded = x.round();
    (x - rounded).abs() < eps
}

fn compute_tokens(input: &str, x: f64) -> Option<usize> {
    let mut A = (0.,0.);
    let mut B = (0.,0.);
    let mut Prize = (0., 0.);
    let mut sum = 0;
    for (i,line) in input.lines().enumerate() {
        let nums = get_nums_f64(line);
        if i % 4 == 0 {
            A = (nums[0], nums[1]);
        } else if i % 4 == 1 {
            B = (nums[0], nums[1]);
        } else if i % 4 == 2 {
            Prize = (nums[0] + x, nums[1] + x);
        } else {
            let (a1, b1, c1) = (A.0, B.0, Prize.0);
            let (a2, b2, c2) = (A.1, B.1, Prize.1);
            // Compute determinants
            let d  = a1 * b2 - a2 * b1;
            let da = c1 * b2 - c2 * b1;
            let db = a1 * c2 - a2 * c1;

            // if d.abs() < f64::EPSILON {
            //     println!("No unique solution exists (determinant is zero).");
            // } else {
            let a = da / d;
            let b = db / d;
            if is_close_to_int(a, 1e-9) && is_close_to_int(b, 1e-9) {
                // println!("a = {}, b = {}", a, b);
                sum += 3* a.round() as usize + b.round() as usize;
            }
            // }
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
