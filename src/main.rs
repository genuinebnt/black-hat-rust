use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, hash};

use sha1::Digest;

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist> <sha1 hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err(format!("Not a valid hash {}", hash_to_crack.len()).into());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let password = line.trim();
        if hash_to_crack == hex::encode(sha1::Sha1::digest(password)) {
            println!("Password found {}", password);
            return Ok(());
        }
    }

    Ok(())
}
