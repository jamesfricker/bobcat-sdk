// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {IBozo} from "./IBozo.sol";

interface IToken {
    function approve(address, uint256) external;
    function permit(
        address owner,
        address spender,
        uint value,
        uint deadline,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external;
    function transferFrom(address, address, uint256) external;
}

contract PermitRelayer {
    IBozo public immutable BOZO;
    IToken public constant TOKEN = IToken(0x4ecf61a6c2FaB8A047CEB3B3B263B401763e9D49);

    constructor(IBozo _bozo) {
        BOZO = _bozo;
        TOKEN.approve(address(BOZO), type(uint256).max);
    }

    function mint(
        uint256 _amount,
        bytes32 _comment,
        uint256 _deadline,
        uint8 _v,
        bytes32 _r,
        bytes32 _s
    ) external {
        if (_r != bytes32(0))
            TOKEN.permit(msg.sender, address(this), type(uint256).max, _deadline, _v, _r, _s);
        TOKEN.transferFrom(msg.sender, address(this), _amount);
        BOZO.play(_amount, msg.sender, _comment);
    }
}
