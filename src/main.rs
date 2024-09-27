// test porpouse
use curve25519_dalek::Scalar;
use hex;
use log::debug;
use sha2::{Digest, Sha256};

use rublock::{
    blockchain::ledger::Ledger, blockchain::transaction::Transaction,
    consensus::validator::Validator, crypto::keypair::KeyPairRublock,
};

fn main() {
    env_logger::init();

    // init the ledger
    let mut ledger = Ledger::new();

    // generate key paris for the validators
    let (_, bob_validator) = Validator::new(100, 0);
    let (_, lana_validator) = Validator::new(300, 0);

    ledger.add_validator(bob_validator);
    ledger.add_validator(lana_validator);

    // Generate the keys for the users wallet, this SHOULDNT be done here but is just testing porpouse
    let (bob_keypair, src_key) = KeyPairRublock::generate_keypair();
    // wallet naming
    let bob_key = derive_address(&bob_keypair);
    let (lana_keypair, _) = KeyPairRublock::generate_keypair();
    // wallet naming
    let lana_key = derive_address(&lana_keypair);

    debug!("naming: {}", bob_key);

    // TODO: switch the naming, wrong naming
    let transaction = Transaction::new(
        bob_key,     // sender's public key
        lana_key,    // receiver's public key
        100,         // amount
        1,           // nonce
        src_key,     // sender's secret key
        bob_keypair, // sender's public key
    );

    ledger.add_transaction(transaction);
    ledger.create_block();

    // println!("Ledger: {:?}", ledger);
}

// Utility function to derive address from public key
fn derive_address(public_key: &Scalar) -> String {
    let public_key_bytes = public_key.to_bytes();
    let hash = Sha256::digest(&public_key_bytes);
    hex::encode(&hash)
}
