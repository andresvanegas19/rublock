// single block in the blockchain
use sha2::{Sha256, Digest};
use log::debug;

pub struct Block {
    index: u64,
    // TODO: if we need to do complex operation with timestamp, we can use chrono::DateTime
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    // Block's hash in Vectors of bytes to enhance the perf
    pub hash: Vec<u8>,
    // ensures that each block’s hash is unique, even if other contents are the same.
    // introduce an element of randomness that’s useful for certain protocols or security measures.
    // easier to switch between consensus mechanisms or implement hybrid models.
    pub nonce: u64,
}

impl Block {
    pub fn calculate_hash(&self) -> Vec<u8> {
        let data = format!(
            "{}{}{:?}{:?}{}",
            self.index,
            self.timestamp,
            self.transactions,
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
}
