// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface ITracingCounter {
    function hello() external;
}

contract TracingCounter is Test {
    ITracingCounter tracingCounter;

    function setUp() public {
        tracingCounter = ITracingCounter(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/tracing-counter.wasm"
        ));
    }

    function testFuzz_counter(uint256 x) public {
        vm.assume(x > uint256(type(uint32).max));
        for(uint i = 0; i < x; i++) {
            tracingCounter.hello();
        }
        uint256 counter;
        uint256 id = uint256(keccak256(abi.encodePacked("bobcat.tracing.counter"))) - 1;
        assembly {
            counter := tload(id)
        }
        assertEq(x, counter);
    }
}
