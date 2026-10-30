//! Lesson 12: digital signatures with Ed25519.
//! Grown from the repository's original `Cyber/SigValidate/Main.rs` (ported to ed25519-dalek 2).

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

/// Sign a message with a private key.
fn sign(key: &SigningKey, message: &[u8]) -> Signature {
    key.sign(message)
}

/// Anyone with the public key can check the signature.
fn is_authentic(public: &VerifyingKey, message: &[u8], sig: &Signature) -> bool {
    public.verify(message, sig).is_ok()
}

fn main() {
    let signing_key = SigningKey::generate(&mut OsRng);
    let public = signing_key.verifying_key();

    let message = b"Rust Blockchain Trust";
    let sig = sign(&signing_key, message);
    println!("public key: {}", hex(public.as_bytes()));
    println!("signature:  {}", hex(&sig.to_bytes()));

    println!("original message verifies: {}", is_authentic(&public, message, &sig));
    println!("tampered message verifies: {}", is_authentic(&public, b"Rust Blockchain Trust!", &sig));
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genuine_message_verifies() {
        let key = SigningKey::generate(&mut OsRng);
        let sig = sign(&key, b"pay 10");
        assert!(is_authentic(&key.verifying_key(), b"pay 10", &sig));
    }

    #[test]
    fn tampered_message_fails() {
        let key = SigningKey::generate(&mut OsRng);
        let sig = sign(&key, b"pay 10");
        assert!(!is_authentic(&key.verifying_key(), b"pay 1000", &sig));
    }

    #[test]
    fn someone_elses_key_fails() {
        let (a, b) = (SigningKey::generate(&mut OsRng), SigningKey::generate(&mut OsRng));
        assert!(!is_authentic(&b.verifying_key(), b"hi", &sign(&a, b"hi")));
    }
}
