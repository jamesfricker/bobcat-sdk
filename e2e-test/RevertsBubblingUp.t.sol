// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

import {ERC20} from "./ERC20.sol";

contract TestERC20 is ERC20 {
    function name() public override pure returns (string memory) {
        return "Hello";
    }

    function symbol() public override pure returns (string memory) {
        return "World";
    }
}

contract TestERC20Two {
    function transferFrom(address,address,uint256) external pure returns (bool) {
        revert("testing");
    }
}

interface IRevertsBubblingUp {
    function test(TestERC20Two target) external;
}

contract RevertsBubblingUp is Test {
    TestERC20Two erc20;
    IRevertsBubblingUp revertsBubblingUp;

    function setUp() public {
        erc20 = new TestERC20Two();
        revertsBubblingUp = IRevertsBubblingUp(
            IArbFoundry(address(vm)).deployStylusCode(
                "e2e-test/reverts-bubbling-up.wasm"
            )
        );
    }

    function testShouldRevert() external {
        try revertsBubblingUp.test(erc20) {
            revert("Didn't revert");
        }
        catch Error(string memory m) {
            assertEq("testing", m);
        }
    }
}
