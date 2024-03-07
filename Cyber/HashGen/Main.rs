use sha2::{Sha256, Digest};

fn main() {
    // Input data to hash
    let data = "Rust Blockchain Data";

    // Create a Sha256 object
    let mut hasher = Sha256::new();

    // Process the input data
    hasher.update(data);

    // Acquire the resulting hash
    let result = hasher.finalize();

    // Convert the hash to a hexadecimal string for display
    println!("SHA-256 hash of '{}': \n{:x}", data, result);
}
