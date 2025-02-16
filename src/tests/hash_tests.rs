use std::str::FromStr;
use hex;
use primitive_types::U256;
use sha256::digest;
use hashcash::hashcash; 

#[test]
fn hash_test_u256_inputs() {
    let input_1 = U256::from_str("0").unwrap();
    let input_2 = U256::from_str("0").unwrap();

    let input_1_be = input_1.to_big_endian();
    let input_2_be = input_2.to_big_endian();

    println!("input 1: {:?}", input_1_be);
    println!("input 2: {:?}", input_2_be);

    let mut combined = Vec::with_capacity(64);
    combined.extend_from_slice(&input_1_be);
    combined.extend_from_slice(&input_2_be);

    let result_hex = digest(&combined);
    println!("Raw SHA-256 hex: 0x{}", result_hex);

    let hash_bytes = hex::decode(&result_hex).unwrap();
    println!("Hash bytes: {:?}", hash_bytes);

    let hash_u256 = U256::from_big_endian(&hash_bytes);

    let noir_prime_str =
        "21888242871839275222246405745257275088548364400416034343698204186575808495617";
    let noir_prime = U256::from_dec_str(noir_prime_str).unwrap();
    let hash_mod_prime = hash_u256 % noir_prime;

    println!("Hash mod Noir's prime: 0x{:x}", hash_mod_prime);

    assert!(true);
}

#[test]
fn test_hashcash_function() {
    let difficulty = 2; // Small difficulty for testing
    let header_hex = "0x0";

    let result = hashcash(header_hex, difficulty, 0, 1, &std::sync::atomic::AtomicBool::new(false));
    
    assert!(result.is_some(), "hashcash() did not return a result");

    let result_str = result.unwrap();
    println!("Hashcash result:\n{}", result_str);

    // Ensure result contains the expected fields
    assert!(result_str.contains("header: 0x0"), "Result does not contain the expected header");
    assert!(result_str.contains("nonce: "), "Result does not contain the expected nonce");
    assert!(result_str.contains("noir hash (mod prime, hex): 0x"), "Result does not contain the expected hash");
}
