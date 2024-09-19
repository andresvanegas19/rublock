use log::debug;
use sha2::{Digest, Sha256};
use std::sync::Arc;

use crate::blockchain::penalty::Penalty;
use crate::blockchain::transaction::Transaction;

pub struct Block {
    index: u64,
    // TODO: if we need to do complex operation with timestamp, we can use chrono::DateTime
    pub timestamp: u64,
    pub transactions: Arc<Vec<Transaction>>,
    pub previous_hash: String,
    // Block's hash in Vectors of bytes to enhance the perf
    pub hash: Vec<u8>,
    // ensures that each block’s hash is unique, even if other contents are the same.
    // introduce an element of randomness that’s useful for certain protocols or security measures.
    // easier to switch between consensus mechanisms or implement hybrid models.
    pub nonce: u64,
    pub penalties: Vec<Penalty>,
}

impl Block {
    pub fn new(
        timestamp: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        nonce: u64,
        penalties: Vec<Penalty>,
    ) -> Self {
        let hash = vec![];
        // TOOD: finding a way to calculate the number of the block
        let index = 0;

        Block {
            index,
            timestamp,
            transactions: Arc::new(transactions),
            previous_hash,
            hash,
            nonce,
            penalties,
        }
    }

    pub fn calculate_hash(&self) -> Vec<u8> {
        let data = format!(
            "{}{}{:?}{:?}{}",
            self.index,
            self.timestamp,
            self.hashing_transactions(),
            self.previous_hash,
            self.nonce
        );

        debug!("Data to hash: {}", data);

        // produce a fixed-size output that is highly sensitive to input changes
        // Simple checksums can be more easily manipulated to produce the same
        // output from different inputs
        let mut hasher = Sha256::new();

        hasher.update(data.as_bytes());

        let result = hasher.finalize();
        debug!("Hash result: {:?}", result);

        result.to_vec()
    }

    // get in string all the signature of the transactions
    fn hashing_transactions(&self) -> String {
        self.transactions
            .iter()
            .map(|transaction| {
                String::from_utf8(transaction.signature.clone()).expect("Invalid UTF-8 sequence")
            })
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    pub fn set_transactions(&mut self, transactions: Arc<Vec<Transaction>>) {
        self.transactions = transactions;
    }
}
