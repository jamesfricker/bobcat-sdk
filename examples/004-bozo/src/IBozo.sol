// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

interface IBozo {
    function play(
        address asset,
        uint256 camelotMinAssetOut,
        uint256 camelotDeadline,
        uint256 amount,
        address recipient
    ) external returns (uint256 epoch, uint256 deposited);

    struct WinAmount {
        address winner;
        uint256 amountReceived;
    }

    function distributeRewards(address rewardRecipient) external returns (
        uint256 callerReward,
        WinAmount[] memory winners
    );

    function poolSize() external view returns (uint256);
    function poolAsset() external view returns (address);
}
