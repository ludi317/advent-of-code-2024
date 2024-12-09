advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut id = 0;
    let space = 1<<31 - 1;
    let mut count: usize = 0;
    let mut disk: Vec<usize> = Vec::new();
    for (i, ch) in input.bytes().enumerate() {
        let num = ch - b'0';
        if i % 2 == 0 {
            // file
            for _ in 0..num {
                disk.push(id);

            }
            count += num as usize;
            id += 1
        } else {
            for _ in 0..num {
                disk.push(space);
            }
        }
    }


    let mut end = disk.len() - 1;

    let mut checksum = 0;
    for i in 0..count {
        if disk[i] == space {
            while disk[end] == space {
                end -= 1;
            }
            //swap
            disk.swap(end, i);
            end -= 1;
        }
        checksum += i * disk[i]
    }


    Some(checksum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut id = 0;
    let space = 1<<31 - 1;
    let mut count: usize = 0;
    let mut disk: Vec<usize> = Vec::new();
    for (i, ch) in input.bytes().enumerate() {
        let num = ch - b'0';
        if i % 2 == 0 {
            // file
            for _ in 0..num {
                disk.push(id);

            }
            count += num as usize;
            id += 1
        } else {
            for _ in 0..num {
                disk.push(space);
            }
        }
    }

    let mut end = (disk.len() - 1) as isize;
    let mut id = disk[disk.len() - 1];
    while end >= 0 {
        while disk[end as usize] != id {
            end -= 1;
        }
        if id > 0 {
            id -= 1;
        }

        let mut left_end: isize = end ;
        while left_end >= 0 && disk[left_end as usize] == disk[end as usize] {
            left_end -= 1;
        }
        if left_end == -1 {
            break
        }
        let end_count = (end - left_end) as usize;

        // println!("{:?}", disk[end as usize]);
        let mut start = 0;
        while start + end_count <= left_end as usize {
            while disk[start] != space {
                start += 1;
            }
            let mut right_start = start + 1 ;
            while right_start < disk.len() && disk[right_start] == space {
                right_start += 1;
            }
            let start_count = right_start - start;
            if disk[end as usize] == 0 {
                // println!("start: {start}");
                // println!("end: {end}");
                // println!("left end: {left_end}");
            }
            if start_count >= end_count && (end - (end_count - 1) as isize) > (start + (end_count - 1)) as isize{
                for j in 0..end_count {
                    disk.swap(end  as usize - j, start + j);

                }
                // println!("{:?}", disk);
                break
            }
            start = right_start;
        }

        end = left_end;
    }

    // println!("{:?}", disk);

    let mut checksum = 0;
    for i in 0..disk.len() {
        if disk[i] == space {
            continue
        }
        checksum += i * disk[i]
    }


    Some(checksum)
}


// let length = input.len();
// let input_bytes: Vec<_> = input.as_bytes();
// let mut id = length / 2 + 1;
// let mut f = length - 1;
// let mut s = 1;
// while f >= 0 {
//
// input_bytes.s
//
// id -= 1;
// f -= 2;
// }
// // 2333133121414131402
// // 2 3 1 3 2 4 4 3 4 2 -- files
// //  3 3 3 1 1 1 1 1 0  -- spaces
//
// // 2 3 1 3 2 4 4  4  -- files
// //  1 3 1 1 1 1 1 0  -- spaces
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
