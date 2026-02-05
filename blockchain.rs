use crate::block::Block;

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