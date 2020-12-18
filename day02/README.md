# Advent of Code Day 02

I was reminded of clippy after my first solution commit. Clippy taught me that:

```rust
iter().collect().len()
```

can be replaced by

```rust
iter().count()
```

In my original solution, I had duplication because I couldn't think how to pass
a struct method to a function. But, struct methods are just functions that take
a `self` as the first argument. So, struct methods can be passed just like any
other functions, with the exception that their function names require the
`struct_name::fn_name` syntax.
