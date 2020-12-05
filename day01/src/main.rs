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

fn find_pair_that_sums_to_2020(nums: &Vec<i32>) -> (i32, i32) {
    for (i, &v1) in nums.iter().enumerate() {
        for &v2 in nums.split_at(i + 1).1 {
            if v1 + v2 == 2020 {
                return (v1, v2);
            }
        }
    }

    (0, 0)
}

fn find_triple_that_sums_to_2020(nums: &Vec<i32>) -> (i32, i32, i32) {
    for (i, &v1) in nums.iter().enumerate() {
        for (j, &v2) in nums.split_at(i + 1).1.iter().enumerate() {
            for &v3 in nums.split_at(j + 1).1 {
                if v1 + v2 + v3 == 2020 {
                    return (v1, v2, v3);
                }
            }
        }
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
