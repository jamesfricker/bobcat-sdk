// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface IProxyPredictsCreate2 {
    function deploy(address) external returns (address);
    function predict(address) external view returns (address);
}

contract Swag {
    address immutable IMPL;

    constructor() {
        IMPL = address(this);
    }

    function migrate() external {
        bytes32 slotImpl = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;
        address impl = IMPL;
        assembly {
            sstore(slotImpl, impl)
        }
    }

    function swag() external pure returns (string memory) {
        return "Swag";
    }
}

contract ProxyPredictsCreate2 is Test {
    IProxyPredictsCreate2 proxyPredictsCreate2;
    Swag swag;

    function setUp() public {
        proxyPredictsCreate2 = IProxyPredictsCreate2(
            IArbFoundry(address(vm)).deployStylusCode(
                "e2e-test/proxy-predicts-create2.wasm"
            )
        );
        swag = new Swag();
    }

    function test_fuzzEqualToEstimation(address _sender) external {
        vm.startPrank(_sender);
        Swag d = Swag(proxyPredictsCreate2.deploy(address(swag)));
        assertEq(proxyPredictsCreate2.predict(address(swag)), address(d));
        d.migrate();
        assertEq("Swag", d.swag());
    }
}
