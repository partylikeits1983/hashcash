/// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

contract HashCash {
    function hash(uint256 x, uint256 y) public pure returns (uint256) {
        uint256 noir_prime = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
        uint256 sha_256_hash = uint256(sha256(abi.encodePacked(x, y)));
        uint256 hash_mod_prime = sha_256_hash % noir_prime;

        return hash_mod_prime;
    }
}
