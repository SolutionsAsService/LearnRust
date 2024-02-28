use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    timestamp: u64,
    data: String,
    prev_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(data: String, prev_hash: String) -> Block {
        let mut block = Block {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            data,
            prev_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.timestamp.to_string() + &self.data + &self.prev_hash + &self.nonce.to_string());
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: usize) {
        while &self.hash[..difficulty] != &"0".repeat(difficulty) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::new("Genesis Block".to_string(), "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
            difficulty: 4,
        }
    }

    fn add_block(&mut self, data: String) {
        let prev_hash = self.chain.last().unwrap().hash.clone();
        let mut new_block = Block::new(data, prev_hash);
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }
}

fn main() {
    let mut my_blockchain = Blockchain::new();

    my_blockchain.add_block("Block 1 Data".to_string());
    my_blockchain.add_block("Block 2 Data".to_string());

    println!("{:#?}", my_blockchain);
}
