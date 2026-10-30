---
title: Ownership, moves and borrows
track: Foundations
summary: Every value has one owner. Learn when a value moves, when it is borrowed, and what the compiler says when you get it wrong.
concepts: ownership, move, borrow (&), clone, slices, lifetimes
---
## What you'll build
Three small text functions: one borrows, one returns a slice of its input, one takes ownership.

## Run it
```bash
cargo run -p lesson-02-ownership
cargo test -p lesson-02-ownership
```

## How it works
- A `String` has exactly one owner. `let t = s;` **moves** it: `s` can no longer be used.
- `&s` lends the value without moving it. Many shared borrows can exist at once.
- `.clone()` makes a second, independent copy when two owners really are needed.
- `longest_word` returns a `&str` that points into its input. The lifetime `'a` tells the compiler the result can't outlive the text it came from.

Try uncommenting a use of `s` after `shout(s)`. The compiler stops you with:

```text
error[E0382]: borrow of moved value: `s`
```

That error is the borrow checker doing its job: it found a use-after-move before the program ever ran.

## Try this
1. Change `shout` to take `&mut String` and modify the text in place. What changes at the call site?
2. Write `first_word(text: &str) -> &str`.
3. Return a `String` from `longest_word` instead of a slice. What does it cost?
