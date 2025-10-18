// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {console} from "forge-std/console.sol";

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface IVrfCallback {
    function initiate() external returns (uint256);
    function wasCalled() external view returns (bool);
}

contract ChainlinkVrfTest is Test {
    IVrfCallback vrfCallback;

    function setUp() public {
        vm.createSelectFork("https://sepolia-rollup.arbitrum.io/rpc", 205928753);
        vrfCallback = IVrfCallback(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/chainlink-vrf-test.wasm"
        ));
    }

    function testCallback() public {
        assert(!vrfCallback.wasCalled());
    }

    function testInitiate() public {
        console.log(vrfCallback.initiate());
    }
}
