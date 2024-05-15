use aes_gcm::{
    aead::{AeadCore, AeadInPlace, KeyInit, OsRng, heapless},
    Aes256Gcm,
};


fn main() {
    
    // create secret key from a random number (computed with OsRng)
    let secretkey = Aes256Gcm::generate_key(&mut OsRng);

    // initiate the algorithm used for encrypting data
    let cipher = Aes256Gcm::new(&secretkey);

    // generate a nonce (or IV) which can be public but needs to be unique/must not be re-used
    // size of nonce is same as the block size used in the algorithm
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);


    // determine a text that needs to be hashed
    let text: String = String::from("plaintext message");

    // turn text string into bytes to use for the key generation operation
    // Optional: for full heapless use heapless::Vec
    let text_bytes: Vec<u8> = text.into_bytes();


    // create buffer for in-place encryption
    // buffer should be as large as the block size (which is 128 bits for AES)
    let mut buffer: heapless::Vec<u8, 128> = heapless::Vec::new(); 

    // store text to be encrypted in the buffer
    buffer.extend_from_slice(&text_bytes);

    // encrypt buffer in-place, replacing the plaintext contents with ciphertext
    cipher.encrypt_in_place(&nonce, b"", &mut buffer).unwrap();

    // assert that buffer no longer contains the plain text
    assert_ne!(&buffer, b"plaintext message");



    // decrypt buffer in-place, replacing its ciphertext context with the original plaintext
    cipher.decrypt_in_place(&nonce, b"", &mut buffer).unwrap();

    // assert that buffer contains the plain text again
    assert_eq!(&buffer, b"plaintext message");

}
