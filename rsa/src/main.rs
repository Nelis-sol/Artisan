use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

fn main() {

    // determine a text that needs to be encrypted
    let text: String = String::from("plaintext message");

    // turn text string into bytes to use for the key generation operation
    let text_bytes: Vec<u8> = text.into_bytes();

    // create a random number generator
    let mut rng = rand::thread_rng();

    // determine the bits size, typically 1024, 2048 or 4096 bits
    // number is the number of bits in the modulus (encryptedMsg = (msg)e mod n)
    let bits = 3072;

    // generate private key from random number generator and the specified bits
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

    // derive public key from the private key
    let pub_key = RsaPublicKey::from(&priv_key);

    // asymetrically encrypt data to a pubkey (in this case our own) with our private key and add padding
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &text_bytes[..]).expect("failed to encrypt");

    // Optional: encode the hash into a readable string for inspection
    let hex_string = hex::encode(&enc_data);

    // Optional: print the hash
    println!("{:?}", hex_string);

    // Decrypt using a private key that is associated with the public key used for encryption
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");

    // assert that the decrypted message equals the original text
    assert_eq!(&text_bytes[..], &dec_data[..]);

}
