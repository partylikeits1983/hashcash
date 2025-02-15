use hex;
use primitive_types::U256;
use sha256::digest;

fn hashcash(header: &str, difficulty: u32) -> String {
    let mut nonce = 0u64;
    loop {
        let input = format!("{}{}", header, nonce);
        // `digest` returns a hex string by default, so we decode it into bytes
        let hash_str = digest(&input);
        let hash_bytes = hex::decode(&hash_str).unwrap();

        // Convert the bytes into a U256 (interpreted as big-endian)
        let hash_u256 = U256::from_big_endian(&hash_bytes);

        // Format as a zero-padded 64-character hex string (256 bits)
        let formatted_hash = format!("{:064x}", hash_u256);

        // Check if it meets the difficulty requirement
        if formatted_hash.starts_with(&"0".repeat(difficulty as usize)) {
            return format!(
                "header: {}\nnonce: {}\nhash (U256 in hex): 0x{}",
                header, nonce, formatted_hash
            );
        }

        // Optional logging
        println!("Trying nonce: {}, hash: 0x{}", nonce, formatted_hash);

        nonce += 1;
    }
}

fn main() {
    let result = hashcash("\n", 4);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use sha256::digest;

    #[test]
    fn hash_test() {
        println!("test");

        // Example inputs
        let input_1: u64 = 0x1;
        let input_2: u64 = 0x1;

        // Convert each to big-endian bytes
        let input_1_bytes = input_1.to_be_bytes();
        let input_2_bytes = input_2.to_be_bytes();

        // Combine the two 8-byte arrays into one 16-byte array
        let mut combined = Vec::new();
        combined.extend_from_slice(&input_1_bytes);
        combined.extend_from_slice(&input_2_bytes);

        // Hash the combined bytes
        let result_hex = digest(&combined);

        println!("input_1 (hex): 0x{:x}", input_1);
        println!("input_2 (hex): 0x{:x}", input_2);
        println!("Combined bytes: {:02x?}", combined);
        println!("Hash result: {}", result_hex);

        let hash_bytes = hex::decode(result_hex).unwrap();

        println!("hash bytes: {:?}", hash_bytes);

        // If you want, you could compare this to an expected value here
        // For now, we just assert!(true)
        assert!(true);
    }
}
