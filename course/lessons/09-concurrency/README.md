---
title: Fearless concurrency
track: Systems
summary: Split work across threads, pass results over channels, and share a map safely with Arc and Mutex.
concepts: thread::scope, thread::spawn, mpsc channels, Arc, Mutex, Send/Sync
---
## What you'll build
A parallel sum of a million numbers, and a word counter where several threads update one shared map.

## Run it
```bash
cargo run -p lesson-09-concurrency
cargo test -p lesson-09-concurrency
```

## How it works
- `thread::scope` lets threads borrow `numbers` safely, because they're guaranteed to finish before the scope ends.
- A channel (`mpsc`) moves values between threads; dropping every sender ends the receiver's loop.
- `Arc` shares ownership across threads; `Mutex` makes sure only one thread changes the map at a time.
- Forget the `Mutex` and the program doesn't compile. Data races are compile errors in safe Rust, which is what "fearless concurrency" means.

## Try this
1. Review, lesson 02: why must the closure passed to `thread::spawn` be `move`? What would it borrow otherwise?
2. Time `parallel_sum` with 1, 2, 4 and 8 workers (`std::time::Instant`).
3. Replace the shared map with per-thread maps merged at the end. Which is faster?
4. Remove `Arc::clone` and read the compiler error.
