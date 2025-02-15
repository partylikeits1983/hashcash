use hex;
use sha1::{Digest, Sha1};

fn hashcash(header: &str, difficulty: u32) -> String {
    let mut nonce = 0;
    loop {
        let input = format!("{}{}", header, nonce);
        let mut hasher = Sha1::new();
        hasher.update(input.as_bytes());

        // Finalize the hasher instead of calling `digest()`
        let result = hasher.finalize().to_vec();

        // Check if the hash has at least 'difficulty' leading zeros
        let hex_hash = hex::encode(result);
        if hex_hash.starts_with(&"0".repeat(difficulty as usize)) {
            return format!("header: {} nonce: {} hash: {}", header, nonce, hex_hash);
        }

        nonce += 1;
        println!("nonce: {:?} hex_hash: {:?}", nonce, hex_hash)
    }
}

fn main() {
    // Example usage:
    let result = hashcash("example@header.com\n", 5); 
    println!("{}", result);
}
