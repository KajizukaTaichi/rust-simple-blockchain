use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, String::new(), "Genesis Block".to_string());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: String) -> Option<()> {
        let previous_block = self.chain.last()?;
        let new_block = Block::new(previous_block.index + 1, previous_block.hash.clone(), data);

        if new_block.is_valid(previous_block) {
            self.chain.push(new_block);
            Some(())
        } else {
            None
        }
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if !current_block.is_valid(previous_block) {
                return false;
            }
        }
        true
    }
}
