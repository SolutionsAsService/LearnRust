//! Lesson 11: SHA-256 hashing of text and files.
//! Grown from the repository's original `Cyber/HashGen/Main.rs`.

use sha2::{Digest, Sha256};
use std::{env, fs, io};

/// Hex-encode a SHA-256 digest of some bytes.
fn sha256_hex(data: &[u8]) -> String {
    let digest = Sha256::digest(data);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// Hash a file without reading it all into memory.
fn sha256_file(path: &str) -> io::Result<String> {
    let mut hasher = Sha256::new();
    let mut file = fs::File::open(path)?;
    io::copy(&mut file, &mut hasher)?;
    Ok(hasher.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

fn main() {
    match env::args().nth(1) {
        Some(path) => match sha256_file(&path) {
            Ok(h) => println!("{h}  {path}"),
            Err(e) => eprintln!("{path}: {e}"),
        },
        None => {
            for text in ["Rust Blockchain Data", "Rust Blockchain Datb"] {
                println!("{}  {text:?}", sha256_hex(text.as_bytes()));
            }
            println!("(one changed letter, a completely different hash: the avalanche effect)");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_the_published_test_vector() {
        // FIPS 180-2 test vector for "abc"
        assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }

    #[test]
    fn file_and_bytes_agree() {
        let path = std::env::temp_dir().join("learnrust-hash-test.txt");
        std::fs::write(&path, b"abc").unwrap();
        assert_eq!(sha256_file(path.to_str().unwrap()).unwrap(), sha256_hex(b"abc"));
    }
}
