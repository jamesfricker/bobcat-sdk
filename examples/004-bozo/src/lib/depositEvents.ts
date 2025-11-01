import type { AbiEvent } from 'viem';

export const depositEventAbi = [
  {
    type: 'event',
    name: 'DepositMade',
    inputs: [
      {
        name: 'recipient',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amount',
        type: 'uint256',
        indexed: true,
      },
      {
        name: 'currentPool',
        type: 'uint256',
        indexed: true,
      },
    ],
  },
] as const satisfies readonly AbiEvent[];

export const DEPOSIT_LOOKBACK_BLOCKS = 20_000n;
