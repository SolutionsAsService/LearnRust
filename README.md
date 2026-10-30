# LearnRust

**Learn Rust by owning it.** A free course of 13 small, tested Rust programs in three tracks, from ownership to a toy blockchain, with a website generated from the course itself.

| Track | Lessons |
|---|---|
| Foundations | 01 Hello, Cargo · 02 Ownership, moves and borrows · 03 Structs, enums and match · 04 Collections: a bookstore inventory · 05 Error handling with Result · 06 Traits and generics · 07 Iterators and closures · 08 Modules and testing |
| Systems | 09 Fearless concurrency · 10 A TCP echo server |
| Cryptography | 11 Hashing with SHA-256 · 12 Digital signatures · 13 A toy blockchain |

## Use the course

```bash
cd course
cargo test --workspace            # 35 tests across all lessons
cargo run -p lesson-02-ownership  # run one lesson
```

Each lesson in `course/lessons/NN-name/` is a Cargo package with a `README.md` (what you'll build, how it works, things to try), its source, its tests, and `OUTPUT.txt` (what it really printed).

## Repository layout

```
course/                 Cargo workspace: one package per lesson
  lessons/NN-name/      README.md, src/, tests/, OUTPUT.txt
site/                   the website (committed build output, static)
tools/
  build_site.py         builds site/ from course/ (pure Python 3)
  capture_outputs.py    runs each lesson and saves OUTPUT.txt
  home.html, setup.md   hand-written parts of the home and setup pages
index.html              redirect to site/
vercel.json             deploy config (outputDirectory: site)
.github/workflows/      cargo test on every push
```

The Rust workspace lives in `course/`, not at the root, so static hosts (Vercel, Shadw) don't mistake the repository for a Rust app to build.

## Rebuild the website

```bash
python tools/capture_outputs.py   # after changing a lesson's program
python tools/build_site.py
```

Lesson pages, the course map and the home page's counts are generated from the lessons, so the site can't drift from the code.

## Where the lessons came from

This repository started as six loose programs. They are now lessons, fixed and tested:

| Original | Lesson | Fixed |
|---|---|---|
| `bookstore_inventory.rs` | 04 | panicked on a non-numeric stock; logic now separate from input |
| `Cyber/TCPServer/Main.rs` | 10 | looped forever on disconnect; `write` → `write_all`; errors handled |
| `Cyber/HashGen/Main.rs` | 11 | no Cargo project; now hashes files too, checked against a test vector |
| `Cyber/SigValidate/Main.rs` | 12 | used the ed25519-dalek 1 API, removed in 2; ported |
| `Basic_blockchain.rs` | 13 | no validator, index missing from the hash; both added |

The originals are on the `legacy-2026-10-30` branch.

## Deploy

Static, no install or build step on the host. The root `vercel.json` serves `site/`; `site/vercel.json` covers a Root Directory set to `site`.

## Licence

MIT (see `LICENSE`). Contributions welcome (see `CONTRIBUTING.md`). Not affiliated with the Rust Foundation; "Rust" is a trademark of the Rust Foundation.
