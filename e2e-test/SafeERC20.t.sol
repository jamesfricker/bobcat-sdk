// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {console} from "forge-std/console.sol";

import "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface ISafeERC20 {
    function test(address) external pure returns (bool);
}

contract ERC20FalseReply {
    function transfer(address,uint256) external pure returns (bool) {
        return false;
    }
}

contract ERC20TrueReply {
    function transfer(address,uint256) external pure returns (bool) {
        return true;
    }
}

contract ERC20NoReply {
    function transfer(address,uint256) external pure {}
}

contract ERC20Revert {
    function transfer(address,uint256) external pure {
        revert("uhoh stinky");
    }
}

contract MulDiv is Test {
    ISafeERC20 safeErc20;

    function setUp() public {
        safeErc20 = ISafeERC20(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/safe-erc20.wasm"
        ));
    }

    function test_falseReply() external {
        assertEq(false, safeErc20.test(address(new ERC20FalseReply())));
    }

    function test_trueReply() external {
        assertEq(true, safeErc20.test(address(new ERC20TrueReply())));
    }

    function test_noReply() external {
        assertEq(true, safeErc20.test(address(new ERC20NoReply())));
    }

    function test_revert() external {
        assertEq(false, safeErc20.test(address(new ERC20Revert())));
    }
}
