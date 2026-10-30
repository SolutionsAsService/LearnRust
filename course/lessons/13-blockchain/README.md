---
title: A toy blockchain
track: Cryptography
summary: Chain blocks by hash, mine them with proof of work, and write the validator that catches tampering.
concepts: sha2, serde + serde_json, proof of work, Result for validation, Vec manipulation
---
## What you'll build
A blockchain that mines blocks to a chosen difficulty, prints them as JSON, and detects when anyone edits history.

## Run it
```bash
cargo run --release -p lesson-13-blockchain
cargo test -p lesson-13-blockchain
```

## How it works
- Each block's hash covers its index, time, data, the previous block's hash and a `nonce`.
- **Mining** tries nonces until the hash starts with `difficulty` zeros. Each extra zero means about 16 times more tries on average, because a hex digit has 16 possible values and only one of them is `0`.
- Because every block stores the previous hash, changing old data breaks every link after it, which `validate` reports.
- `#[derive(Serialize)]` plus `serde_json` turns a block into JSON with one call.
- Use `--release` to run. On the machine that built this site, the whole program took about 11 s in a debug build and 2.4 s with `--release` (two runs each); your numbers will differ.

## Where this came from
The original `Basic_blockchain.rs` mined blocks but had no validator, no Cargo project, and its hash input didn't include the block index. This version adds all three and the tests.

## Try this
1. Review, lesson 09: mine with several threads, each trying a different range of nonces, and stop them all when one finds a hash.
2. Time mining at difficulty 3, 4 and 5.
3. Sign each block's data with lesson 12's keys and check signatures in `validate`.
4. Store a `Vec` of transactions per block instead of a string.
