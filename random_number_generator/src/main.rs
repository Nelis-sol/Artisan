use rand::prelude::*;


fn main() {

    // create a random number generator object
    // threadRng uses the same PRNG as StdRng for security and performance and is automatically seeded from OsRng
    let mut rng = rand::thread_rng();


    // retrieve random number from generator
    // change the data type of random_number to retrieve a different number type
    let random_number: u128 = rng.gen();

    // Optional: print the number
    println!("{}", random_number)

}
