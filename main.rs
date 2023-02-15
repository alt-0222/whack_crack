/// ## SHA-1 is a hash function used by a lot of old websites to store the passwords of the users.
/// In theory, a hashed password can’t be recovered from its hash.
/// ## A hash cracker is a program that will try many hashes in order to find the original password.
///

use sha1::Digest;
use std::{
    error::Error,
    env,
    fs::File,
    io::{BufRead, BufReader}
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    // env::args() calls the args function from this module and returns an iterator
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }
    
    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 has is not valid".into());
    }
    
    // create a dict to reduce testing combinations of hashes
    let wl_file = File::open(&args[1])?;
    let reader = BufReader::new(&wl_file);
    
    for line in reader.lines() {
        let line = line?.trim().to_owned();
        let common_password = line.trim();
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }
    println!("password not found in wordlist :(");
    Ok(())
}
