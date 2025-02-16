use hex;
use primitive_types::U256;
use sha256::digest;

fn hashcash(header_hex: &str, difficulty: u32) -> String {
    let header_u256 = U256::from_str_radix(header_hex, 16).unwrap();

    // Noir's BN254 prime
    let noir_prime_str =
        "21888242871839275222246405745257275088548364400416034343698204186575808495617";
    let noir_prime = U256::from_dec_str(noir_prime_str).unwrap();

    let mut nonce: u128 = 0u128;
    loop {
        let header_be = header_u256.to_big_endian();

        let nonce_256 = U256::from_dec_str(&format!("{}", nonce));

        println!("print nonce: {:?} nonce_256: {:?}", nonce, nonce_256);

        let nonce_be = U256::from_dec_str(&format!("{}", nonce))
            .unwrap()
            .to_big_endian();

        println!("header be: {:?}", header_be);
        println!("nonce be: {:?}", nonce_be);

        // -- Concatenate => 16 total bytes --
        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(&header_be);
        combined.extend_from_slice(&nonce_be);

        // -- SHA-256 of these 16 bytes --
        let hash_hex = digest(&combined);
        let hash_bytes = hex::decode(hash_hex).unwrap();
        let hash_u256 = U256::from_big_endian(&hash_bytes);

        // -- Reduce mod Noir’s BN254 prime --
        let hash_mod_prime = hash_u256 % noir_prime;

        println!("hash_mod_prime: {:?}", hash_mod_prime);

        // -- Format the reduced hash (hex, zero-padded to 64 nibbles) --
        let formatted_hash = format!("{:064x}", hash_mod_prime);

        // -- Check difficulty by counting leading zero nibbles --
        if formatted_hash.starts_with(&"0".repeat(difficulty as usize)) {
            return format!(
                "header: 0x{}\nnonce: {}\nnoir hash (mod prime, hex): 0x{}",
                header_hex, nonce, formatted_hash
            );
        }

        println!("Trying nonce: {}, noir hash: 0x{}", nonce, formatted_hash);
        nonce = nonce.wrapping_add(1);
    }
}

fn main() {
    let result = hashcash("0", 4);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use hex;
    use primitive_types::U256;
    use sha256::digest;

    #[test]
    fn hash_test_u256_inputs() {
        // Two U256 inputs (both are small: 0x123 fits in 64 bits)
        let input_1 = U256::from_str("0").unwrap();
        let input_2 = U256::from_str("0").unwrap();

        // -- STEP 1: Convert each input to 8 big-endian bytes (just like Noir) --
        let input_1_be = input_1.to_big_endian();
        let input_2_be = input_2.to_big_endian();

        println!("input 1: {:?}", input_1_be);
        println!("input 2: {:?}", input_2_be);

        // -- STEP 2: Concatenate => 16 total bytes
        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(&input_1_be);
        combined.extend_from_slice(&input_2_be);

        // -- STEP 3: SHA-256
        let result_hex = digest(&combined);
        println!("Raw SHA-256 hex: 0x{}", result_hex);

        let hash_bytes = hex::decode(&result_hex).unwrap();
        println!("Hash bytes: {:?}", hash_bytes);

        let hash_u256 = U256::from_big_endian(&hash_bytes);

        // -- STEP 4: Reduce mod BN254 prime
        let noir_prime_str =
            "21888242871839275222246405745257275088548364400416034343698204186575808495617";
        let noir_prime = U256::from_dec_str(noir_prime_str).unwrap();
        let hash_mod_prime = hash_u256 % noir_prime;

        println!("Hash mod Noir's prime: 0x{:x}", hash_mod_prime);

        assert!(true);
    }
}
