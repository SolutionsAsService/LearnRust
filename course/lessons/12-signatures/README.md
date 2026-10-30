---
title: Digital signatures
track: Cryptography
summary: Generate an Ed25519 key pair, sign a message, and prove that any change to the message, or the wrong key, fails verification.
concepts: ed25519-dalek 2, SigningKey / VerifyingKey, OsRng, Cargo features
---
## What you'll build
A program that signs a message and shows that the original verifies while a tampered copy doesn't.

## Run it
```bash
cargo run -p lesson-12-signatures
cargo test -p lesson-12-signatures
```

## How it works
- A **signing key** stays private; its **verifying key** can be shared with anyone.
- `sign` produces a 64-byte signature tied to both the key and the exact message.
- `verify` fails if a single byte of the message changes, or if the wrong public key is used.
- `features = ["rand_core"]` in `Cargo.toml` switches on the crate's key generation from a random source.

## Where this came from
The original `Cyber/SigValidate` used `Keypair` and `PublicKey` from ed25519-dalek 1. Version 2 renamed them `SigningKey` and `VerifyingKey`, so the old code no longer compiles against the current crate. This lesson is the port.

## Try this
1. Review, lesson 11: sign the SHA-256 hex of a message instead of the message itself. Does verification still catch tampering?
2. Save the public key to a file and verify in a second program.
3. Sign the SHA-256 of a file (lesson 11) instead of the file itself.
4. Measure how many signatures per second your machine verifies.
