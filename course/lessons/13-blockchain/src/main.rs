//! Lesson 13: a toy proof-of-work blockchain, with validation.
//! Grown from the repository's original `Basic_blockchain.rs`.

use serde::Serialize;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    prev_hash: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn hash_of(index: u64, timestamp: u64, data: &str, prev_hash: &str, nonce: u64) -> String {
        let digest = Sha256::digest(format!("{index}|{timestamp}|{data}|{prev_hash}|{nonce}"));
        digest.iter().map(|b| format!("{b:02x}")).collect()
    }

    fn compute_hash(&self) -> String {
        Self::hash_of(self.index, self.timestamp, &self.data, &self.prev_hash, self.nonce)
    }

    /// Proof of work: find a nonce whose hash starts with `difficulty` zeros.
    fn mine(index: u64, data: &str, prev_hash: &str, difficulty: usize) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let target = "0".repeat(difficulty);
        let mut nonce = 0;
        loop {
            let hash = Self::hash_of(index, timestamp, data, prev_hash, nonce);
            if hash.starts_with(&target) {
                return Block { index, timestamp, data: data.into(), prev_hash: prev_hash.into(), nonce, hash };
            }
            nonce += 1;
        }
    }
}

#[derive(Debug, Serialize)]
struct Chain {
    difficulty: usize,
    blocks: Vec<Block>,
}

impl Chain {
    fn new(difficulty: usize) -> Chain {
        Chain { difficulty, blocks: vec![Block::mine(0, "genesis", "0", difficulty)] }
    }

    fn add(&mut self, data: &str) -> &Block {
        let prev = self.blocks.last().expect("a chain always has a genesis block");
        let block = Block::mine(prev.index + 1, data, &prev.hash, self.difficulty);
        self.blocks.push(block);
        self.blocks.last().unwrap()
    }

    /// Every hash must be correct, meet the difficulty, and point at the block before it.
    fn validate(&self) -> Result<(), String> {
        let target = "0".repeat(self.difficulty);
        for (i, b) in self.blocks.iter().enumerate() {
            if b.hash != b.compute_hash() {
                return Err(format!("block {i}: hash doesn't match its contents"));
            }
            if !b.hash.starts_with(&target) {
                return Err(format!("block {i}: not mined to difficulty {}", self.difficulty));
            }
            if i > 0 && b.prev_hash != self.blocks[i - 1].hash {
                return Err(format!("block {i}: broken link to block {}", i - 1));
            }
        }
        Ok(())
    }
}

fn main() {
    let mut chain = Chain::new(4);
    for data in ["Alice pays Bob 5", "Bob pays Carol 2"] {
        let b = chain.add(data);
        println!("mined block {} with nonce {:>6}: {}", b.index, b.nonce, b.hash);
    }
    println!("valid: {:?}", chain.validate());

    chain.blocks[1].data = "Alice pays Bob 500".into(); // tamper
    println!("after tampering: {:?}", chain.validate());

    println!("{}", serde_json::to_string_pretty(&chain.blocks[0]).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fresh_chain_is_valid() {
        let mut c = Chain::new(2);
        c.add("a");
        c.add("b");
        assert_eq!(c.validate(), Ok(()));
    }

    #[test]
    fn tampering_is_detected() {
        let mut c = Chain::new(2);
        c.add("a");
        c.blocks[1].data = "z".into();
        assert!(c.validate().unwrap_err().contains("block 1"));
    }

    #[test]
    fn relinking_is_detected() {
        let mut c = Chain::new(2);
        c.add("a");
        c.add("b");
        c.blocks.remove(1);
        assert!(c.validate().is_err());
    }
}
