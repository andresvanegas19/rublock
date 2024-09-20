// test porpouse
use curve25519_dalek::Scalar;
use hex;
use sha2::{Digest, Sha256};

use rublock::{
    blockchain::ledger::{self, Ledger},
    consensus::validator::Validator,
    crypto::keypair::KeyPairRublock,
};

fn main() {
    env_logger::init();

    // init the ledger
    let mut ledger = Ledger::new();

    // generate key paris for the validators
    let (validator_bob_secret_key, bob_validator) = Validator::new(100, 0);
    let (validator_lana_secret_key, lana_validator) = Validator::new(300, 0);

    ledger.add_validator(bob_validator);
    ledger.add_validator(lana_validator);

    // Generate the keys for the users wallet, this SHOULDNT be done here but is just testing porpouse
    let (bob_keypair, _) = KeyPairRublock::generate_keypair();
    let bob_key = derive_address(&bob_keypair);
    let (lana_keypair, _) = KeyPairRublock::generate_keypair();
    let lana_key = derive_address(&lana_keypair);

    // // Reigster public key in ledger
    // ledger
    //     .address_to_public_key
    //     .insert(bob_keypair.clone(), bob_keypair.public_key.clone());
    // ledger
    //     .address_to_public_key
    //     .insert(lana_keypair.clone(), lana_keypair.public_key.clone());

    // let transaction = Transaction::new(
    //     bob_keypair.clone(),
    //     lana_keypair.clone(),
    //     100, // amount
    //     1,   // nonce
    //     &bob_keypair,
    // );

    // ledger
    //     .add_transaction(transaction)
    //     .expected("Failed to add transaction");

    // ledger.create_block();

    // println!("Ledger: {:?}", ledger);
}

// Utility function to derive address from public key
fn derive_address(public_key: &Scalar) -> String {
    let public_key_bytes = public_key.to_bytes();
    let hash = Sha256::digest(&public_key_bytes);
    hex::encode(&hash)
}
