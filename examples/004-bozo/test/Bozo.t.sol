// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Test} from "forge-std/Test.sol";

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
        if (allowance[_from][msg.sender] != type(uint256).max)
            allowance[_from][msg.sender] -= _value;
        _transfer(_from, _to, _value);
    }

    function _transfer(address _from, address _to, uint256 _value) internal {
        if (_value > balanceOf[_from]) {
            revert("transfer too much");
        }
        unchecked {
            balanceOf[_from] -= _value;
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

    function createGame(uint256 alexBal, uint256 erikBal) internal returns (IBozo a, ERC20 token) {
        a = IBozo(IArbFoundry(address(vm)).deployStylusCode("bozo.wasm"));
        token = new ERC20();
        token.transfer(alex, alexBal);
        token.transfer(erik, erikBal);
        a.initialise(alex, address(token));
        vm.prank(alex);
        token.approve(address(a), type(uint256).max);
        vm.prank(erik);
        token.approve(address(a), type(uint256).max);
    }

    function test_fuzzGame(uint256 alexDeposit) external {
        vm.assume(alexDeposit > 100);
        vm.assume(1e50 > alexDeposit);
        uint256 erikDeposit = alexDeposit + ((alexDeposit * 3) / 10);
        uint256 pool = alexDeposit + erikDeposit;
        (IBozo a, ERC20 token) = createGame(alexDeposit, erikDeposit);
        vm.warp(405611000);
        vm.prank(alex);
        a.play(alexDeposit, alex, 0, 0);
        assertEq(0, token.balanceOf(alex));
        assertEq(0, a.currentEpoch());
        vm.warp(405611745);
        vm.prank(erik);
        a.play(erikDeposit, erik, 0, 0);
        assertEq(0, token.balanceOf(erik));
        assertEq(block.timestamp + 2400, a.deadline());
        vm.warp(405910536);
        vm.prank(alex);
        a.distributeRewards(0, alex, 123);
        vm.assumeNoRevert();
        uint256 exp = pool - ((pool * 5) / 100);
        vm.assertApproxEqAbsDecimal(
            exp,
            token.balanceOf(alex) + token.balanceOf(erik),
            1000,
            6
        );
    }
}
