// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {console} from "forge-std/console.sol";

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface IMulDiv {
    function muldiv(uint256,uint256) external pure returns (uint256);
}

contract MulDiv is Test {
    IMulDiv mulDiv;

    function setUp() public {
        mulDiv = IMulDiv(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/mul-div.wasm"
        ));
    }

    function testMulDiv() public {
        assertEq(14798, mulDiv.muldiv(123, 12031));
    }
}
