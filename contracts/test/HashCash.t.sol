// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {HashCash} from "../src/HashCash.sol";

contract HashCashTest is Test {
    HashCash public hashcash;

    function setUp() public {
        hashcash = new HashCash();
    }

    function test_Increment() public {
        hashcash.hash();
    }
}
