// simple Proof-of-Stake (PoS) consensus mechanism.
// Validators are chosen to create new blocks based on their stake in the network.
// The more stake a validator has, the higher the probability of being selected.

// Node: Any computer that participates in the network by storing and sharing data.
// Validator: A specialized node that actively participates in the consensus process by validating
// transactions and creating new blocks

use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use rand::rngs::OsRng;

pub struct Validator {
    pub adresses: String,
    // TODO: Implement a system for new validators to join the network.
    pub stake: u64,
    pub reputation: i32,
    // key personalizes the VRF
    pub public_key: Vec<u8>,
}

impl Validator {
    pub fn new(address: String, stake: u64, reputation: i32) -> Self {
        // Validators generate a key pair (secret and public key) when they set up their validator node.
        // The secret key is used to sign messages and the public key is used to verify the signature.
        let (secret_key, public_key) = Self::generate_key_pair();

        Validator {
            address,
            stake,
            reputation,
        }
    }

    fn generate_key_pair() -> (Vec<u8>, Vec<u8>) {
        //  generating random numbers
        let mut csprng = OsRng {};

        // used for generating and handling Ed25519 key pairs.
        let keypar: Keypair = Keypair::generate(&mut csprng);
        let secret_key = keypar.secret.to_bytes().to_vec();
        let public_key = keypar.public.to_bytes().to_vec();

        (secret_key, public_key)
    }
}