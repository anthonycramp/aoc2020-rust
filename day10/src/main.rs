const INPUT: &str = include_str!("../../resources/day10_input.txt");

fn main() {
    let joltages = parse_input(INPUT);
    let part1 = find_one_jolt_diffs(&joltages) * find_three_jolt_diffs(&joltages);

    println!("Day 10 Part 1: {}", part1);
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.trim().parse().expect("Couldn't parse input number"))
        .collect()
}

fn find_jolt_diffs(joltages: &[i32], diff: i32) -> i32 {
    let mut sum = 0;
    for i in 0..joltages.len() - 1 {
        if joltages[i + 1] - joltages[i] == diff {
            sum += 1
        }
    }

    sum
}

fn find_one_jolt_diffs(joltages: &[i32]) -> i32 {
    let mut jolt_copy = joltages.to_vec();
    jolt_copy.sort_unstable();
    let one_jolt_from_outlet = if jolt_copy[0] == 1 { 1 } else { 0 };

    one_jolt_from_outlet + find_jolt_diffs(&jolt_copy, 1)
}

fn find_three_jolt_diffs(joltages: &[i32]) -> i32 {
    let mut jolt_copy = joltages.to_vec();
    jolt_copy.sort_unstable();

    // + 1 below is for the device's 3-jolt diff from hightest adapter
    find_jolt_diffs(&jolt_copy, 3) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4";
        let joltages = parse_input(input);
        let one_jolt_diffs = find_one_jolt_diffs(&joltages);
        let three_jolt_diffs = find_three_jolt_diffs(&joltages);

        assert_eq!(one_jolt_diffs, 7);
        assert_eq!(three_jolt_diffs, 5);
    }

    #[test]
    fn test2() {
        let input = "28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";
        let joltages = parse_input(input);
        let one_jolt_diffs = find_one_jolt_diffs(&joltages);
        let three_jolt_diffs = find_three_jolt_diffs(&joltages);

        assert_eq!(one_jolt_diffs, 22);
        assert_eq!(three_jolt_diffs, 10);
    }
}
