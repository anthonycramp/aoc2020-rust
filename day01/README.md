# Advent of Code 2020 Day 01

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

## Alternate solutions

From
https://github.com/coriolinus/adventofcode-2020/blob/master/day01/src/lib.rs

The inner loop for Part 1 can be replaced with a query for whether the remainder
of the data `contains` value `target - v1`.

For Part 2, the two inner loops can be replaced with a call to the Part 1
solution.
