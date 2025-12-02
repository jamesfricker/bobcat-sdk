// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface IConsoleLogTxt {
    function print(uint256,uint256) external pure returns (uint256);
}

contract ConsoleLogTxt is Test {
    IConsoleLogTxt consoleLogTxt;

    function setUp() public {
        consoleLogTxt = IConsoleLogTxt(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/console-log-txt.wasm"
        ));
    }

    function testPrint() public {
        /* console.log(consoleLogTxt.print(123, 19201)); */
    }
}
