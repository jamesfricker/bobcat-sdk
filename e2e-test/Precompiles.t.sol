// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Vm, Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface IPrecompiles {
    function ecrecover_(bytes32, uint8, bytes32, bytes32) external view returns (address);
}

contract Ecrecover is Test {
    IPrecompiles precompiles;

    function setUp() public {
        precompiles = IPrecompiles(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/precompiles.wasm"
        ));
    }

    function test_fuzzEcrecover(bytes32 digest) public {
        // It isn't so important that we create a new private key for this:
        Vm.Wallet memory wallet = vm.createWallet(uint256(keccak256(bytes("1"))));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(wallet, digest);
        assertEq(wallet.addr, precompiles.ecrecover_(digest, v, r, s));
    }
}
