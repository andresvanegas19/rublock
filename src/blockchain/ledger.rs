// manage the chain of blocks, ensuring the order and integrity of transactions.
use std::time::{SystemTime, UNIX_EPOCH};
pub struct Ledger {
    pub chain: Vec<Block>,
    // works as a blob
    pub current_transactions: Vec<Transaction>,
}

impl Ledger {
    pub fn new() -> Self {
        let mut ledger = Ledger {
            chain: Vec::new(),
            current_transactions: Vec::new(),
        };

        ledger.create_block(0, vec![0; 32]);

        ledger
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.current_transactions.push(transaction)
    }

    fn current_timestamp() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        since_the_epoch.as_secs()
    }

    pub fn create_block(&mut self, nonce: u64, previous_hash: Vec<u8>) -> &Block {
        let block = Block {
            index: self.chain.len() as u64,
            timestamp: current_timestamp(),
            transaction: self.current_transactions.clone(),
            // ensures that any alteration in a previous block would invalidate all subsequent blocks
            previous_hash,
            hash: vec![],
            nonce,
        };

        let block_hash = block.calculate_hash();
        let mut new_block = block;
        self.current_transactions.clear();
        self.chain.push(new_block);

        // Return the latest block
        self.chain.last().unwrap()
    }
}
