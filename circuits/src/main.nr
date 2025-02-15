fn main(prev_hash: pub Field, nonce: Field, solution: Field, _address: pub Field) {

    let prev_hash_bytes: [u8; 8] = prev_hash.to_be_bytes();
    let nonce_bytes: [u8; 8] = nonce.to_be_bytes();

    let mut combined_bytes: [u8; 16] = [0; 16];
    for i in 0..8 {
        combined_bytes[i] = prev_hash_bytes[i];
        combined_bytes[i + 8] = nonce_bytes[i];
    }



}

#[test]
fn test_hash() {
    // test case: input_1 and input_2
    let input_1: Field = 0x1;
    let input_2: Field = 0x1;

    // Convert each Field to an 8-byte array in big-endian order
    let input_1_bytes: [u8; 8] = input_1.to_be_bytes();
    let input_2_bytes: [u8; 8] = input_2.to_be_bytes();

    let mut combined_bytes: [u8; 16] = [0; 16];
    for i in 0..8 {
        combined_bytes[i] = input_1_bytes[i];
        combined_bytes[i + 8] = input_2_bytes[i];
    }

    // Hash the combined bytes
    let hash = std::sha256::sha256_var(combined_bytes, 16);

    // Print out for debugging
    println("input_1 bytes:");
    println(input_1_bytes);
    println("input_2 bytes:");
    println(input_2_bytes);
    println("combined:");
    println(combined_bytes);
    println("hash:");
    println(hash);

    let hash_254: Field = Field::from_be_bytes::<32>(hash);
    println(hash_254);

}
