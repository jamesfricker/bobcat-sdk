// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface IMathsZone {
    function constBasic() external pure returns (uint256);
    function constShift() external pure returns (uint256);
    function constPow() external pure returns (uint256);
    function addConst(uint256) external pure returns (uint256);
    function mask(uint256) external pure returns (uint256);
}

contract MathsZone is Test {
    IMathsZone mathsZone;

    function setUp() public {
        mathsZone = IMathsZone(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/maths-zone.wasm"
        ));
    }

    function test_constants() public view {
        assertEq(mathsZone.constBasic(), 7);
        assertEq(mathsZone.constShift(), 0xff00);
        assertEq(mathsZone.constPow(), 65536);
    }

    function testFuzz_addConst(uint256 x) public view {
        vm.assume(x <= type(uint256).max - 1_000_000);
        assertEq(mathsZone.addConst(x), x + 1_000_000);
    }

    function testFuzz_mask(uint256 x) public view {
        assertEq(mathsZone.mask(x), x & 0xff);
    }
}
