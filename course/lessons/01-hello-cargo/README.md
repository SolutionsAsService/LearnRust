---
title: Hello, Cargo
track: Foundations
summary: Create, run and test a Rust program with Cargo, Rust's build tool and package manager.
concepts: cargo, fn, String vs &str, format!, unit tests
---
## What you'll build
A program that prints a greeting and a formatted number, with one test that proves the greeting is right.

## Run it
```bash
cargo run -p lesson-01-hello-cargo
cargo test -p lesson-01-hello-cargo
```

## How it works
- `cargo` creates the project, compiles it, runs it and runs its tests. You rarely call `rustc` yourself.
- `fn greet(name: &str) -> String` borrows a piece of text (`&str`) and returns a new, owned `String`.
- `format!` builds a `String` with the same `{}` placeholders `println!` uses. `{name}` reads a variable straight from scope.
- Tests live next to the code in a `#[cfg(test)]` module and only compile under `cargo test`.

## Try this
1. Add a second parameter so `greet` can say good morning or good evening.
2. Print a table of three items and prices with aligned columns.
3. Break the test on purpose and read what `cargo test` tells you.
