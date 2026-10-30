---
title: Traits and generics
track: Foundations
summary: Share behaviour across types with traits, write one function for many types with generics, and mix types with trait objects.
concepts: trait, default methods, impl Trait for, generics, trait bounds, Box<dyn Trait>
---
## What you'll build
A shelf of books and films that all know how to describe themselves, plus a generic `largest` that works on any comparable list.

## Run it
```bash
cargo run -p lesson-06-traits-generics
cargo test -p lesson-06-traits-generics
```

## How it works
- A **trait** is a set of methods a type promises to have. `describe` has a default body; `Book` overrides it, `Film` doesn't.
- `largest<T: PartialOrd + Copy>` is written once and compiled for each type you call it with (static dispatch).
- `Vec<Box<dyn Describe>>` holds different types side by side and calls the right method at run time (dynamic dispatch).
- Returning `Option<T>` makes "the list was empty" a value, not a crash.

## Try this
1. Review, lesson 05: make `largest` return `Result<T, &'static str>` instead of `Option<T>`. Which do you prefer, and why?
2. Add a `Podcast` type and put it on the shelf.
3. Implement `std::fmt::Display` for `Book` so `println!("{book}")` works.
4. Make `largest` return a reference instead of requiring `Copy`.
