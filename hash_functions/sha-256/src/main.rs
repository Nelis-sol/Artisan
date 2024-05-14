use sha256::digest;


fn main() {

    // determine a text that needs to be hashed
    let text: String = String::from("hello");

    // use the digest function from the sha256 to take in a String and return a hash (which is return as a String).
    let data: String = digest(text);

    // verify that the digest function return the expected hash
    assert_eq!(
        data, 
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );


    // Optional: print the hash
    println!("Hash (SHA256): {}", data);

}
