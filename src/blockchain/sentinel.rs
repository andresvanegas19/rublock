// will be in charge on check transations, block and manage reputation of validators
// also will be in charge of slashing validators

pub struct SentinelBlockchain {
    pub validator: Validator,
    pub blocklist: Vec<Validator>,
    pub slashing_history: Vec<MaliciousEvidenceValidator>,
}

impl SentinelBlockchain {
    pub fn validate_transaction(&self, transaction: &Transaction) -> bool {
        // TODO: Implement signature verification, balance checks, and double-spend prevention
        //     Invalid Blocks: Blocks that don’t conform to protocol rules.
        //      Double-Spending Attempts: Transactions that try to spend the same funds more than once.
        //      Consensus Violations: Actions that disrupt the consensus mechanism, like signing conflicting blocks.
        true
    }

    pub fn validate_block(&self, block: &Block) -> bool {
        // TODO: Verify previous_hash, hash validity, and transaction correctness. Liveness Faults: Failing to participate when required.
        //     Independent Verification: Each node verifies blocks and transactions independently.
        // •	Cross-Node Communication: Nodes share information about suspicious activities.
        // •	Consensus Mechanisms: Protocols like Byzantine Fault Tolerance (BFT) help in reaching agreement even with some malicious actors.
        true
    }

    pub fn slash_validator(&mut self, validator_address: &String) {
        let Some(guilty_validator) = self
            .validators
            .iter()
            // the find method is borrowing the address field for comparison, rather than taking ownership of it
            .find(|v| &v.address == validator_address);

        if (guilty_validator.is_some()) {
            // Reduce the validator's stake as a penalty
            validator.stake /= 4;
            debug!(
                "Validator {} has been slashed. New stake: {}",
                validator_address, validator.stake
            );
        }
    }

    pub fn update_reputation(
        validators: Vec<Validator>,
        validator_address: &String,
        reputation: i32,
        negative: bool,
    ) {
        let Some(validator) = validators
            .iter_mut()
            .find(|v| &v.address == validator_address);

        if (validator.is_some()) {
            if negative {
                validator.reputation -= reputation;
            } else {
                validator.reputation += reputation;
            }
            debug!(
                "Validator {} reputation updated. New reputation: {}",
                validator_address, validator.reputation
            );
        }
    }

    pub fn report_malicious_validator(&self, evidence: MaliciousEvidenceValidator) {}
}
