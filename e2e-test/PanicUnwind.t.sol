// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {console} from "forge-std/console.sol";

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface IPanicUnwind {
    function panic() external;
}

contract ChainlinkVrfTest is Test {
    IPanicUnwind panicUnwind;

    function setUp() public {
        panicUnwind = IPanicUnwind(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/panic-unwind.wasm"
        ));
    }

    function testPanic() public {
        /* panicUnwind.panic(); */
    }
}
