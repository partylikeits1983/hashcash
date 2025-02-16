use hex;
use primitive_types::U256;
use sha256::digest;

pub fn hashcash(header_hex: &str, difficulty: u32) -> String {
    let header_u256 = U256::from_str_radix(header_hex, 16).unwrap();

    // Noir's BN254 prime
    let noir_prime_str =
        "21888242871839275222246405745257275088548364400416034343698204186575808495617";
    let noir_prime = U256::from_dec_str(noir_prime_str).unwrap();

    let mut nonce: u128 = 0u128;
    loop {
        let header_be = header_u256.to_big_endian();

        let nonce_be = U256::from_dec_str(&format!("{}", nonce))
            .unwrap()
            .to_big_endian();

        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(&header_be);
        combined.extend_from_slice(&nonce_be);

        let hash_hex = digest(&combined);
        let hash_bytes = hex::decode(hash_hex).unwrap();
        let hash_u256 = U256::from_big_endian(&hash_bytes);

        // -- Reduce mod Noir’s BN254 prime --
        let hash_mod_prime = hash_u256 % noir_prime;

        let formatted_hash = format!("{:064x}", hash_mod_prime);
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
