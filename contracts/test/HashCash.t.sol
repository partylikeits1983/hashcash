// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Test, console} from "forge-std/Test.sol";
import {HashCash} from "../src/HashCash.sol";

contract HashCashTest is Test {
    HashCash public hashcash;

    function setUp() public {
        hashcash = new HashCash();
    }

    function test_Increment() public view {
        uint256 x = 0;
        uint256 y = 0;

        uint256 result = hashcash.hash(x, y);

        console.log(result);
        console.logBytes32(bytes32(result));
    }
}
