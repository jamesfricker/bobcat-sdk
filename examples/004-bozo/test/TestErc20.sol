// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {ERC20} from "./ERC20.sol";

contract TestErc20 is ERC20 {
    function name() public pure override returns (string memory) {
        return "TestErc20";
    }

    function symbol() public pure override returns (string memory) {
        return "TE20";
    }
}