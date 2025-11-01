// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

interface IBozo {
    function play(
        address asset,
        uint256 camelotMinAssetOut,
        uint256 camelotDeadline,
        uint256 amount,
        address recipient
    ) external returns (uint256 epoch, uint256 deposited);

    function distributeRewards(
        address rewardRecipient,
        uint256 rngWord
    ) external returns (
        uint256 winnerReward,
        uint256 losersDistributed,
        address[] memory winners
    );

    function poolSize() external view returns (uint256);
    function poolAsset() external view returns (address);
    function lastBettorAmount() external view returns (uint256);
    function lastBettorAddress() external view returns (uint256);
    function deadline() external view returns (uint256);
    function playerCount() external view returns (uint256);
    function ticketCount() external view returns (uint256);

    function upgrade(address newImpl) external;

    function changeAdmin(address newAdmin) external;

    event DepositMade(
        address indexed recipient,
        uint256 indexed amount,
        uint256 indexed currentPool
    );

    event WinnerChosen(
        address indexed recipient,
        uint256 indexed amount,
        bool indexed isLottery
    );
}
