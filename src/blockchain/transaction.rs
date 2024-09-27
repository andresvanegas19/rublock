// struct that represents a transfer of value or data between participants in the network

use crate::crypto::keypair::KeyPairRublock;
use curve25519_dalek::{EdwardsPoint, MontgomeryPoint, Scalar};

pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub nonce: u64,
    pub signature: Vec<u8>,
    pub fee: f32,
}

impl Transaction {
    pub fn new(
        sender: String,
        receiver: String,
        amount: u64,
        nonce: u64,
        public_key: MontgomeryPoint,
        secret_key: Scalar,
    ) -> Self {
        let fee: f32 = 0.001; // Example initialization

        let mut tx = Transaction {
            sender,
            receiver,
            amount,
            nonce,
            signature: Vec::new(),
            fee,
        };

        tx.sign_transaction(&public_key, &secret_key);

        tx
    }

    fn sign_transaction(&mut self, public_key: &MontgomeryPoint, secret_key: &Scalar) {
        let tx_data = self.serialize_for_signing();
        let (ristretto_point, _) = KeyPairRublock::sign(tx_data.as_bytes(), *secret_key);

        // Convert MontgomeryPoint to EdwardsPoint
        let edwards_point: EdwardsPoint = public_key.to_edwards(0).unwrap();

        self.signature = [
            tx_data.as_bytes(),                             // message: &[u8],
            edwards_point.compress().to_bytes().as_ref(),   // public_key: EdwardsPoint,
            ristretto_point.compress().to_bytes().as_ref(), // r_point: RistrettoPoint,
        ]
        .concat();
    }

    // signed with the sender’s private key - verified using the sender’s public key.
    fn serialize_for_signing(&self) -> String {
        format!(
            "{}{}{:?}{}{}",
            self.sender, self.receiver, self.fee, self.amount, self.nonce
        )
    }
}
