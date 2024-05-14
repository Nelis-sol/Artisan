use scrypt::{
    scrypt,
    Params
};

fn main() {


    // determine a text (or password in this case) that needs to be hashed
    let password: String = String::from("p@$Sw0rD~7");

    // turn password string into bytes to use for the key generation operation
    let password_bytes: Vec<u8> = password.into_bytes();

    // determine a string that is used as salt (should be securely random generated)
    let salt: String = String::from("aa1f2d3f4d23ac44e9c5a6c3d8f9ee8c");

    // turn salt string into bytes to use for the hashing operation
    let salt_bytes: Vec<u8> = salt.into_bytes();


    // Set parameters for the key generation function
    let settings = Params::new(
        11, // N - the log2 of the iteration count, in this case 2048
        8, // r - block size 
        1, // p - parallelism
        32 // key length
    ).unwrap();
 

    // buffer serves as an output for the scrypt key generation function
    let mut buffer: Vec<u8> = vec![0u8; 32];

    // generate key
    scrypt(&password_bytes, &salt_bytes, &settings, &mut buffer).unwrap();


    // Optional: encode the hash into a readable string for inspection
    let hex_string = hex::encode(buffer);

    // Optional: print the hash
    println!("{:?}", hex_string);


}
