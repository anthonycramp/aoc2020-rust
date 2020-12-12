const INPUT: &str = include_str!("../../resources/day09_input.txt");

fn main() {
    let numbers = parse_input(INPUT);
    let window_size = 25;

    let part1_weakness = find_xmas_weakness(&numbers, window_size);
    println!("Day 09 Part 1: {}", part1_weakness);

    let part2_weakness = find_xmas_encryption_weakness(&numbers, part1_weakness);
    println!("Day 09 Part 2: {}", part2_weakness);
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.trim().parse().expect("Failed parsing number"))
        .collect()
}

fn find_pair_that_sums_to_target(nums: &[i64], target: i64) -> Option<(i64, i64)> {
    for (i, &v1) in nums.iter().enumerate() {
        let remainder = target - v1;
        if nums.split_at(i + 1).1.contains(&remainder) {
            return Some((v1, remainder));
        }
    }

    None
}

fn find_xmas_weakness(numbers: &[i64], window_size: i64) -> i64 {
    let mut index = 0;

    loop {
        let target_number = numbers[(index + window_size) as usize];
        let window_lower = index as usize;
        let window_upper = window_lower + window_size as usize;

        match find_pair_that_sums_to_target(&numbers[window_lower..window_upper], target_number) {
            None => return target_number,
            _ => (),
        }
        index += 1
    }
}

fn find_xmas_encryption_weakness(numbers: &[i64], target_num: i64) -> i64 {
    let mut low_index = 0;

    loop {
        for i in 0..numbers.split_at(low_index + 1).1.len() {
            let high_index = low_index + i;
            let contiguous_range = low_index as usize..high_index as usize;
            let contiguous_sum: i64 = numbers[contiguous_range.clone()].iter().sum();

            if contiguous_sum == target_num {
                return numbers[contiguous_range.clone()]
                    .iter()
                    .min()
                    .expect("Couldn't find min!")
                    + numbers[contiguous_range.clone()]
                        .iter()
                        .max()
                        .expect("Couldn't find max!");
            } else if contiguous_sum > target_num {
                break;
            }
        }

        low_index += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";
        let numbers = parse_input(input);
        let weakness = find_xmas_weakness(&numbers, 5);
        assert_eq!(weakness, 127);
    }

    #[test]
    fn test2() {
        let input = "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";
        let numbers = parse_input(input);
        let weakness = find_xmas_encryption_weakness(&numbers, 127);
        assert_eq!(weakness, 62);
    }
}
