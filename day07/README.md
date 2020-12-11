# AOC 2020 Day 07

## Problem

Due to recent aviation regulations, many rules (your puzzle input) are being
enforced about bags and their contents; bags must be color-coded and must
contain specific quantities of other color-coded bags. Apparently, nobody
responsible for these regulations considered how long they would take to
enforce!

For example, consider the following rules:

```text
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
```

These rules specify the required contents for 9 bag types. In this example,
every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded
blue and 6 dotted black), and so on.

You have a shiny gold bag. If you wanted to carry it in at least one other bag,
how many different bag colors would be valid for the outermost bag? (In other
words: how many colors can, eventually, contain at least one shiny gold bag?)

In the above rules, the following options would be available to you:

- A bright white bag, which can hold your shiny gold bag directly.
- A muted yellow bag, which can hold your shiny gold bag directly, plus some
  other bags.
- A dark orange bag, which can hold bright white and muted yellow bags, either
  of which could then hold your shiny gold bag.
- A light red bag, which can hold bright white and muted yellow bags, either of
  which could then hold your shiny gold bag.

So, in this example, the number of bag colors that can eventually contain at
least one shiny gold bag is 4.

How many bag colors can eventually contain at least one shiny gold bag? (The
list of rules is quite long; make sure you get all of it.)

## Discussion

We can ignore the bag counts for Part 1.

The rules basically describe a tree where the nodes need not be unique, or a
graph if the nodes are unique.

If we take a tree, then the answer is the sum of the path lenghts from the
"shiny gold" nodes to the root. Actually, there may be many root nodes so we're
talking multi-tree not just single tree.

If we take graph then we need to maintain directed edges (contains, contained
by) and walk all the paths from "shiny gold" to all nodes that have no contained
by edges. Tree seems simpler.

Token based parsing is fairly simple.

The bag counts are going to be a pain in the arse for Part 2.
