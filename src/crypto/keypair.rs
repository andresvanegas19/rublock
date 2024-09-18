// Transactions need to be signed using the sender’s private key to ensure they are
// authentic and haven’t been tampered with.

use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature};
use rand::rngs::OsRng;

pub struct KeyPairRublock {
    pub public_key: PublicKey,
    secret_key: SecretKey,
}

impl KeyPairRublock {
    pub fn generate() -> Self {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);

        KeyPairRublock {
            public_key: keypair.public,
            secret_key: keypair.secret,
        }
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.secret_key.sign(message).to_bytes().to_vec()
    }
}
