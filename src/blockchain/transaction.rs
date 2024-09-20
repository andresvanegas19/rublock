// struct that represents a transfer of value or data between participants in the network
// use crate::crypto::keypair::KeyPairRublock;

// use super::ledger::Ledger;

use curve25519_dalek::{RistrettoPoint, Scalar};

use crate::crypto::keypair::KeyPairRublock;

pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub nonce: u64,
    pub signature: Vec<u8>,
    // TODO: implement the state of fees to avoid spam in the nextwork
    pub fee: u64,
}

impl Transaction {
    pub fn new(
        self,
        sender: String,
        receiver: String,
        amount: u64,
        nonce: u64,
        fee: u64,
        public_key: RistrettoPoint,
        secret_key: Scalar,
    ) -> Self {
        let mut tx = Transaction {
            sender,
            receiver,
            amount,
            nonce,
            signature: Vec::new(),
            fee,
        };

        let tx_data = self.serialize_for_signing();
        let (ristretto_point, _) = KeyPairRublock::sign(tx_data.as_bytes(), secret_key);

        tx.signature = [
            tx_data.as_bytes(),                             // message: &[u8],
            public_key.compress().to_bytes().as_ref(),      // public_key: RistrettoPoint,
            ristretto_point.compress().to_bytes().as_ref(), // r_point: RistrettoPoint,
        ]
        .concat();

        tx
    }

    // signed with the sender’s private key - verified using the sender’s public key.
    fn serialize_for_signing(&self) -> String {
        format!(
            "{}{}{}{}{}",
            self.sender, self.receiver, self.fee, self.amount, self.nonce
        )
    }
}
