// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {console} from "forge-std/console.sol";

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface IConsoleLogTxt {
    function print(uint256,uint256) external pure returns (uint256);
}

contract ChainlinkVrfTest is Test {
    IConsoleLogTxt consoleLogTxt;

    function setUp() public {
        consoleLogTxt = IConsoleLogTxt(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/console-log-txt.wasm"
        ));
    }

    function testPrint() public {
        console.log(consoleLogTxt.print(123, 19201));
    }
}
