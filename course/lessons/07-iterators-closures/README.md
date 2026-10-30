---
title: Iterators and closures
track: Foundations
summary: Chain filter, map and fold into a small log analyser, and write closures that capture their surroundings.
concepts: Iterator, filter_map, fold, collect, closures, move, impl Fn
---
## What you'll build
A log analyser that counts lines per level, extracts slow-request durations and filters long lines.

## Run it
```bash
cargo run -p lesson-07-iterators-closures
cargo test -p lesson-07-iterators-closures
```

## How it works
- Iterators are lazy: nothing runs until something consumes them, like `collect`, `count` or `fold`.
- `filter_map` filters and transforms in one step by returning `Option`; the `None`s are dropped.
- `fold` threads an accumulator (here a `HashMap`) through every item.
- A closure can capture variables; `move` makes it own them, so `longer_than` can return it.
- These chains compile to loops as fast as ones you would write by hand.

## Try this
1. Review, lesson 06: write `fn count_by<T, F: Fn(&T) -> String>(items: &[T], key: F)` and use it to count log levels.
2. Find the most common request path.
3. Return the ERROR lines with their line numbers (`enumerate`).
4. Read the log from a file named on the command line (`std::env::args`).
