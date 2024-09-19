// simple Proof-of-Stake (PoS) consensus mechanism.
// Validators are chosen to create new blocks based on their stake in the network.
// The more stake a validator has, the higher the probability of being selected.

// Node: Any computer that participates in the network by storing and sharing data.
// Validator: A specialized node that actively participates in the consensus process by validating
// transactions and creating new blocks

use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use rand::rngs::OsRng;

pub struct Validator {
    // Identifier derived from a public key
    pub address: String,
    // TODO: Implement a system for new validators to join the network.
    pub stake: u64,
    pub reputation: i32,
    // Key personalizes the VRF
    pub public_key: PublicKey,
}

impl Validator {
    pub fn new(address: String, stake: u64, reputation: i32) -> (Self, SecretKey) {
        // Validators generate a key pair (secret and public key) when they set up their validator node.
        // The secret key is used to sign messages and the public key is used to verify the signature.
        let (secret_key, public_key) = Self::generate_key_pair();

        let validator = Validator {
            address,
            stake,
            reputation,
            public_key,
        };

        // Return the validator instance and the secret key separately
        (validator, secret_key)
    }

    fn generate_key_pair() -> (SecretKey, PublicKey) {
        // Generating random numbers
        let mut csprng = OsRng {};

        // Used for generating and handling Ed25519 key pairs.
        let keypair: Keypair = Keypair::generate(&mut csprng);

        // TODO: Implement proper error handling and consider using crates like zeroize to clear secret keys from memory when they’re no longer needed.
        (keypair.secret, keypair.public)
    }
}
