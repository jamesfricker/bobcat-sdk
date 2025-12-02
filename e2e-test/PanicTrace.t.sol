// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

contract PanicTrace is Test {
    function test_panicTrace() public {
        address c = IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/panic-trace.wasm"
        );
        (bool rc, bytes memory rd) = c.call("");
        assert(!rc);
        //trace str: src/main.rs:8
        assertEq(hex"0x08c379a0000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000187472616365207374723a207372632f6d61696e2e72733a3800000000", rd);
    }
}
