// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

import {IBozo} from "../src/IBozo.sol";

contract Bozo is Test {
    address impl;

    function deployProxy(address _impl) internal returns (address deployed) {
        // Proxy taken from the eip1967 code:
        bytes memory bytecode = abi.encodePacked(
            hex"73",
            _impl,
            hex"7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc55603a8060403d393df3365f5f375f5f365f7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc545af45f3d5f5f3e3d9161003857fd5bf3"
        );
        assembly {
            deployed := create(0, add(bytecode, 0x20), mload(bytecode))
        }
    }

    function createGame() internal returns (IBozo a) {
        a = IBozo(deployProxy(impl));
    }

    function setUp() external {
        impl = IArbFoundry(address(vm)).deployStylusCode(
            "bozo.wasm"
        );
    }
}
