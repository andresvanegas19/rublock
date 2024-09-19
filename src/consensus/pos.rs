use crate::consensus::validator::Validator;
use lazy_static::lazy_static;

use log::error;
use vrf::openssl::CipherSuite;
use vrf::{ECVRF, VRF};

// TODO: create the default validator
// Placeholder for the default validator.
// lazy_static! {
//     static ref DEFAULT_VALIDATOR: Validator = Validator {
//         address: "default_address".to_string(),
//         stake: 0,
//         reputation: 0,
//         public_key: vec![0; 33], // Assuming a 33-byte public key for example
//     };
// }

// select a validator in a way that is random yet verifiable and influenced by the validator’s stake
// TODO: make a more robust randomest - Commit-Reveal Schemes - Hash-Based Randomness - Decentralized Randomness
// Seed: Ensures that all validators are using the same base data for randomness generation.
// TODO: Previous Block Hash - Epoch Number or Slot ID
pub fn select_random_validator(
    validators: &[Validator],
    secret_key: &[u8],
    seed: &[u8],
) -> Option<&Validator> {
    if validators.is_empty() {
        // Handle the creation of the genesis block explicitly
        return None;
    }

    // Calculate the total stake of all selected validators in a single iteration.
    let total_stake: u64 = validators.iter().fold(0, |acc, v| acc + v.stake);
    // Handle the case where there is no stake by returning the default validator
    if total_stake == 0 {
        return Some(&*DEFAULT_VALIDATOR);
    }

    let mut outputs = Vec::new();
    // TODO: Implement incentives for users to become validators by staking tokens.
    // TODO: Penalize validators who act maliciously or fail to validate correctly.
    for validator in validators {
        // include stake in the selection, adjust the VRF output based on the validator’s stake.
        // SECP256K1_SHA256_TAI Specifies the elliptic curve and hash function to be used.
        // TODO: Ensure that all validators are using the same cipher suite to avoid verification issues
        let vrf = match ECVRF::from_secret_key(secret_key, CipherSuite::SECP256K1_SHA256_TAI) {
            Ok(vrf) => vrf,
            Err(e) => {
                error!("Error proving VRF with seed {:?}", seed);
                debug!("Error proving VRF with seed {:?}: {:?}", seed, e);
                return None; // Gracefully handle the error by returning None
            }
        };
        let (hash, proof) = match vrf.prove(seed) {
            Ok(result) => result,
            Err(e) => {
                debug!("Error proving VRF: {:?}", e);
                continue;
            }
        };
        let threshold = calculate_threshold(validator.stake, total_stake);

        // Convert the hash output to a numeric value for comparison.
        if hash_to_number(&hash) < threshold {
            outputs.push((validator, hash));
        }
    }

    outputs.sort_unstable_by(|a, b| a.1.cmp(&b.1)); // Sort by hash value
    outputs
        .first()
        .map(|(validator, _)| *validator)
        .or(Some(&*DEFAULT_VALIDATOR));
}

fn calculate_threshold(stake: u64, total_stake: u64) -> u64 {
    if total_stake == 0 {
        return u64::MAX;
    }
    (stake as u128 * u64::MAX as u128 / total_stake as u128) as u64
}

// Convert the first 8 bytes of the hash to a u64 number.
// This is done to reduce the hash to a manageable size for comparison purposes.
// Using the first 8 bytes ensures that we get a consistent and reproducible numeric value.
fn hash_to_number(hash: &[u8]) -> u64 {
    // Convert the first 8 bytes of the hash to a u64 number
    let mut array = [0u8; 8];
    let bytes = &hash[..8];
    match bytes.try_into() {
        Ok(b) => array = b,
        Err(_) => {
            error!("slice with incorrect length");
            return 0; // or handle the error as needed
        }
    }
    array = bytes.try_into().expect("slice with incorrect length");

    u64::from_be_bytes(array)
}
