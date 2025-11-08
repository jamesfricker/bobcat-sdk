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

interface IRevertsBubblingUp {
    function test(TestERC20 target) external;
}

contract RevertsBubblingUp is Test {
    TestERC20 erc20;
    IRevertsBubblingUp revertsBubblingUp;

    function setUp() public {
        erc20 = new TestERC20();
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
        catch Error(string memory msg) {
            revert(msg);
        }
        catch (bytes memory rd) {
             assertEq(ERC20.InsufficientAllowance.selector, bytes4(rd));
        }
    }
}
