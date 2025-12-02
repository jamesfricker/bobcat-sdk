// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface ICounter {
    function number() external view returns (uint256);
    function setNumber(uint256 value) external;
    function mulNumber(uint256 value) external;
    function addNumber(uint256 value) external;
    function increment() external;
    function addFromMsgValue() external payable;
}

contract CounterTest is Test {
    ICounter counter;

    function setUp() public {
        counter = ICounter(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/counter.wasm"
        ));
    }

    function testSetAndGetNumber() public {
        counter.setNumber(10);
        assertEq(counter.number(), 10);
    }

    function testAddNumber() public {
        counter.setNumber(5);
        counter.addNumber(3);
        assertEq(counter.number(), 8);
    }

    function testMulNumber() public {
        counter.setNumber(4);
        counter.mulNumber(3);
        assertEq(counter.number(), 12);
    }

    function testIncrement() public {
        counter.setNumber(7);
        counter.increment();
        assertEq(counter.number(), 8);
    }

    function testAddFromMsgValue() public {
        counter.setNumber(1);
        counter.addFromMsgValue{value: 9 ether}();
        assertEq(counter.number(), 9 ether + 1);
    }
}
