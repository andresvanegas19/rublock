// simple Proof-of-Stake (PoS) consensus mechanism.
// Validators are chosen to create new blocks based on their stake in the network.
// The more stake a validator has, the higher the probability of being selected.

pub struct Validator {
    pub adresses: String,
    // TODO: Implement a system for new validators to join the network.
    pub stake: u64,
    pub reputation: i32,
}

pub struct MaliciousEvidenceValidator {
    // TODO: complete this Broadcast Evidence - Collective Agreement - Consensus on Punishment:
}
