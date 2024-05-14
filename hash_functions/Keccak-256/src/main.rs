use sha3::{Digest, Keccak256};
use hex_literal::hex;

fn main() {

    // create a SHA3-256 object
    let mut hasher = Keccak256::new();

    // determine a text that needs to be hashed
    let text: String = String::from("hello");

    // turn text string into bytes to use for the hashing operation
    let text_bytes: Vec<u8> = text.into_bytes();

    // process the input data
    hasher.update(text_bytes);

    // retrieve the hash result in a byte array
    let result = hasher.finalize();

    // verify that the digest function return the expected hash
    assert_eq!(
        result[..], 
        hex!("1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8")[..]
    );


    // Optional: encode the hash into a readable string for inspection
    let hex_string = hex::encode(result);

    // Optional: print the hash
    println!("Hash (SHA3-Keccak256): {}", hex_string);


}
