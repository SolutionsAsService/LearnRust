# DESIGN-LOG — LearnRust (reorganize, expand, frontend; dada-lite)

## Plan
- **Brief:** reorganize the repo, expand it, and build a nice frontend with dada.
- **Mode:** Adaptive (MAYA): beginners deciding whether to learn Rust, and learners mid-course.
- **Pages:** home, course map, 13 lesson pages, set up, 404, all generated from the course.
- **Seed:** stance `The Art of Looking Sideways (Alan Fletcher)`; rules `The Elements of Typographic Style (Robert Bringhurst)`, `WCAG 2.2 Contrast and Perceivability (W3C)`; breaker `Saul Bass: A Life in Film and Design (Jennifer Bass, Pat Kirkham)` (one moving picture that is the whole idea); counterpoint `Interaction of Color (Josef Albers)` via `wcag -.->|contrast floor constrains| albers`.
- **Premortem:** six loose `.rs` files, no Cargo project (nothing built as committed), no README. The TCP server looped forever on disconnect; the signature program used the ed25519-dalek 1 API removed in 2; the inventory panicked on bad input; the blockchain had no validator.

## Moves
- **M1** KEEP. Reorganize: a Cargo workspace in `course/` (not the root, so static hosts don't detect a Rust app), one package per lesson with README, source, tests and captured output; originals on `legacy-2026-10-30`.
- **M2** KEEP. Expand: 13 lessons in three tracks; the six originals became lessons 04, 10–13, each fixed (see README table).
- **M3** KEEP. Evidence on the page: test counts come from the source, output from a real run (`tools/capture_outputs.py`), warnings from a real build, the compiler error from real rustc.
- **M4** BREAK. **The move:** the home hero animates ownership: `s` owns "hello" → `let t = s` moves it (s shown "moved") → using `s` triggers the real `error[E0382]` → `s.clone()` fixes it, with real cargo output.
- **M5** ADAPT (measured). Grid tracks sized to their widest code line, widening pages (lessons to 793 px at 375, hero to 1,111 px at 1,009): `minmax(0, 1fr)` on the code column and the hero figure.
- **M6** ADAPT. Code font ligatures turned `->` into an arrow; ligatures off so learners see what they type.
- **M7** BREAK (ambition push, Shklovsky R1). **Guess first:** the hero stops before the error and asks "Which line won't compile?" Line 4 is right; line 3 gets "close: that's where the move happens, but the error is reported where `s` is used again."

## Measurements
| Check | Result |
|---|---|
| Tests | `cargo test --workspace`: 35 passed (33 `#[test]` + 2 doc tests), matching the site's derived count |
| Warnings | `cargo build --workspace`: 0 (counted by the site build) |
| E0382 text | captured from rustc 1.99.1 on a probe crate; shown shortened, and labelled so |
| Release vs debug (lesson 13) | 11.4 s / 10.9 s debug, 2.3 s / 2.6 s release (two runs each) |
| Overflow | 375 px: every page scrollWidth = clientWidth after M5; 1,009 px: 1,009 |
| Hero | guess shown at step 2; clicking line 3 → step 3 with the "close" reply; aria-live off while playing, polite after |

## Panel R1 (one sub-agent, five seats incl. a retrieval-practice seat; filed in dada-lessons)
| Seat | Verdict | Resolution |
|---|---|---|
| Provocateur (Shklovsky) | OBJECT | ADAPT: error step holds 6 s; M7 guess-first. |
| Usability (Krug) | OBJECT | ADAPT: every lesson's "Run it" opens with where to run (the `course/` folder) and a link to Set up. |
| Evidence (Cairo) | OBJECT | ADAPT: "many times faster" replaced by the measured 11 s vs 2.4 s; "16×" now states its reason; "rustc output, shortened" on the error; warnings measured by the build. |
| Inclusion (Mace) | OBJECT | ADAPT: aria-live off while autoplaying, polite when the visitor drives; line numbers and diagram labels raised to #8a9099; a Next step button. |
| Retrieval practice (Roediger & Karpicke, Bjork) | OBJECT | ADAPT: output hidden behind "Predict first, then open"; lessons 03–13 open Try this with a "Review, lesson NN" exercise that reuses an earlier idea. |
Andon: none.

## Stop check (bounded)
- 1 panel round + measured re-render. S2/S3 not met (bounded run).
- Open: no answers or checks for the Try-this exercises yet; GitHub Actions workflow added (`cargo test` on push) but not run here.
