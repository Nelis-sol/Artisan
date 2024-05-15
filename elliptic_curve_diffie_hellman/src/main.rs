use x25519_dalek::{EphemeralSecret, PublicKey};
use rand_core::OsRng;


fn main() {

    // create a secret from a random number (computed with OsRng)
    // an ephemeral secret is only generated once and wiped from memory
    let alice_secret = EphemeralSecret::random_from_rng(OsRng);

    // derive the public key based off the secret key
    let alice_public = PublicKey::from(&alice_secret);

    // create a secret from a random number (computed with OsRng)
    // an ephemeral secret is only generated once and wiped from memory
    let bob_secret = EphemeralSecret::random_from_rng(OsRng);

    // derive the public key based off the secret key
    let bob_public = PublicKey::from(&bob_secret);

    // compute the secret shared between one's private key and another one's public key
    let alice_shared_secret = alice_secret.diffie_hellman(&bob_public);

    // compute the secret shared between one's private key and another one's public key
    let bob_shared_secret = bob_secret.diffie_hellman(&alice_public);
    
    // the secret is the same for alice and bob, hence it's a shared secret
    assert_eq!(alice_shared_secret.as_bytes(), bob_shared_secret.as_bytes());


    // Optional: encode the public key into a readable string for inspection
    let hex_string = hex::encode(alice_public);
    
    // Optional: print the hex encoded public key
    println!("Alice's public key: {:?}", hex_string);


}
