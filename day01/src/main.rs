fn main() {
    let input = aoc2020::get_input_string_from_file().unwrap();

    let nums = parse_input_whitespace_to_i32(&input);
    let pair = find_pair_that_sums_to_2020(&nums);
    let triple = find_triple_that_sums_to_2020(&nums);
    println!("Day 01, Part 1: {}", pair.0 * pair.1);
    println!("Day 01, Part 2: {}", triple.0 * triple.1 * triple.2);
}

fn parse_input_whitespace_to_i32(input: &str) -> Vec<i32> {
    let fields: Vec<&str> = input.split_whitespace().collect();
    fields.iter().map(|f| f.parse().unwrap()).collect()
}

fn find_pair_that_sums_to_target(nums: &Vec<i32>, target: i32) -> (i32, i32) {
    for (i, &v1) in nums.iter().enumerate() {
        let remainder = target - v1;
        if nums.split_at(i + 1).1.contains(&remainder) {
            return (v1, remainder);
        }
    }

    (0, 0)
}

fn find_pair_that_sums_to_2020(nums: &Vec<i32>) -> (i32, i32) {
    find_pair_that_sums_to_target(nums, 2020)
}

fn find_triple_that_sums_to_2020(nums: &Vec<i32>) -> (i32, i32, i32) {
    for (i, &v1) in nums.iter().enumerate() {
        let remainder = 2020 - v1;
        let pair = find_pair_that_sums_to_target(&Vec::from(nums.split_at(i + 1).1), remainder);
        if pair.0 == 0 && pair.1 == 0 {
            continue;
        }
        return (v1, pair.0, pair.1);
    }

    (0, 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1721
        979
        366
        299
        675
        1456";
        let nums = parse_input_whitespace_to_i32(&input);
        assert_eq!(nums.len(), 6);
        assert_eq!(nums[0], 1721);
        assert_eq!(nums[1], 979);
        assert_eq!(nums[2], 366);
        assert_eq!(nums[3], 299);
        assert_eq!(nums[4], 675);
        assert_eq!(nums[5], 1456);
    }

    #[test]
    fn test_find_pair_that_sums_to_2020() {
        let input = "1721
        979
        366
        299
        675
        1456";
        let nums = parse_input_whitespace_to_i32(&input);
        let pair = find_pair_that_sums_to_2020(&nums);
        assert_eq!(pair.0, 1721);
        assert_eq!(pair.1, 299);
        assert_eq!(pair.0 * pair.1, 514579);
    }

    #[test]
    fn test_find_triple_that_sums_to_2020() {
        let input = "1721
        979
        366
        299
        675
        1456";
        let nums = parse_input_whitespace_to_i32(&input);
        let triple = find_triple_that_sums_to_2020(&nums);
        assert_eq!(triple.0, 979);
        assert_eq!(triple.1, 366);
        assert_eq!(triple.2, 675);
        assert_eq!(triple.0 * triple.1 * triple.2, 241861950);
    }
}
