// SPDX-License-Identifier: UNLICENSED
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

    function setUp() external {
        vm.createSelectFork("https://arb1.arbitrum.io/rpc");
        c = IBozo(IArbFoundry(address(vm)).deployStylusCode(
            "bozo.wasm"
        ));
        vm.deal(address(this), 10e18);
        weth.deposit{value: 10e18}();
        weth.approve(address(c), type(uint256).max);
        assertEq(type(uint256).max, weth.allowance(address(this), address(c)));
    }

    function test_contractDeployed() public view {
        assertNotEq(address(0), address(c));
    }

    function test_play() external {
        (uint256 epoch,) = c.play(
            address(weth),
            0,
            block.timestamp + 1,
            1e18,
            address(this)
        );
        assertEq(0, epoch);
    }
}
