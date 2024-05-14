use sha3::{Digest, Sha3_256};
use hex_literal::hex;

fn main() {

    // create a SHA3-256 object
    let mut hasher = Sha3_256::new();

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
        hex!("3338be694f50c5f338814986cdf0686453a888b84f424d792af4b9202398f392")[..]
    );


    // Optional: encode the hash into a readable string for inspection
    let hex_string = hex::encode(result);

    // Optional: print the hash
    println!("Hash (SHA3-256): {}", hex_string);


}
