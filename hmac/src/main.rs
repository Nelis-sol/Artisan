use sha2::Sha256;
use hmac::{Hmac, Mac};
use hex_literal::hex;

fn main() {

    // create alias for HMAC-SHA256
    type HmacSha256 = Hmac<Sha256>;

    // determine a secretkey
    let secretkey: String = String::from("12345");

    // turn text string into bytes to use for the hashing operation
    let secretkey_bytes: Vec<u8> = secretkey.into_bytes();

    // determine a message
    let message: String = String::from("sample message");

    // turn text string into bytes to use for the hashing operation
    let message_bytes: Vec<u8> = message.into_bytes();

    // ingest the secretkey
    let mut mac = HmacSha256::new_from_slice(&secretkey_bytes).unwrap();

    // process the message
    mac.update(&message_bytes);

    // retrieve the result into a byte array
    let result = mac.finalize();

    // turn result into usable format (byte array in this case)
    let code_bytes = result.into_bytes();

    // determine expect result for checking
    let expected = hex!("ee40ca7bc90df844d2f5b5667b27361a2350fad99352d8a6ce061c69e41e5d32");
    
    // assert that the result matches the expect result
    assert_eq!(
        code_bytes[..], 
        expected[..]
    );


}
