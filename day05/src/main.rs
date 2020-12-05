fn main() {
    let input = aoc2020::get_input_string_from_file().unwrap();

    let part1_output = run_part1(&input);
    let part2_output = run_part2(&input);

    println!("Day 05 Part 1: {}", part1_output);
    println!("Day 05 Part 2: {}", part2_output);
}

fn run_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| compute_seat_id(line, compute_seat_row_col_binary))
        .max()
        .expect("Unexpected")
}

fn run_part2(input: &str) -> i32 {
    let mut seat_ids = input
        .lines()
        .map(|line| compute_seat_id(line, compute_seat_row_col_binary))
        .collect::<Vec<_>>();
    seat_ids.sort();
    let mut iter = seat_ids.iter_mut();
    let mut first = iter.next().expect("unexpected");
    let mut second = iter.next().expect("unexpected");

    while *second - *first == 1 {
        first = second;
        second = iter.next().expect("unexpected");
    }

    *first + 1
}

fn compute_seat_id(boarding_pass: &str, compute_row_col_fn: fn(&str) -> (i32, i32)) -> i32 {
    let (row, col) = compute_row_col_fn(&boarding_pass);
    row * 8 + col
}

fn compute_seat_row_col(boarding_pass: &str) -> (i32, i32) {
    let row_code = &boarding_pass[0..7];
    let col_code = &boarding_pass[7..];

    let mut low = 0;
    let mut high = 127;

    for code in row_code.chars() {
        match code {
            'F' => high = low + (high - low) / 2,
            'B' => low = low + (high - low) / 2,
            _ => panic!("Unknown row code: {}", code),
        }
    }

    let row = high;

    let mut low = 0;
    let mut high = 7;

    for code in col_code.chars() {
        match code {
            'L' => high = low + (high - low) / 2,
            'R' => low = low + (high - low) / 2,
            _ => panic!("Unknown row code: {}", code),
        }
    }

    let col = high;

    (row, col)
}

fn compute_seat_row_col_binary(boarding_pass: &str) -> (i32, i32) {
    let row_code = &boarding_pass[..7];
    let row_code_binary = row_code
        .chars()
        .map(|c| match c {
            'F' => '0',
            _ => '1',
        })
        .collect::<String>();

    let row = i32::from_str_radix(row_code_binary.as_str(), 2).expect("Error decoding binary row");

    let col_code = &boarding_pass[7..];
    let col_code_binary = col_code
        .chars()
        .map(|c| match c {
            'L' => '0',
            _ => '1',
        })
        .collect::<String>();
    let col = i32::from_str_radix(col_code_binary.as_str(), 2).expect("Error decoding binary col");

    (row, col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_seat_row_col1() {
        let boarding_pass = "BFFFBBFRRR";
        let seat_row_col = compute_seat_row_col(&boarding_pass);
        assert_eq!(seat_row_col.0, 70);
        assert_eq!(seat_row_col.1, 7);
    }

    #[test]
    fn test_compute_seat_row_col_binary1() {
        let boarding_pass = "BFFFBBFRRR";
        let seat_row_col = compute_seat_row_col_binary(&boarding_pass);
        assert_eq!(seat_row_col.0, 70);
        assert_eq!(seat_row_col.1, 7);
    }

    #[test]
    fn test_compute_seat_row_col2() {
        let boarding_pass = "FFFBBBFRRR";
        let seat_row_col = compute_seat_row_col(&boarding_pass);
        assert_eq!(seat_row_col.0, 14);
        assert_eq!(seat_row_col.1, 7);
    }

    #[test]
    fn test_compute_seat_row_col3() {
        let boarding_pass = "BBFFBBFRLL";
        let seat_row_col = compute_seat_row_col(&boarding_pass);
        assert_eq!(seat_row_col.0, 102);
        assert_eq!(seat_row_col.1, 4);
    }
}
