    use argon2::{Argon2, Algorithm, Version, Params};


fn main() {


    // determine a text (or password in this case) that needs to be hashed
    let password: String = String::from("password");

    // turn password string into bytes to use for the key generation operation
    let password_bytes: Vec<u8> = password.into_bytes();

    // determine a string that is used as salt (should be securely random generated)
    let salt: String = String::from("some salt");

    // turn salt string into bytes to use for the hashing operation
    let salt_bytes: Vec<u8> = salt.into_bytes();
  

    // set parameters for the key generation function
    let settings = Params::new(
        2u32.pow(15),   // m - amount of memory (in kilobytes) required to use
        16, // t - iterations - number of iterations to perform
        2, // p - parallelism - degree of parallelism (i.e. number of threads)
        Some(32) // T - desired number of returned bytes
    ).unwrap();

    // create new Argon2 instance using the parameters we require
    let instance = Argon2::new(
        Algorithm::Argon2id,    // which Argon2 algorithm to use
        Version::V0x13, // which version of the algorithm to use
        settings // parameters for this instance
    );


    // buffer serves as an output for the scrypt key generation function
    let mut buffer: Vec<u8> = vec![0u8; 32];

    // generate the key
    instance.hash_password_into(&password_bytes, &salt_bytes, &mut buffer).unwrap();


    // Optional: encode the hash into a readable string for inspection
    let hex_string = hex::encode(buffer);

    // Optional: print the hash
    println!("{:?}", hex_string);

  
}
