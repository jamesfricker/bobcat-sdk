// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

import {TestErc20} from "./TestErc20.sol";

import {IBozo} from "../src/IBozo.sol";

contract Bozo is Test {
    IBozo c;
    TestErc20 token;

    function setUp() external {
        c = IBozo(IArbFoundry(address(vm)).deployStylusCode(
            "bozo.wasm"
        ));
        vm.etch("0xaf88d065e77c8cC2239327C5EDb3A432268e5831", TestErc20.bytecode);
    }

    function test_contractDeployed() public view {
        assertNotEq(address(0), address(c));
    }

    function test_assetWorking() public view {
        assertEq(0x015580BaeBBdD8dDacDD7c66fBF3008564B06359, c.poolAsset());
    }

    function test_
}
