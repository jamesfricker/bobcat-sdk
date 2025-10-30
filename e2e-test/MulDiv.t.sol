// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {console} from "forge-std/console.sol";

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface IMulDiv {
    function online(uint256,uint256,uint256) external pure returns (uint256);
    function offline(uint256,uint256,uint256) external pure returns (uint256);
}

contract MulDiv is Test {
    IMulDiv mulDiv;

    function setUp() public {
        mulDiv = IMulDiv(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/mul-div.wasm"
        ));
    }

    function testFuzz_online(uint256 x, uint256 y, uint256 z) public {
        assertEq(mulDiv.offline(x, y, z), mulDiv.online(x, y, z));
    }
}
