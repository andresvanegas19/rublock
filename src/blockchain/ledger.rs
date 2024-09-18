// manage the chain of blocks, ensuring the order and integrity of transactions.
use crate::consensus::challenge::Challenge;
use crate::consensus::pos::select_validator;
use crate::consensus::validator::Validator;
use crate::sentinel::SentinelBlockchain;
use crate::utils::current_timestamp;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Ledger {
    pub chain: Vec<Block>,
    // works as a blob
    pub current_transactions: Vec<Transaction>,
    pub validators: Vec<Validator>,
    pub sentinel: SentinelBlockchain,
}

impl Ledger {
    pub fn new() -> Self {
        let mut ledger = Ledger {
            chain: Vec::new(),
            current_transactions: Vec::new(),
            validators: Vec::new(),
            sentinel: SentinelBlockchain::new(),
        };

        // this is the genesis block, containing the previous block
        // as placeholder for a 32-byte hash
        ledger.create_genesis_block();

        ledger
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block {
            index: 0,
            timestamp: current_timestamp(),
            transactions: vec![],
            previous_hash: vec![0; 32],
            hash: vec![],
            nonce: 0,
        };

        genesis_block.hash = genesis_block.calculate_hash();

        self.chain.push(genesis_block);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.current_transactions.push(transaction)
    }

    pub fn create_block(&mut self, nonce: u64, previous_hash: Vec<u8>) -> &Block {
        let validator = select_validator(&self.validators)?;

        // TODO: for now it will be not necessary but in the future we will need to implement a PoW
        let nonce = 0; // In PoS, nonce might not be necessary

        let block = Block {
            index: self.chain.len() as u64,
            timestamp: current_timestamp(),
            transaction: self.current_transactions.clone(),
            // ensures that any alteration in a previous block would invalidate all subsequent blocks
            previous_hash,
            hash: vec![],
            nonce,
        };

        // ensure data integrity
        let block_hash = block.calculate_hash();
        let mut new_block = block;
        self.current_transactions.clear();
        self.chain.push(new_block);

        // Return the latest block
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        if self.sentinel.validate_block(&block) {
            self.chain.push(block);
            true
        }

        false
    }

    pub fn submit_challenge(&mut self, challenge: Challenge) {
        // TODO: Process the challenge
    }
}
