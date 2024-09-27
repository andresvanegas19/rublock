use log::debug;
use std::{collections::HashMap, sync::Arc};

use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
// use crate::consensus::challenge::Challenge;
// use crate::consensus::pos::select_highest_stake_validator;
use crate::consensus::validator::Validator;
use crate::utils::time::current_timestamp;

// use log::debug;

// manage the chain of blocks, ensuring the order and integrity of transactions.
// will be in charge on check transations, block and manage reputation of validators
// also will be in charge of slashing validators

pub struct Ledger {
    pub chain: Vec<Block>,
    // works as a blob
    pub current_transactions: Vec<Transaction>,
    // Maps addresses to their last known nonce
    pub nonces: HashMap<String, u64>,
    pub address_to_public_key: HashMap<String, Validator>,
}

impl Ledger {
    pub fn new() -> Self {
        let ledger: Ledger = Ledger {
            chain: Vec::new(),
            current_transactions: Vec::new(),
            nonces: HashMap::new(),
            address_to_public_key: HashMap::new(),
        };

        // this is the genesis block, containing the previous block
        // as placeholder for a 32-byte hash
        // ledger.create_genesis_block();

        debug!("Ledger created");

        ledger
    }
    // TODO: needs to finished
    // fn create_genesis_block(&mut self) {
    //     let genesis_block = Block {
    //         timestamp: current_timestamp(),
    //         previous_hash: vec![0; 32],
    //         hash: vec![],
    //         nonce: 0,
    //         penalties: todo!(),
    //         index: todo!(),
    //     };
    //     genesis_block.hash = genesis_block.calculate_hash();
    //     genesis_block.set_transactions(self.current_transactions.clone());
    //     self.chain.push(genesis_block);
    // }

    // TODO: move to box or use Arc to avoid clone and make it more efficient and safe
    pub fn add_validator(&mut self, validator: Validator) {
        self.address_to_public_key
            .insert(validator.get_string_address(), validator);
        debug!("Validator added to ledger");
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.current_transactions.push(transaction)
    }

    pub fn create_block(&mut self) -> &Block {
        // let validator: &Validator = match select_highest_stake_validator(&self.validators) {
        //     Ok(v) => v,
        //     Err(e) => {
        //         debug!("Error selecting highest stake validator: {}", e);
        //         return Err("No validators available".into());// or handle the error appropriately
        //     }
        // };

        // TODO: for now it will be not necessary but in the future we will need to implement a PoW
        let nonce = 0; // In PoS, nonce might not be necessary
        let transactions_arc = Arc::new(std::mem::take(&mut self.current_transactions));

        let mut block: Block = Block::new(
            self.chain.len() as u64,
            current_timestamp(),
            transactions_arc,
            vec![], // previous_hash
            nonce,
            Vec::new(), // penalties
        );

        // ensure data integrity
        block.calculate_hash();
        self.current_transactions.clear();
        self.chain.push(block);

        debug!("Chain created");

        // Return the latest block
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        self.chain.push(block);
        true
    }
}
