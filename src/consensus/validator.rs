// simple Proof-of-Stake (PoS) consensus mechanism.
// Validators are chosen to create new blocks based on their stake in the network.
// The more stake a validator has, the higher the probability of being selected.

// Node: Any computer that participates in the network by storing and sharing data.
// Validator: A specialized node that actively participates in the consensus process by validating
// transactions and creating new blocks

// use rand::rngs::OsRng;

use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::MontgomeryPoint;
use log::debug;

use crate::crypto::keypair::KeyPairRublock;
// TODO: learn about elliptic curve-based

pub struct Validator {
    // Identifier derived from a public key
    pub address: Vec<u8>,
    // TODO: Implement a system for new validators to join the network.
    pub stake: u64,
    pub reputation: i32,
    // Key personalizes the VRF
    pub public_key: MontgomeryPoint,
}

impl Validator {
    pub fn new(stake: u64, reputation: i32) -> (Scalar, Self) {
        // Validators generate a key pair (secret and public key) when they set up their validator node.
        // The secret key is used to sign messages and the public key is used to verify the signature.
        let (secret_key, public_key) = KeyPairRublock::generate_keypair();

        // Generate the Address of the validator
        let address: Vec<u8> = public_key.to_bytes().to_vec();

        let validator = Validator {
            address,
            stake,
            reputation,
            public_key,
        };

        debug!("Validator created, stake {}", validator.stake);

        // Return the validator instance and the secret key separately
        (secret_key, validator)
    }

    pub fn get_string_address(&self) -> String {
        // Convert the Vec<u8> address to a hexadecimal string
        self.address
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }

    // TODO: make a serailization to validator being a string
}

// Test the keys and the info of the validator
