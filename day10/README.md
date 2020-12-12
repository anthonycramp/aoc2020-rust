# AOC 2020 Day 10

## Working through Part 2

Looking for a mathematical solution to this.

All the 3 jolt diffs need to appear in all combinations. So, there are at least
that many.

Sequences of 1-jolt, 1-jolt; 1-jolt, 2-jolt; 2-jolt, 1-jolt; 1-jolt, 1-jolt,
1-jolt can be replaced with 2- and 3-jolt diffs.

For this test case `(0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)`, in diff
terms we have

```text
1,3,1,1,1,3,1,1,3,1,3,3
```

This has sub-sequences:

```text
1,1,1 -> 2,1; 1,2; 3
1,1 -> 2
```

All combinations

```text
(0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4, 5, 6, 7, 10, 12, 15, 16, 19, (22) - 1,1 -> 2
(0), 1, 4, 5, 7, 10, 11, 12, 15, 16, 19, (22) - 1,1,1 -> 1,2
(0), 1, 4, 5, 7, 10, 12, 15, 16, 19, (22) - 1,1,1 -> 1,2; 1,1 -> 2
(0), 1, 4, 6, 7, 10, 11, 12, 15, 16, 19, (22) - 1,1,1 -> 2,1
(0), 1, 4, 6, 7, 10, 12, 15, 16, 19, (22) - 1,1,1 -> 2,1; 1,1 -> 2
(0), 1, 4, 7, 10, 11, 12, 15, 16, 19, (22) - 1,1,1 -> 3
(0), 1, 4, 7, 10, 12, 15, 16, 19, (22) - 1,1,1 -> 3; 1,1 -> 2
```

original (1) + mutually exclusive sequences (4) + combinations of mutually
exclusive sequences (3 \* 1)
