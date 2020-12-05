fn main() {
    let input = aoc2020::get_input_string_from_file().unwrap();

    let part1_output = run_part1(&input);
    let part2_output = run_part2(&input);

    println!("Day 05 Part 1: {}", part1_output);
    println!("Day 05 Part 2: {}", part2_output);
}

fn run_part1(_input: &str) -> bool {
    false
}

fn run_part2(_input: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_input() {}
}
