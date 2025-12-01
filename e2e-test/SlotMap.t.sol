// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {console} from "forge-std/console.sol";

import "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface ISlotMap {
    function print(uint256,uint256) external pure returns (uint256);
}

contract MulDiv is Test {
    ISlotMap slotMap;

    function setUp() public {
        slotMap = ISlotMap(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/slot-map.wasm"
        ));
    }

    function testFuzz_correct(uint256 xx, uint256 xy, uint256 yx, uint256 yy) public view {
        vm.assume(xx != yx && xy != yy);
        assertNotEq(slotMap.print(xx, xy), slotMap.print(yx, yy));
    }
}
