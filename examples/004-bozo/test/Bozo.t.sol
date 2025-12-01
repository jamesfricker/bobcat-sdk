// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test} from "forge-std/Test.sol";
import "forge-std/console.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

import {IBozo} from "../src/IBozo.sol";

contract ERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(
        address indexed sender,
        address indexed recipient,
        uint256 amount
    );

    constructor() {
        balanceOf[msg.sender] = type(uint256).max;
    }

    function transferFrom(address _from, address _to, uint256 _value) external {
        console.log("transfer from", _value);
        console.log("balance for transfer from", balanceOf[_from]);
        console.log("allowance for transfer from", allowance[_from][_to]);
        if (allowance[_from][msg.sender] != type(uint256).max)
            allowance[_from][msg.sender] -= _value;
        _transfer(_from, _to, _value);
    }

    function _transfer(address _from, address _to, uint256 _value) internal {
        console.log("transfer", _value);
        balanceOf[_from] -= _value;
        unchecked {
            balanceOf[_to] += _value;
        }
        emit Transfer(_from, _to, _value);
    }

    function transfer(address _to, uint256 _value) external {
        _transfer(msg.sender, _to, _value);
    }

    function approve(address _spender, uint256 _value) external {
        allowance[msg.sender][_spender] = _value;
    }
}

contract Bozo is Test {
    address alex = 0x6221A9c005F6e47EB398fD867784CacfDcFFF4E7;
    address erik = 0xdd50872400Fb1dA43FFfA87Be38b85AA79DFa0ae;

    event Transfer(
        address indexed sender,
        address indexed recipient,
        uint256 amount
    );

    function createGame() internal returns (IBozo a) {
        a = IBozo(IArbFoundry(address(vm)).deployStylusCode("bozo.wasm"));
        ERC20 token = new ERC20();
        token.transfer(alex, 1e18);
        a.initialise(alex, address(token));
        vm.prank(alex);
        token.approve(address(a), type(uint256).max);
        vm.prank(erik);
        token.approve(address(a), type(uint256).max);
    }

    function test_everything() external {
        IBozo a = createGame();
        console.log("current deadline before first player", a.deadline());
        console.log("current block timestamp", block.timestamp);
        vm.prank(alex);
        a.play(76293945312500, alex, 0, 0);
        console.log("current deadline after first player", a.deadline());
        assertEq(0, a.currentEpoch());
        vm.warp(405611745);
        vm.prank(erik);
        a.play(6756406260067100922, erik, 0, 0);
        console.log("current deadline before second player", a.deadline());
        //assertEq(0, a.currentEpoch());
        console.log(a.deadline());
        assertEq(block.timestamp + 2400, a.deadline());
        vm.warp(405910536);
        vm.prank(alex);
        vm.expectEmit();
        emit Transfer(address(a), alex, 10);
        a.distributeRewards(1, alex, 123);
    }
}
