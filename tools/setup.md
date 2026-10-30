## 1. Install Rust
Install `rustup`, which manages the compiler (`rustc`), the build tool (`cargo`) and updates. Follow the instructions at [rustup.rs](https://rustup.rs). On macOS and Linux it is one command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Windows, download and run `rustup-init.exe` from the same site. Then check it worked:

```bash
rustc --version
cargo --version
```

## 2. Get the course
```bash
git clone https://github.com/SolutionsAsService/LearnRust
cd LearnRust/course
```

## 3. Run every test
The first run downloads a few crates (`sha2`, `ed25519-dalek`, `serde`) and compiles everything, which takes a minute or two.

```bash
cargo test --workspace
```

## 4. Run one lesson
Each lesson is a package named `lesson-` plus its folder name:

```bash
cargo run -p lesson-01-hello-cargo
cargo test -p lesson-02-ownership
```

## 5. An editor that helps
Any editor works. Visual Studio Code with the **rust-analyzer** extension shows types, errors and documentation as you type, which makes the borrow checker much friendlier.

## When something goes wrong
- `error: linker 'cc' not found` on Linux: install your distribution's build tools (for example `build-essential`).
- On Windows, `rustup` may ask for the Visual Studio C++ build tools; follow its prompt.
- Read compiler errors from the top. The first one is usually the real problem, and `rustc --explain E0382` (with your error's code) prints a full explanation.
