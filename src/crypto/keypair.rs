// Transactions need to be signed using the sender’s private key to ensure they are
// authentic and haven’t been tampered with.

use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer};
use rand::rngs::OsRng;

pub struct KeyPairRublock {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl KeyPairRublock {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);

        KeyPairRublock {
            public_key: keypair.public,
            secret_key: keypair.secret,
        }
    }

    // `message` refers to the serialized transaction data that the sender signs with
    // their private key.
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        // Create a `Keypair` from the existing public and secret keys.
        let keypair = Keypair {
            public: self.public_key,
            secret: self.secret_key,
        };
        keypair.sign(message).to_bytes().to_vec() // Sign the message and return the signature as a byte vector.
    }
}
