// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {console} from "forge-std/console.sol";

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

contract Swag {
    string public hello;
    uint256 public no;

    function setHello(string memory x) external returns (string memory) {
        string memory p = hello;
        hello = x;
        return p;
    }

    function setNumber(uint256 x) external returns (uint256) {
        no = x;
        return no;
    }
}

interface IProxyCreator {
    function create(address) external returns (address);
}

contract Eip1967 is Test {
    IProxyCreator proxyCreator;
    Swag swag;

    function setUp() public {
        proxyCreator = IProxyCreator(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/eip1967-proxy.wasm"
        ));
        swag = Swag(proxyCreator.create(address(new Swag())));
    }

    function testFuzz_hello(string memory s) public {
        swag.setHello(s);
        assertEq(s, swag.hello());
    }

    function testFuzz_number(uint256 x) public {
        assertEq(x, swag.setNumber(x));
    }
}
