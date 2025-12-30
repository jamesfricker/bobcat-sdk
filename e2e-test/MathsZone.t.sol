// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface IMathsZone {
    function constValue() external pure returns (uint256);
    function combined(uint256,uint256) external pure returns (uint256);
    function xorMix(uint256,uint256) external pure returns (uint256);
}

contract MathsZone is Test {
    IMathsZone mathsZone;

    function setUp() public {
        mathsZone = IMathsZone(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/maths-zone.wasm"
        ));
    }

    function test_constValue() public view {
        assertEq(mathsZone.constValue(), 261);
    }

    function testFuzz_combined(uint256 x, uint256 y) public view {
        vm.assume(x <= type(uint64).max);
        vm.assume(y <= type(uint64).max);
        uint256 expected = ((x + y) * 3) / 2;
        assertEq(mathsZone.combined(x, y), expected);
    }

    function testFuzz_xorMix(uint256 x, uint256 y) public view {
        uint256 expected = (x ^ y) | 0xff;
        assertEq(mathsZone.xorMix(x, y), expected);
    }
}
