use ed25519_dalek::{Keypair, Signer, Verifier, Signature, PublicKey};
use rand::rngs::OsRng;

fn main() {
    // Step 1: Generate a key pair
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    // Step 2: Create a message
    let message: &[u8] = b"Rust Blockchain Trust";

    // Step 3: Sign the message
    let signature: Signature = keypair.sign(message);

    // Step 4: Verify the signature
    match keypair.public.verify(message, &signature) {
        Ok(_) => println!("Signature verified successfully!"),
        Err(_) => println!("Failed to verify signature."),
    }

    // Optional: Demonstrate failure case
    let different_message: &[u8] = b"Tampered Message";
    match keypair.public.verify(different_message, &signature) {
        Ok(_) => println!("Signature verified successfully!"),
        Err(_) => println!("Failed to verify signature for the tampered message."),
    }
}
