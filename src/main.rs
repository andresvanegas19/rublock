use rublock::blockchain::ledger::Ledger;

fn main() {
    // init the ledger
    let mut ledger = Ledger::new();

    // // generate key paris for the validators
    // let bob_keypair = KeyPair::generate();
    // let lana_keypair = KeyPair::generate();

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
