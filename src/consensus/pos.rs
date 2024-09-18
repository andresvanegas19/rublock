use crate::consensus::validator::Validator;
use rand::rngs::OsRng;
use vrf::openssl::CipherSuite;
use vrf::VRF;

// Placeholder for the default validator.
// TODO: how to create this default validator
const DEFAULT_VALIDATOR: Validator = Validator {
    address: String::new(),
    stake: 0,
    reputation: 0,
    secret_key: vec![],
    public_key: vec![],
};

// select a validator in a way that is random yet verifiable and influenced by the validator’s stake
// TODO: make a more robust randomest - Commit-Reveal Schemes - Hash-Based Randomness - Decentralized Randomness
// Seed: Ensures that all validators are using the same base data for randomness generation.
// TODO: Previous Block Hash - Epoch Number or Slot ID
pub fn select_validator(validators: &[Validator], seed: &[u8]) -> Option<&Validator> {
    if validatos.is_empty() {
        // Fallback to a default validator or handle genesis block creation
        return Some(&DEFAULT_VALIDATOR);
    }

    // Get the total of total_ stake in selected validators, because
    // we dont have certainty that when generate the block how many validators we will have
    let total_stake: u64 = validators.iter().map(|v| v.stake).sum();
    // TODO: there is no stake, help with this case
    if total_stake == 0 {
        return None;
    }

    let mut outputs = Vec::new();
    // TODO: Implement incentives for users to become validators by staking tokens.
    // TODO: Penalize validators who act maliciously or fail to validate correctly.
    for validator in validators {
        // include stake in the selection, adjust the VRF output based on the validator’s stake.
        // SECP256K1_SHA256_TAI Specifies the elliptic curve and hash function to be used.
        // TODO: Ensure that all validators are using the same cipher suite to avoid verification issues
        let vrf = ECVRF::from_secret_key(&validator.secret_key, CipherSuite::SECP256K1_SHA256_TAI);
        let (hash, proof) = match vrf.prove(seed) {
            Ok(result) => result,
            Err(_) => continue,
        };
        let threshold = calculate_threshold(validator.stake, total_stake);

        // Convert the hash output to a numeric value for comparison.
        if hash_to_number(&hash) < threshold {
            outputs.push((validator, hash));
        }
    }

    outputs.sort_by(|a, b| a.1.cmp(&b.1)); // Sort by hash value
    outputs.first().map(|(validator, _)| *validator)
}

fn calculate_threshold(stake: u64, total_stake: u64) -> u64 {
    let stake_ratio = stake as f64 / total_stake as f64;
    let threshold = (u64::MAX as f64 * stake_ratio) as u64;

    threshold
}

// Placeholder for the hash_to_number function.
fn hash_to_number(hash: &[u8]) -> u64 {
    // Convert the first 8 bytes of the hash to a u64 number
    let mut array = [0u8; 8];
    let bytes = &hash[..8];
    array.copy_from_slice(bytes);
    u64::from_be_bytes(array)
}
