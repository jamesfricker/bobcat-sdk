// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

import {IBozo} from "../src/IBozo.sol";

interface IWETH10 {
    function deposit() payable external;
    function balanceOf(address) external view returns (uint256);
    function approve(address, uint256) external;
    function allowance(address, address) external view returns (uint256);
}

contract Bozo is Test {
    IBozo c;
    IWETH10 weth = IWETH10(0x82aF49447D8a07e3bd95BD0d56f35241523fBab1);

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

    function setUp() external {
        vm.createSelectFork("https://arb1.arbitrum.io/rpc");
        c = IBozo(deployProxy(IArbFoundry(address(vm)).deployStylusCode(
            "bozo.wasm"
        )));
        vm.deal(address(this), 10e18);
        weth.deposit{value: 10e18}();
        weth.approve(address(c), type(uint256).max);
        assertEq(type(uint256).max, weth.allowance(address(this), address(c)));
    }

    function test_contractDeployed() public view {
        assertNotEq(address(0), address(c));
    }

    function test_fuzzFlay() external {
        // Test that a user can start the game, a number of other users can
        // deposit liquidity, then the winner goes to redeem, and some of the
        // losers receive their money. This code tests that the contract remains
        // solvent.
        (uint256 epoch,) = c.play(
            address(weth),
            0,
            block.timestamp + 1,
            1e18,
            address(this)
        );
        assertEq(0, epoch);
        c.distributeRewards(0, address(this), 123);
    }
}
