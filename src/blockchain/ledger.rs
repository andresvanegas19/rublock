use std::collections::HashMap;

use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use crate::consensus::challenge::Challenge;
use crate::consensus::pos::select_highest_stake_validator;
use crate::consensus::validator::Validator;
use crate::utils::time::current_timestamp;

use ed25519_dalek::PublicKey;

// manage the chain of blocks, ensuring the order and integrity of transactions.
// will be in charge on check transations, block and manage reputation of validators
// also will be in charge of slashing validators

pub struct Ledger {
    pub chain: Vec<Block>,
    // works as a blob
    pub current_transactions: Vec<Transaction>,
    pub validators: Vec<Validator>,
    // Maps addresses to their last known nonce
    pub nonces: HashMap<String, u64>,
    pub address_to_public_key: HashMap<String, PublicKey>,
}

impl Ledger {
    pub fn new() -> Self {
        let mut ledger = Ledger {
            chain: Vec::new(),
            current_transactions: Vec::new(),
            validators: Vec::new(),
            nonces: HashMap::new(),
            address_to_public_key: HashMap::new(),
        };

        // this is the genesis block, containing the previous block
        // as placeholder for a 32-byte hash
        // ledger.create_genesis_block();

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

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.current_transactions.push(transaction)
    }

    pub fn create_block(&mut self, nonce: u64, previous_hash: Vec<u8>) -> &Block {
        // let validator = select_highest_stake_validator(&self.validators, "")?;

        // TODO: for now it will be not necessary but in the future we will need to implement a PoW
        let nonce = 0; // In PoS, nonce might not be necessary

        let block = Block {
            index: self.chain.len() as u64,
            timestamp: current_timestamp(),
            // ensures that any alteration in a previous block would invalidate all subsequent blocks
            previous_hash,
            hash: vec![],
            nonce,
        };

        block.set_transactions(Arc::new(self.current_transactions.clone()));

        // ensure data integrity
        let block_hash = block.calculate_hash();
        let mut new_block = block;
        self.current_transactions.clear();
        self.chain.push(new_block);

        // Return the latest block
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        if self.validate_block(&block) {
            self.chain.push(block);

            true;
        }

        false
    }

    // DOC: Validate transactions and blocks, and slash validators for malicious behavior
    // Rate penalty
    // 1. Validator's stake is reduced by 25%.
    // 2. Validator's reputation is reduced by 1.
    // 3. Validator is removed from the validator set.
    // 4. Validator is reported to the network.
    // cases where we need to slash a validator and his respective penalty
    //      - Validator signs two conflicting blocks. (1)
    //      - Any actions that violate the agreed-upon protocol rules. (1)
    //      - Validator signs votes that conflict in a way that one vote “surrounds” another. (2)
    //      - Validator signs a block that is not part of the longest chain. (3)
    //      - Failing to participate in the consensus process when required. (4)
    pub fn apply_penalties(&mut self, block: &Block) {
        for penalty in &block.penalties {
            if let Some(validator) = self
                .validators
                .iter_mut()
                .find(|v| &v.address == penalty.validator_address)
            {
                penalty.apply_penalties(validator);
                debug!(
                    "Validator {} has been penalized by the block {}. New stake: {}, New reputation: {}",
                    penalty.validator_address, penalty.block_index, validator.stake, validator.reputation
                );
            }
        }
    }

    pub fn validate_transaction(&self, transaction: &Transaction) -> bool {
        true
    }

    pub fn is_valid_nonce(self, transaction: &Transaction) -> bool {
        let expected_nonce = self.nonces.get(&transaction.sender).unwrap_or(&0) + 1;
        transaction.nonce == expected_nonce
    }

    pub fn update_nonce(self, transaction: &Transaction) {
        self.nonces
            .insert(transaction.sender.clone(), transaction.nonce);
    }

    pub fn validate_block(&self, block: &Block) -> bool {
        // TODO: Verify previous_hash, hash validity, and transaction correctness. Liveness Faults: Failing to participate when required.
        //     Independent Verification: Each node verifies blocks and transactions independently.
        // •	Cross-Node Communication: Nodes share information about suspicious activities.
        // •	Consensus Mechanisms: Protocols like Byzantine Fault Tolerance (BFT) help in reaching agreement even with some malicious actors.
        true
    }

    pub fn slash_validator(&mut self, validator_address: &String) {
        let Some(guilty_validator) = self
            .validators
            .iter()
            // the find method is borrowing the address field for comparison, rather than taking ownership of it
            .find(|v| &v.address == validator_address);

        if (guilty_validator.is_some()) {
            // Reduce the validator's stake as a penalty
            validator.stake /= 4;
            debug!(
                "Validator {} has been slashed. New stake: {}",
                validator_address, validator.stake
            );
        }
    }

    pub fn update_reputation(
        validators: Vec<Validator>,
        validator_address: &String,
        reputation: i32,
        negative: bool,
    ) {
        let Some(validator) = validators
            .iter_mut()
            .find(|v| &v.address == validator_address);

        if (validator.is_some()) {
            if negative {
                validator.reputation -= reputation;
            } else {
                validator.reputation += reputation;
            }
            debug!(
                "Validator {} reputation updated. New reputation: {}",
                validator_address, validator.reputation
            );
        }
    }

    pub fn report_malicious_validator(&self, evidence: MaliciousEvidenceValidator) {}

    pub fn submit_challenge(&mut self, challenge: Challenge) {
        // TODO: Process the challenge
    }

    // DOC: Fork Choice Rule (FCR) - Longest Chain Rule (LCR)
    // determine which chain a node should consider the valid one when multiple competing chains
    // (forks) exist. They ensure network consensus and help resolve conflicts.
    // GHOST Protocol - Greedy Heaviest Observed SubTree vairant of GHOST -> LMD-GHOST
    pub fn select_chain(&self, chains: Vec<Vec<Block>>) -> &[Block] {
        if chains.is_empty() {
            return vec![];
        }

        // Find the longest chain
        let mut longest_chain = &chains[0];
        let mut max_length = longest_chain.len();
        let mut max_weight = self.calculate_chain_weight(longest_chain);

        for chain in &chains[1..] {
            let length = chain.len();
            let weight = self.calculate_chain_weight(chain);
            // TODO : Add this conditionals also Total Stake Weight - Block Timestamps
            if legth > max_length || (length == max_length && weight > max_weight) {
                max_length = length;
                max_weight = weight;
                longest_chain = chain;
            }
        }

        longest_chain
    }

    fn calculate_chain_weight(&self, chain: &Vec<Block>) -> u64 {
        chain.iter().map(|b| b.transactions.len()).sum()
    }

    // DOC: logic for transactions
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        if !transaction.verify() {
            return Err("Transaction signature is invalid".to_string());
            debug!("Transaction signature is invalid");
        }

        if !self.is_valid_nonce(&transaction) {
            return Err("Invalid transaction nonce".to_string());
            debug!("Invalid transaction nonce");
        }
    }

    // DOC: handles adding or remove validators from the network
    pub fn get_public_key_from_address(&self, address: &String) -> Option<PublicKey> {
        self.address_to_public_key.get(address).cloned()
    }
}
