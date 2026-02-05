use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let data = serde_json::json!({
            "index": self.index,
            "timestamp": self.timestamp,
            "data": self.data,
            "previous_hash": self.previous_hash,
            "nonce": self.nonce,
        });
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let prefix = "0".repeat(difficulty);
        while !self.hash.starts_with(&prefix) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut genesis_block = Block::new(0, String::from("Genesis Block"), String::from("0"));
        genesis_block.mine_block(difficulty);
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let mut new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.previous_hash.clone(),
        );
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut chain = Blockchain::new(2);
    
    println!("Mining block 1...");
    chain.add_block(String::from("First Transaction"));
    
    println!("Mining block 2...");
    chain.add_block(String::from("Second Transaction"));

    println!("Full chain data:");
    println!("{:?}", chain.chain);
    
    println!("Is chain valid? {}", chain.is_chain_valid());
}