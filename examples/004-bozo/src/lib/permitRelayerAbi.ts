export const permitRelayerAbi = [
  {
    type: 'function',
    name: 'mint',
    stateMutability: 'nonpayable',
    inputs: [
      { name: '_amount', type: 'uint256' },
      { name: '_comment', type: 'bytes32' },
      { name: '_deadline', type: 'uint256' },
      { name: '_v', type: 'uint8' },
      { name: '_r', type: 'bytes32' },
      { name: '_s', type: 'bytes32' },
    ],
    outputs: [],
  },
] as const;
