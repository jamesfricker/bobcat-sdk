// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test} from "forge-std/Test.sol";
import "forge-std/console.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

import {IBozo} from "../src/IBozo.sol";

contract Bozo is Test {
    event Transfer(
        address indexed sender,
        address indexed recipient,
        uint256 indexed amount
    );

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

    function test_everything() external {
        uint256 id = vm.createFork("https://arb1.arbitrum.io/rpc", 405609348);
        vm.selectFork(id);
        impl = IArbFoundry(address(vm)).deployStylusCode(
            "bozo.wasm"
        );
        IBozo a = createGame();
        vm.prank(0x6221A9c005F6e47EB398fD867784CacfDcFFF4E7);
        console.log("current deadline before first player", a.deadline());
        console.log("current block timestamp", block.timestamp);
        a.play(76293945312500, 0x6221A9c005F6e47EB398fD867784CacfDcFFF4E7, 0, 0);
        console.log("current deadline after first player", a.deadline());
        assertEq(0, a.currentEpoch());
        vm.warp(405611745);
        vm.prank(0xdd50872400Fb1dA43FFfA87Be38b85AA79DFa0ae);
        a.play(6756406260067100922, 0xdd50872400Fb1dA43FFfA87Be38b85AA79DFa0ae, 0, 0);
        console.log("current deadline before second player", a.deadline());
        //assertEq(0, a.currentEpoch());
        console.log(a.deadline());
        assertEq(block.timestamp + 2400, a.deadline());
        vm.warp(405910536);
        vm.prank(0x6221A9c005F6e47EB398fD867784CacfDcFFF4E7);
        vm.expectEmit();
        emit Transfer(address(a), 0x6221A9c005F6e47EB398fD867784CacfDcFFF4E7, 10);
        a.distributeRewards(1, 0x6221A9c005F6e47EB398fD867784CacfDcFFF4E7, 123);
    }
}
