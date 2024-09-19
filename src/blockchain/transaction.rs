// struct that represents a transfer of value or data between participants in the network
use crate::crypto::keypair::KeyPairRublock;

use super::ledger::Ledger;

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
        keypair: &KeyPairRublock,
        fee: u64,
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
        tx.signature = keypair.sign(tx_data.as_bytes());

        tx
    }

    // signed with the sender’s private key - verified using the sender’s public key.
    fn serialize_for_signing(&self) -> String {
        format!(
            "{}{}{}{}",
            self.sender, self.receiver, self.amount, self.nonce
        )
    }

    pub fn verify(&self, ledger: &Ledger) -> bool {
        // Retrieve the senders pub key
        let public_key = ledger.get_public_key_from_address(&self.sender)?;
        // Serialize transaction data
        let tx_data = self.serialize_for_signing();

        public_key.verify(&tx_data, &self.signature).is_ok()
    }
}
