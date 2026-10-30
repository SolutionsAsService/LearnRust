---
title: Hashing with SHA-256
track: Cryptography
summary: Use your first crate, hash text and files, check against a published test vector, and see the avalanche effect.
concepts: crates.io dependencies, sha2, traits from crates (Digest), io::copy, hex formatting
---
## What you'll build
A command-line hasher: `cargo run -p lesson-11-hashing -- <file>` prints the file's SHA-256; with no argument it shows how one changed letter changes the whole hash.

## Run it
```bash
cargo run -p lesson-11-hashing
cargo run -p lesson-11-hashing -- Cargo.toml
cargo test -p lesson-11-hashing
```

## How it works
- `sha2 = "0.10"` in `Cargo.toml` pulls the crate from crates.io; `cargo` downloads and builds it.
- `Sha256` implements `Write`, so `io::copy` can stream a file of any size through it.
- `{b:02x}` prints each byte as two hex digits.
- The test checks the published FIPS 180-2 vector for `"abc"`, so you know the hashing is right, not just consistent.

## Where this came from
The original `Cyber/HashGen` had the code but no Cargo project, so it couldn't be built as committed. Here it's a full crate with tests.

## Try this
1. Review, lesson 07: hash every line of a file and print only the hashes that start with `00`, using one iterator chain.
2. Hash every file in a folder and print the results sorted by name.
3. Add a `--check <hash>` flag that says whether a file matches.
4. Compare SHA-256 and SHA-512 output lengths.
