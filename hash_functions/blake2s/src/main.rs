use blake2::{Digest, Blake2s256};
use hex_literal::hex;

fn main() {

    // create a Blake2s (256) object
    let mut hasher = Blake2s256::new();

    // determine a text that needs to be hashed
    let text: String = String::from("hello");

    // turn text string into bytes to use for the hashing operation
    let text_bytes: Vec<u8> = text.into_bytes();

    // process the input data
    hasher.update(text_bytes);

    // retrieve the hash result in a byte array
    let result = hasher.finalize();

    assert_eq!(
        result[..], 
        hex!("19213bacc58dee6dbde3ceb9a47cbb330b3d86f8cca8997eb00be456f140ca25")[..]
    );


    // Optional: encode the hash into a readable string for inspection
    let hex_string = hex::encode(result);

    // Optional: print the hash
    println!("Hash (Blake2s256): {}", hex_string);


}
