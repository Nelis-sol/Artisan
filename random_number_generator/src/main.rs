use rand::prelude::*;
use rand_core::OsRng;


fn main() {

    // 01. Random number for non-crytographic purposes (better performance)

    
    // create a random number generator object
    // threadRng uses the same PRNG as StdRng for security and performance and is automatically seeded from OsRng
    let mut rng = rand::thread_rng();


    // retrieve random number from generator
    // change the data type of random_number to retrieve a different number type
    let random_number: u64 = rng.gen();

    // Optional: print the number
    println!("random number: {}", random_number);


    
    // 02. Random number for cryptographic purposes (true entropy)


    // create a random number generator object
    // OsRng directly accesses the operating system's random number generator
    let mut rng_os = OsRng;

    let random_number_os: u64 = rng_os.next_u64();

    // Optional: print the number
    println!("random number os: {}", random_number_os);

}
