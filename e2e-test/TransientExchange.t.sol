// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface ITransientExchange {
    // We use view here so that a user can use transient storage.
    function execute() external returns (bool);
}

contract TransientStorage is Test {
    ITransientExchange transientExchange;

    function setUp() public {
        transientExchange = ITransientExchange(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/transient-exchange.wasm"
        ));
    }

    function testWorking() public {
        assert(transientExchange.execute());
    }
}
