use crate::consensus::validator::Validator;
use rand::Ring;

pub fn select_validator(validators: &[Validator], seed: &[u8]) -> Option<&Validator> {
    if validatos.is_empty() {
        // TODO: how to create this default validator
        // Fallback to a default validator or handle genesis block creation
        return Some(&DEFAULT_VALIDATOR);
    }

    // Get the total of total_ stake in selected validators, because
    // we dont have certainty that when generate the block how many validators we will have
    let total_stake: u64 = validators.iter().map(|v| v.stake).sum();

    // TODO: Since there are no validators initially, the genesis block must
    // be created without validator selection.
    if total_stake == 0 {
        return None;
    }

    // TODO: make a more robust randomest - VRFs - Commit-Reveal Schemes - Hash-Based Randomness:
    // Decentralized Randomness
    // TODO: provide test for this functions
    let random_number = get_random_number(total_stake);
    let mut cumulative = 0;

    // Return the winner base on the stake of user could have
    for validator in validators {
        cumulative += validator.stake;
        if random_number < cumulative {
            // TODO: Implement incentives for users to become validators by staking tokens.
            // TODO: Penalize validators who act maliciously or fail to validate correctly.
            return Some(validator);
        }
    }

    None
}

fn get_random_number(top: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..top)
}
