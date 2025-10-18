// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface VrfCallback {
    function initiate() external;
}

contract ChainlinkVrfTest is Test {
    VrfCallback vrfCallback;

    function setUp() public {
        vm.createSelectFork("https://sepolia-rollup.arbitrum.io/rpc", 205928753);
        vrfCallback = VrfCallback(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/chainlink-vrf-test.wasm"
        ));
    }

    function testInitiate() public {
        vrfCallback.initiate();
    }
}
