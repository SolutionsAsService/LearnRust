---
title: Modules and testing
track: Foundations
summary: Split code into modules and a library crate, control visibility, and test it three ways: unit, doc and integration tests.
concepts: mod, pub, pub(crate), lib.rs + main.rs, doc tests, tests/ folder
---
## What you'll build
A small temperature library with a binary that uses it, and every kind of test Cargo supports.

## Run it
```bash
cargo run -p lesson-08-modules-testing
cargo test -p lesson-08-modules-testing
```

## How it works
- `src/lib.rs` is a library crate; `src/main.rs` is a binary that uses it like any other dependency.
- `pub` exposes an item; `pub(crate)` keeps `describe` usable inside the crate but invisible outside.
- **Unit tests** sit inside the module and can test private code.
- **Doc tests** are the examples in `///` comments: `cargo test` compiles and runs them, so the docs can't drift.
- **Integration tests** in `tests/` only see the public API.

## Try this
1. Review, lesson 07: add an integration test that counts the words in a `report` string with an iterator chain.
2. Add Kelvin conversions with a doc test each.
3. Move `temperature` into its own file, `src/temperature.rs`.
4. Try calling `temperature::describe` from `tests/report.rs` and read the error.
