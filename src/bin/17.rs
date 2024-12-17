use advent_of_code::get_nums_usize;

advent_of_code::solution!(17);


pub fn part_one(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let A = get_nums_usize(lines.next().unwrap())[0];
    let B = get_nums_usize(lines.next().unwrap())[0];
    let C = get_nums_usize(lines.next().unwrap())[0];
    lines.next();
    let program = get_nums_usize(lines.next().unwrap());
    let out = evaluate(A, B, C, &program);

    Some(out.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
}

fn evaluate(mut A: usize, mut B: usize, mut C: usize, program: &Vec<usize>) -> Vec<usize> {
    let mut out = vec![];
    let mut i = 0;
    while i+1 < program.len() {
        match program[i] {
            0 => A = A / (1 << combo_operand(program[i+1], A, B, C)),
            1 => B ^= program[i+1],
            2 => B = combo_operand(program[i+1], A, B, C) % 8,
            3 => {
                if A == 0 {
                    i += 1;
                } else {
                    i = program[i+1];
                }
                continue
            },
            4 => B ^= C,
            5 => out.push(combo_operand(program[i+1], A, B, C) % 8),
            6 => B = A / (1 << combo_operand(program[i+1], A, B, C)),
            7 => C = A / (1 << combo_operand(program[i+1], A, B, C)),
            _ => unreachable!(),
        }
        i += 2;
    }
    out

}
fn combo_operand(x: usize, A: usize, B: usize, C: usize) -> usize {
    match x {
        0..=3 => x,
        4 => A,
        5 => B,
        6 => C,
        _ => unreachable!()
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    lines.next(); // toss A
    let mut A = 0;
    let B = get_nums_usize(lines.next().unwrap())[0];
    let C = get_nums_usize(lines.next().unwrap())[0];
    lines.next();

    let program = get_nums_usize(lines.next().unwrap());
    for matched in 0..program.len() {
        A *= 8;
        while evaluate(A, B, C, &program) != program[program.len() - matched - 1..] {
            A += 1
        }
    }
    Some(A)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(117440));
    }
}
