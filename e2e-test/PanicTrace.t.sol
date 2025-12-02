// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface IPanicRevert {
    function panic() external;
}

contract PanicRevert is Test {
    IPanicRevert panicRevert;

    function setUp() public {
        panicRevert = IPanicRevert(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/panic-revert.wasm"
        ));
    }

    function testPanic() public {
        /* panicRevert.panic(); */
    }
}
