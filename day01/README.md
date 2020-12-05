# Advent of Code 2020 Day 01

https://adventofcode.com/2020/day/1

Reddit post:
https://www.reddit.com/r/rust/comments/k4hoyk/advent_of_code_2020_day_1/

## My Solution

Brute force:

```rust
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
```

Things to remember:

- `enumerate()` returns `(index, &val)`
- `split_at(index)` returns a tuple of two iterators, the first covering
  [0,index) and the second [index,len).

Part 2 duplicated Part 1:

```rust
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
```

Error handling isn't great either.

## Updated solutions

From
https://github.com/coriolinus/adventofcode-2020/blob/master/day01/src/lib.rs

The inner loop for Part 1 can be replaced with a query for whether the remainder
of the data `contains` value `target - v1`.

For Part 2, the two inner loops can be replaced with a call to the Part 1
solution.

## Use Itertools

Using `cartesian_product` to first create all pairs/triples:
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=3f68293bd4685a5ff0e73530826dcc34

```rust
let query = 2020;
let result = INPUT.iter()
    .cartesian_product(&INPUT)
    .find(|(l, r)| *l + *r == query)
    .expect("Query was not found");
println!("found {}, {}, with product: {}", result.0, result.1, result.0 * result.1);
```

## Algorithmic Improvement

https://github.com/SkiFire13/adventofcode-2020-rs/blob/master/src/day1.rs

Walks up and down a sorted list of numbers to find the sum.

Good example of functions taking slices instead of Vecs.

## include_str!

From:
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=3330d0a8f055abaee1fe43204e892091

Macro `include_str!` loads the contents of a file as a string at compile time.
Also `include_bytes!`.

- https://doc.rust-lang.org/std/macro.include_str.html
- https://doc.rust-lang.org/std/macro.include_bytes.html
