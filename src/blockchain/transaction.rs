// struct that represents a transfer of value or data between participants in the network
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
        sender: String,
        receiver: String,
        amount: u64,
        nonce: u64,
        keypair: &KeyPairRublock,
    ) -> Self {
        let mut tx = Transaction {
            sender,
            receiver,
            amount,
            nonce,
            signature: Vec::new(),
        };

        let tx_data = tx.serialize_for_signing();
        tx.signature = keypair.sign(&tx_data);

        tx
    }

    fn seralize_for_data(&self) -> String {
        format!(
            "{}{}{}{}",
            self.sender, self.receiver, self.amount, self.nonce
        )
    }

    pub fn verify(&self) -> bool {
        // Retrieve the senders pub key
        let bytes_public_key = self.get_public_key_from_adress(&self.sender)?;

        let tx_data = self.serialize_for_signing();
        let public_key = PublicKey::from_bytes(&self.sender).unwrap();
        let signature = Signature::from_bytes(&self.signature).unwrap();

        public_key.verify(&tx_data, &signature).is_ok()
    }

    fn get_public_key_from_adress(&self, address: &str) -> Option<Vec<u8>> {}
}
