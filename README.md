Inspired by [Blockchain](https://github.com/ZuoFuhong/blockchain_rust)

Own thoughts on TODO:
- Validate transactions and adding a new block
- Been able to generate the genesis block
- Create a system of challenge
- work on network and try to fit this project into a docker
- implement account model and asmart contacts
- implement adding the blacklist and penalty in onchain 

TODO:
    1.	Transaction Handling:
        •	Create, sign, and verify transactions.
        •	Ensure transactions can be added to a pool and included in blocks.
    2.	Block Creation:
        •	Implement block struct with basic fields.
        •	Ability to add blocks to the ledger.
    3.	Ledger Management:
        •	Maintain a chain of blocks.
        •	Ensure blocks are linked via previous hashes.
    4.	Validator Selection (Simplified):
        •	For the MVP, you might simplify validator selection.
        •	Use a simple round-robin or random selection without complex VRF.
    5.	Consensus Mechanism:
        •	Implement a basic mechanism to add new blocks to the chain.
        •	For the MVP, consensus can be simplified.

Steps to Get the MVP Running:
    1.	Simplify Components Where Possible:
        •	Validator Selection:
        •	Instead of implementing VRF, select validators randomly or use a fixed validator for testing.
    2.	Ensure Key Functionalities Work End-to-End:
        •	Create a Transaction:
        •	Users can create and sign a transaction.
        •	Add Transaction to Pool:
        •	Transactions are added to a pool awaiting inclusion in a block.
        •	Create a Block:
        •	Collect transactions from the pool and create a new block.
        •	Add Block to Ledger:
        •	Add the new block to the blockchain ledger.
        •	Verify the Chain:
        •	Ensure that the chain remains valid after adding new blocks.
    3.	Testing the MVP:
        •	Unit Tests:
        •	Write tests for creating transactions, signing, and verification.
        •	Integration Test:
        •	Simulate the process of adding transactions and creating blocks.
    4.	Run the Application:
        •	Entry Point:
        •	In main.rs, implement a simple scenario:
        •	Create a ledger.
        •	Generate key pairs for users.
        •	Create and add transactions.
        •	Create and add blocks.
