// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface IMulDiv {
    function uniswap(uint256,uint256,uint256) external pure returns (uint256);
    function widening(uint256,uint256,uint256) external pure returns (uint256);
    function ruint(uint256,uint256,uint256) external pure returns (uint256);
}

contract MulDiv is Test {
    IMulDiv mulDiv;

    function setUp() public {
        mulDiv = IMulDiv(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/mul-div.wasm"
        ));
    }

    function testFuzz_ruint(uint256 x, uint256 y, uint256 z) public view {
        mulDiv.ruint(x, y, z);
    }

    function testFuzz_widening(uint256 x, uint256 y, uint256 z) public view {
        mulDiv.widening(x, y, z);
    }

    function testFuzz_uniswap(uint256 x, uint256 y, uint256 z) public view {
        mulDiv.uniswap(x, y, z);
    }

    function testFuzz_online(uint256 x, uint256 y, uint256 z) public view {
        uint256 ruint = mulDiv.ruint(x, y, z);
        uint256 online = mulDiv.uniswap(x, y, z);
        uint256 widening = mulDiv.widening(x, y, z);
        assertEq(ruint, online);
        assertEq(ruint, widening);
    }
}
