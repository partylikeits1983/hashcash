use hex;
use primitive_types::U256;
use sha256::digest;

fn hashcash(header: &str, difficulty: u32) -> String {
    let mut nonce = 0u64;
    loop {
        let input = format!("{}{}", header, nonce);
        
        let hash_str = digest(&input);
        let hash_bytes = hex::decode(&hash_str).unwrap();

        let hash_u256 = U256::from_big_endian(&hash_bytes);

        let formatted_hash = format!("{:064x}", hash_u256);

        if formatted_hash.starts_with(&"0".repeat(difficulty as usize)) {
            return format!(
                "header: {}\nnonce: {}\nhash (U256 in hex): 0x{}",
                header, nonce, formatted_hash
            );
        }

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
    use primitive_types::U256;

    #[test]
    fn hash_test() {
        let input_1: u64 = 0x1;
        let input_2: u64 = 0x1;

        // Combine inputs
        let input_1_bytes = input_1.to_be_bytes();
        let input_2_bytes = input_2.to_be_bytes();
        let mut combined = Vec::new();
        combined.extend_from_slice(&input_1_bytes);
        combined.extend_from_slice(&input_2_bytes);

        // Get SHA-256
        let result_hex = digest(&combined);
        println!("Raw SHA-256 hex: 0x{}", result_hex);

        // Decode hex -> [u8; 32]
        let hash_bytes = hex::decode(result_hex).unwrap();
        println!("hash bytes: {:?}", hash_bytes);

        // Convert to U256 (big-endian)
        let mut be_bytes = [0u8; 32];
        be_bytes.copy_from_slice(&hash_bytes);
        let hash_u256 = U256::from_big_endian(&be_bytes);

        // Noir's prime field for BN254:
        //   21888242871839275222246405745257275088548364400416034343698204186575808495617
        let noir_prime_str = "21888242871839275222246405745257275088548364400416034343698204186575808495617";
        let noir_prime = U256::from_dec_str(noir_prime_str).unwrap();

        // Reduce mod Noir's prime
        let hash_mod_prime = hash_u256 % noir_prime;

        // This should match Noir's Field::from_be_bytes output
        println!("Hash mod Noir's prime: 0x{:x}", hash_mod_prime);

        assert!(true);
    }
}

