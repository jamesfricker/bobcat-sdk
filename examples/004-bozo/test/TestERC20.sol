// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {ERC20} from "../../../e2e-test/ERC20.sol";

contract TestERC20 is ERC20 {
   function name() public pure override returns (string memory) {
       return "TestERC20";
   }

   function symbol() public pure override returns (string memory) {
      return "TE20";
   }

   function mint(address _r, uint256 _a) external {
       _mint(_r, _a);
   }
}
