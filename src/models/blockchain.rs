use super::block::Block;
use chrono::prelude::*;

type Blocks = Vec<Block>;

#[derive(Clone, Debug)]
pub struct Blockchain {
    pub genesis_block: Block,
    pub chain: Blocks,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default(),
        };

        let chain = vec![genesis_block.clone()];

        Blockchain {
            genesis_block,
            chain,
            difficulty,
        }
    }

    pub fn add_block(&mut self, _nonce: String) {
        let mut new_block = Block::new(
            self.chain.len() as u64,
            self.chain[&self.chain.len() - 1].hash.clone(),
        );

        new_block.mine(self);
        self.chain.push(new_block.clone());
        println!("New block added to the chain -> {:?}", new_block);
    }
}
