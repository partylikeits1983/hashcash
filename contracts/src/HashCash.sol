/// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract HashCash is ERC20 {
    uint256 public constant MAX_EMISSION = 21_000_000 ether; // 21 million tokens
    uint256 public constant DURATION = 20 * 365 * 24 * 60 * 60; // 20 years in seconds
    uint256 public GENESIS_TIMESTAMP;

    uint256 public LAST_MINE_TIMESTAMP;
    uint256 public CURRENT_HASH;
    uint256 public CURRENT_DIFFICULTY;

    constructor(string memory name, string memory symbol) ERC20(name, symbol) {
        GENESIS_TIMESTAMP = block.timestamp;
    }

    function hash(uint256 x, uint256 y) public pure returns (uint256) {
        uint256 noir_prime = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
        uint256 sha_256_hash = uint256(sha256(abi.encodePacked(x, y)));
        uint256 hash_mod_prime = sha_256_hash % noir_prime;

        return hash_mod_prime;
    }

    function emission(uint256 lastTimestamp) public view returns (uint256) {
        require(lastTimestamp >= GENESIS_TIMESTAMP, "Last timestamp must be after start");
        require(block.timestamp >= lastTimestamp, "Current time must be after last emission");
    }

    function mint(bytes memory proof, bytes32[] memory publicInputs) public {
        // prove validity of submission:
        // 1) check validity of proof
        // 2) check validity of PoW difficulty
        // 3) require at least 10 minute gaps
        require(LAST_MINE_TIMESTAMP + 600 < block.timestamp, "10 minute PoW time");

        // compute next hash as h(proof) -> ensures PoW begins each period, not before
        CURRENT_HASH = uint256(sha256(proof));

        // Update current public vars
        uint256 last_mine_time = LAST_MINE_TIMESTAMP;
        LAST_MINE_TIMESTAMP = block.timestamp;
        uint256 delta = LAST_MINE_TIMESTAMP - last_mine_time;

        // delta will have an effect on current difficulty
    }

    // mint
}
