// struct that represents a transfer of value or data between participants in the network

// use serde::{Deserialize, Serialize};

pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

