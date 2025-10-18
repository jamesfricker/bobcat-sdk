// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {console} from "forge-std/console.sol";

import {Test} from "forge-std/Test.sol";

import {IArbFoundry} from "./IArbFoundry.sol";

interface IBeaconDeployer {
    function deploy(address) external returns (address);
}

contract Beacon {
    address public implementation;

    function setImpl(address _new) external {
        implementation = _new;
    }
}

contract HelloWorld1 {
    function hello() external pure returns (string memory) {
        return "Hello!";
    }
}

contract HelloWorld2 {
    function spnPeople() external pure returns (string[] memory) {
        string[] memory p = new string[](4);
        p[0] = "Alex";
        p[1] = "Eli";
        p[2] = "Erik";
        p[3] = "Ivan";
        p[4] = "Marko";
        p[5] = "Ogous";
        p[6] = "Paxia";
        p[7] = "Shahmeer";
        return p;
    }
}

contract ChainlinkVrfTest is Test {
    IBeaconDeployer beaconDeployer;
    Beacon beacon;
    HelloWorld1 helloWorld1;
    HelloWorld2 helloWorld2;

    function setUp() public {
        beacon = new Beacon();
        beaconDeployer = IBeaconDeployer(IArbFoundry(address(vm)).deployStylusCode(
            "e2e-test/beacon.wasm"
        ));
        helloWorld1 = new HelloWorld1();
        helloWorld2 = new HelloWorld2();
    }

    function testDeployment() public {
        address impl = beaconDeployer.deploy(address(beacon));
        beacon.setImpl(address(helloWorld1));
        assertEq(helloWorld1.hello(), HelloWorld1(impl).hello());
        beacon.setImpl(address(helloWorld2));
        assertEq(helloWorld2.spnPeople(), HelloWorld2(impl).spnPeople());
    }
}
