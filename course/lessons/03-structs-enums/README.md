---
title: Structs, enums and match
track: Foundations
summary: Model data with structs and enums, attach behaviour with impl, and let match prove you handled every case.
concepts: struct, enum, impl, match, derive, &'static str
---
## What you'll build
Points and shapes: a `Point` struct with a distance method, and a `Shape` enum whose variants each carry their own measurements.

## Run it
```bash
cargo run -p lesson-03-structs-enums
cargo test -p lesson-03-structs-enums
```

## How it works
- A `struct` groups named fields; `impl` adds methods. `self` (by value) works here because `Point` is `Copy`.
- An `enum` lists every shape a value can take, and each variant can hold different data.
- `match` is exhaustive: add a `Hexagon` variant and the compiler lists every `match` you forgot to update.
- `#[derive(Debug)]` gives you `{:?}` printing for free.

## Try this
1. Review, lesson 02: why can `distance_to` take `self` by value and still let you use `origin` afterwards? (Hint: `Copy`.)
2. Add `Shape::Square { side: f64 }` and follow the compiler errors until it builds.
3. Add a `perimeter` method.
4. Sort the shapes by area (hint: `sort_by` with `partial_cmp`).
