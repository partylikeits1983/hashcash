fn main(prev_hash: pub Field, nonce: Field, solution: Field, _address: pub Field) {
    let prev_hash_bytes: [u8; 32] = prev_hash.to_be_bytes();
    let nonce_bytes: [u8; 32] = nonce.to_be_bytes();

    let mut combined_bytes: [u8; 64] = [0; 64];
    for i in 0..32 {
        combined_bytes[i] = prev_hash_bytes[i];
        combined_bytes[i + 32] = nonce_bytes[i];
    }
    let hash = std::sha256::sha256_var(combined_bytes, 64);
    let hash_field: Field = Field::from_be_bytes::<32>(hash);

    println(hash_field);

    assert(hash_field == solution);
}

#[test]
fn test_main() {
    let prev_hash: Field = 0x0;
    let nonce: Field = 0x0;
    let address: Field = 0;

    let input_1_bytes: [u8; 32] = prev_hash.to_be_bytes();
    let input_2_bytes: [u8; 32] = nonce.to_be_bytes();

    let mut combined_bytes: [u8; 64] = [0; 64];
    for i in 0..32 {
        combined_bytes[i] = input_1_bytes[i];
        combined_bytes[i + 32] = input_2_bytes[i];
    }
    let hash = std::sha256::sha256_var(combined_bytes, 64);
    let solution: Field = Field::from_be_bytes::<32>(hash);

    main(prev_hash, nonce, solution, address);
}

#[test]
fn test_hash() {
    // test case: input_1 and input_2
    let input_1: Field = 0x0;
    let input_2: Field = 0x0;

    // Convert each Field to an 8-byte array in big-endian order
    let input_1_bytes: [u8; 32] = input_1.to_be_bytes();
    let input_2_bytes: [u8; 32] = input_2.to_be_bytes();

    let mut combined_bytes: [u8; 64] = [0; 64];
    for i in 0..32 {
        combined_bytes[i] = input_1_bytes[i];
        combined_bytes[i + 32] = input_2_bytes[i];
    }
    let hash = std::sha256::sha256_var(combined_bytes, 64);
    let hash_field: Field = Field::from_be_bytes::<32>(hash);

    let zero = 0;

    println(zero);
    // Print out for debugging
    println("input_1 bytes:");
    println(input_1_bytes);
    println("input_2 bytes:");
    println(input_2_bytes);
    println("combined:");
    println(combined_bytes);
    println("hash:");
    println(hash);
    println(hash_field);
}
