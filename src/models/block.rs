use super::blockchain::Blockchain;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub proof_of_work: u64,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String) -> Self {
        Block {
            index,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash,
            hash: String::default(),
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut block_data = self.clone();
        block_data.hash = String::default();
        let serialized_block_data = serde_json::to_string(&block_data).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn mine(&mut self, blockchain: &mut Blockchain) {
        loop {
            if self.hash.starts_with(&"0".repeat(blockchain.difficulty)) {
                break;
            } else {
                self.proof_of_work += 1;
                self.hash = self.calculate_hash();
                println!("{}", self.hash)
            }
        }
    }
}
