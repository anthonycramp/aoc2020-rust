# AOC 2020 Day 05

https://adventofcode.com/2020/day/5

https://www.reddit.com/r/rust/comments/k71r9n/advent_of_code_2020_day_5/

## My Solution

Fairly standard "guessing game" solution. Solved both parts in about 30 min.
Rank approx 4200.

```rust
fn compute_seat_id(boarding_pass: &str) -> i32 {
    let (row, col) = compute_seat_row_col(&boarding_pass);
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
```

## Alternate solution

Musing after the fact, I was thinking that the boarding pass code could be
converted to binary and then decimal to yield the seat row and column.
Eyeballing it now, it looks like it will work.

It does work:

```rust
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
```

## More simplification

After reviewing a few other solutions:

Instead of the `chars().map().collect()`, I can just use `replace()`.

The seat id calculation `row * 8 + col` is just a binary right shift by three
and addition. I can just treat the entire boarding pass code as one binary
number to generate the seat id.

Something like:

```rust
fn compute_seat_id_binary(boarding_pass: &str) -> (i32, i32) {
    let binary = boarding_pass
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1");
    i32::from_str_radix(binary.as_str(), 2).expect("Error decoding binary row")
}
```
