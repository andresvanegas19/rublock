// Transactions need to be signed using the sender’s private key to ensure they are
// authentic and haven’t been tampered with.

use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::montgomery::MontgomeryPoint;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use log::debug;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha512};

// TODO: Understand more in depth this file
pub struct KeyPairRublock {
    pub public_key: Scalar,
    // pub secret_key: SecretKey,
}

impl KeyPairRublock {
    pub fn generate_keypair() -> (Scalar, MontgomeryPoint) {
        // Generate private key (random scalar)
        let mut rng = OsRng;
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        let private_key = Scalar::from_bytes_mod_order(bytes);

        // Derive public key (Montgomery point)
        let public_key = &private_key * &curve25519_dalek::constants::X25519_BASEPOINT;

        (private_key, public_key)
    }

    pub fn sign(message: &[u8], secret_key: Scalar) -> (RistrettoPoint, Scalar) {
        // Hash the message
        let mut hasher = Sha512::new();
        hasher.update(message);

        // Generate a random nonc
        let mut rng = OsRng;
        let mut random_bytes = [0u8; 32];
        rng.fill_bytes(&mut random_bytes);
        let nonce = Scalar::from_bytes_mod_order(random_bytes);

        // Calulate R = nonce * G (basepoint)
        let r_point = nonce * RISTRETTO_BASEPOINT_POINT;

        // Hash R and the message to get the challenge
        let mut challenge_hasher = Sha512::new();
        challenge_hasher.update(r_point.compress().as_bytes());
        challenge_hasher.update(message);
        let challenge_output = challenge_hasher.finalize();

        // Convert the challenge hash into a scalar
        let challenge = Scalar::from_bytes_mod_order(challenge_output[..32].try_into().unwrap());

        // TODO: Why is rest?
        // Calculate signature s = nonce - challenge * secret_key
        let sign = nonce - (challenge * secret_key);

        debug!("Signature created");

        (r_point, sign)
    }

    pub fn verify_signature(
        message: &[u8],
        public_key: RistrettoPoint,
        r_point: RistrettoPoint,
        sign: Scalar,
    ) -> bool {
        // Hash the message
        let mut hasher = Sha512::new();
        hasher.update(message);
        let hash_output = hasher.finalize();

        // Convert the hash into a scalar
        let message_scalar: Scalar =
            Scalar::from_bytes_mod_order(hash_output[..32].try_into().unwrap());

        // Recompute R' = sing * G + message_hash * public_key
        let r_prime = sign * RISTRETTO_BASEPOINT_POINT + (message_scalar * public_key);

        let result = r_prime == r_point;
        debug!("Result of the signature verification: {}", result);

        result
    }

    // fn key_exchange(my_private: &Scalar, their_public: &MontgomeryPoint) -> MontgomeryPoint {
    //     // Compute shared secret using Diffie-Hellman
    //     my_private * their_public
    // }

    // `message` refers to the serialized transaction data that the sender signs with
    // their private key.
    // pub fn sign(&self, message: &[u8]) -> Vec<u8> {
    //     // Create a `Keypair` from the existing public and secret keys.
    //     let keypair = Keypair {
    //         public: self.public_key,
    //         secret: self.secret_key,
    //     };
    //     keypair.sign(message).to_bytes().to_vec() // Sign the message and return the signature as a byte vector.
    // }
}
