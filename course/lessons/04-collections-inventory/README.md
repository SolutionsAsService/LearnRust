---
title: Collections: a bookstore inventory
track: Foundations
summary: Keep records in a HashMap, return a Result instead of panicking, and keep the logic testable apart from input and output.
concepts: HashMap, entry API, Vec, sort_by, Result, ? operator, stdin
---
## What you'll build
An interactive inventory: `add <title> | <author> | <copies>`, `sell <title>`, `list`, `exit`.

## Run it
```bash
cargo run -p lesson-04-collections-inventory
cargo test -p lesson-04-collections-inventory
```

## How it works
- `HashMap<String, Book>` keys books by lower-cased title, so "Dune" and "dune" are the same book.
- The **entry API** (`entry().and_modify().or_insert()`) adds or tops up in one expression.
- `sell` returns `Result<u32, SellError>` instead of printing, so the caller decides what to say and tests can check every outcome.
- `?` returns early with the error; `ok_or` turns a missing value (`None`) into an error.

## Where this came from
The repository's first version, `bookstore_inventory.rs`, panicked on a non-numeric stock and mixed input with logic. This lesson keeps its idea and fixes both.

## Try this
1. Review, lesson 03: replace `SellError` with your own enum that also has a `Discontinued` variant, and handle it in every `match`.
2. Add a `restock <title> <n>` command.
3. Save the inventory to a file on exit and load it on start.
4. Add a `search <word>` command that matches authors too.
