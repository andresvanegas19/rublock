// use crate::consensus::validator::Validator;

// enum SlashingReason {
//     DoubleSigning,
//     InvalidBlock,
//     Inactivity,
// }

/// # Validation Checks
/// 1. **Signature Verification**: Ensures that the transaction has been signed by the sender using their private key.
/// 2. **Balance Checks**: Verifies that the sender has sufficient balance to cover the transaction amount and any associated fees.
/// 3. **Double-Spend Prevention**: Ensures that the transaction does not attempt to spend the same funds more than once.
// 4. **Protocol Compliance**: Checks that the transaction conforms to the blockchain's protocol rules, such as format and structure.
// /// 5. **Consensus Violations**: Detects actions that disrupt the consensus mechanism, such as signing conflicting blocks.
// fn penalty_double_signing(&self, validator_amount: u64) -> u64 {
//     validator_amount / 2
//
// fn penalty_invalid_block(&self, validator_amount: u64) -> u64 {
//     validator_amount / 4
// }
// fn penalty_inactivity(&self, validator_reputation: &mut i32) {
//     *validator_reputation -= 1;
// }
// }
// TODO: complete this Broadcast Evidence - Collective Agreement - Consensus on Punishment:
pub struct Penalty {
    pub slashed_validator_address: String,
    pub reporting_validator_address: String,
    // pub reason: SlashingReason,
    // TODO: Could the IP or the address of the validator be hidden so it not publicly available and get all the weight?
    pub block_index: u64,
}

// TODO: imeplemet a system where
// Based on Performance: Rewards are given to validators who reported the misbehavior.
// TODO: prevent this scenario Prevent scenarios where validators might collude to get others slashed for personal gain.

/// # TODO
/// - Implement signature verification.
/// - Implement balance checks.
/// - Implement double-spend prevention.
/// - Implement protocol compliance checks.
/// - Implement consensus violation detection.
// TODO: Implement Slashing rules should be well-defined and agreed upon by the network participants.
impl Penalty {
    pub fn new(
        slashed_validator_address: String,
        reporting_validator_address: String,
        // reason: SlashingReason,
        block_index: u64,
    ) -> Self {
        Penalty {
            slashed_validator_address,
            reporting_validator_address,
            // reason,
            block_index,
        }
    }

    // Return how much it will be the penalty in tokens
    // pub fn apply_penalties(
    //     &self,
    //     slashed_validator: &mut Validator,
    //     reporter_validator: &mut Validator,
    // ) {
    //     let mut amount_penalty: u64 = 0;
    //     match self.reason {
    //         SlashingReason::InvalidBlock => {
    //             amount_penalty = self.penalty_invalid_block(slashed_validator.stake)
    //         }
    //         SlashingReason::Inactivity => self.penalty_inactivity(&mut slashed_validator.reputation),
    //         SlashingReason::DoubleSigning => {
    //             amount_penalty = self.penalty_double_signing(slashed_validator.stake)
    //         }
    //     }

    //     slashed_validator.stake -= amount_penalty;
    //     // TODO: see if only the reported earns the reward or all the network or move to a poll to the developers
    //     reporter_validator.stake += amount_penalty;
}
