---
title: Error handling with Result
track: Foundations
summary: Parse a config file where every failure is a typed value with a line number, not a crash.
concepts: Result, Option, ?, map_err, ok_or, custom error enum, Display, std::error::Error
---
## What you'll build
A parser for `key = value` config text that returns either a `Config` or a precise error such as `line 2: \`70000\` is not a port (0-65535)`.

## Run it
```bash
cargo run -p lesson-05-error-handling
cargo test -p lesson-05-error-handling
```

## How it works
- Rust has no exceptions. A function that can fail returns `Result<T, E>`, and the type system makes callers deal with it.
- `?` unwraps an `Ok` or returns the `Err` early. `map_err` converts a library error (like a failed `parse`) into your own.
- A custom error `enum` carries exactly the facts needed to explain the problem; `Display` turns it into a sentence.
- Parsing `port` as `u16` rejects 70000 for free: the type is the validation.

## Try this
1. Review, lesson 04: rewrite `Inventory::sell` so its error type implements `Display`, like `ConfigError` here.
2. Allow `port` to be omitted with a default of 8080.
3. Add a `timeout_ms: u64` key.
4. Collect *all* errors instead of stopping at the first.
