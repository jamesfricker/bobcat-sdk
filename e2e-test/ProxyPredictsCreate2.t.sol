// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface IProxyPredictsCreate2 {
    function deploy() external returns (address);
    function predict() external view returns (address);
}

contract ProxyPredictsCreate2 is Test {
    IProxyPredictsCreate2 proxyPredictsCreate2;

    function setUp() public {
        proxyPredictsCreate2 = IProxyPredictsCreate2(
            IArbFoundry(address(vm)).deployStylusCode(
                "e2e-test/proxy-predicts-create2.wasm"
            )
        );
    }

    function test_fuzzEqualToEstimation(address _sender) external {
        vm.prank(_sender);
        assertEq(proxyPredictsCreate2.predict(), proxyPredictsCreate2.deploy());
    }
}
