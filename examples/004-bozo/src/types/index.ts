// Core game types
export type GameStatus = 'Active' | 'Finalizing' | 'Closed';
export type ClaimType = 'winner' | 'community' | null;

export interface Player {
  address: string;
  fid?: number;
  handle?: string;
}

export interface GameState {
  potTokenAmount: string;
  minPct: number;
  minToResetTokens: number;
  deadline: string;
  lastDepositor: Player;
  status: GameStatus;
  homeToken: string;
  chain: 'base' | 'arbitrum';
  nextGameStartsAt?: string;
}

export interface Deposit {
  ts: string;
  address: string;
  fid?: number;
  handle?: string;
  amountToken: string;
  potAfterToken: string;
  txHash: string;
}

export interface WinnerEvent {
  ts: string;
  address: string;
  amountToken: string;
  isLottery: boolean;
  txHash: string;
}

export type PlayerActivityType = 'deposit' | 'winner' | 'lottery';

export interface PlayerActivityItem {
  ts: string;
  type: PlayerActivityType;
  amountToken: string;
  txHash: string;
}

export interface BozoComment {
  wallet: string;
  content: string;
  txHash: string;
}

export interface RouteQuote {
  ok: boolean;
  depositAmount: string;
  meetsMinPct: boolean;
  estArrivalSec: number;
  hops: Array<{
    chain: string;
    asset: string;
    action: string;
    toChain?: string;
    to?: string;
  }>;
  router: string;
  tx: {
    to: string;
    data: string;
    value: string;
    chainId: number;
  };
  pendingIntent?: {
    expiresInSec: number;
    hash: string;
  };
}

export interface Winners {
  winner: {
    address: string;
    fid?: number;
    handle?: string;
    amountToken: string;
  };
  community: Array<{
    address: string;
    fid?: number;
    handle?: string;
    amountToken: string;
  }>;
}

export interface RoundWinner {
  type: 'winner' | 'lottery';
  roundNumber: number;
  address: string;
  fid?: number;
  handle?: string;
  amountToken: string;
  ts: string;
}

export interface PendingIntent {
  hash: string;
  expiresAt: Date;
  softExtendSec: number;
}

export interface LeaderboardEntry {
  rank: number;
  address: string;
  handle?: string;
  fid?: number;
  wins?: number;
  totalWon?: number;
  firstIns?: number;
  totalDeposited?: number;
  timesSniped?: number;
  totalLost?: number;
}
