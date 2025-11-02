import { useState, useEffect, useMemo, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { Button } from '../components/ui/button';
import { Card } from '../components/ui/card';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '../components/ui/tabs';
import { BozoModal } from '../components/BozoModal';
import { EndGameScreen } from '../components/EndGameScreen';
import { HowItWorksDialog } from '../components/HowItWorksDialog';
import { Avatar, AvatarFallback } from '../components/ui/avatar';
import { Alert, AlertDescription } from '../components/ui/alert';
import { GameState, Deposit, Winners, WinnerEvent, PlayerActivityItem } from '../types';
import { config } from '../lib/config';
import { formatAddress, formatTokenAmount, formatUsd, getTimeRemaining } from '../lib/utils';
import { Loader2, AlertTriangle, Settings, HelpCircle } from 'lucide-react';
import { toast } from 'sonner@2.0.3';

import { useAccount, useReadContract, usePublicClient } from 'wagmi';
import { arbitrum } from 'wagmi/chains';

import { ConnectButton } from '@rainbow-me/rainbowkit';
import { useComments } from '../providers/CommentsProvider';
import { formatUnits } from 'viem';
import {
  DEPOSIT_LOOKBACK_BLOCKS,
  WINNER_LOOKBACK_BLOCKS,
  depositEventAbi,
  winnerEventAbi,
} from '../lib/depositEvents';

const BOZO_CONTRACT_ADDRESS = '0x944e82782bb29394939483f3380c69b3b89e6426' as const;
const GAME_START_DELAY_MINUTES = 60;
const DEFAULT_TOKEN_PRICE_USD = 1;
const HARD_CODED_MIN_RESET_USD = 1;
const DEFAULT_HOME_TOKEN = 'ETH';
const ZERO_ADDRESS = '0x0000000000000000000000000000000000000000' as const;
const MIN_RESET_PREMIUM_MULTIPLIER = 1.05;

const bozoAbi = [
  {
    type: 'function',
    name: 'poolSize',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ name: '', type: 'uint256' }],
  },
  {
    type: 'function',
    name: 'poolAsset',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ name: '', type: 'address' }],
  },
  {
    type: 'function',
    name: 'lastBettorAmount',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ name: '', type: 'uint256' }],
  },
  {
    type: 'function',
    name: 'lastBettorAddress',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ name: '', type: 'uint256' }],
  },
  {
    type: 'function',
    name: 'deadline',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ name: '', type: 'uint256' }],
  },
] as const;

const erc20Abi = [
  {
    type: 'function',
    name: 'symbol',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ name: '', type: 'string' }],
  },
  {
    type: 'function',
    name: 'decimals',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ name: '', type: 'uint8' }],
  },
] as const;

export function Game() {
  const navigate = useNavigate();
  const [deposits, setDeposits] = useState<Deposit[]>([]);
  const [isInitialLoading, setIsInitialLoading] = useState(true);
  const [bozoModalOpen, setBozoModalOpen] = useState(false);
  const [howItWorksOpen, setHowItWorksOpen] = useState(false);
  const winners: Winners | null = null;
  const [currentTime, setCurrentTime] = useState(Date.now());
  const [poolAssetAddress, setPoolAssetAddress] = useState<`0x${string}` | null>(null);
  const [assetDecimals, setAssetDecimals] = useState<number>(18);
  const [homeToken, setHomeToken] = useState<string>(DEFAULT_HOME_TOKEN);

  const { address: accountAddress, isConnected } = useAccount();
  const { getCommentForTxHash, refresh: refreshComments } = useComments();
  const publicClient = usePublicClient();
  const [winnerEvents, setWinnerEvents] = useState<WinnerEvent[]>([]);

  const fallbackDeadline = useMemo(
    () => new Date(Date.now() + GAME_START_DELAY_MINUTES * 60 * 1000).toISOString(),
    []
  );

  const { data: poolSizeData } = useReadContract({
    address: BOZO_CONTRACT_ADDRESS,
    abi: bozoAbi,
    functionName: 'poolSize',
    chainId: arbitrum.id,
    query: {
      refetchInterval: 15000,
    },
  });

  const { data: poolAssetData } = useReadContract({
    address: BOZO_CONTRACT_ADDRESS,
    abi: bozoAbi,
    functionName: 'poolAsset',
    chainId: arbitrum.id,
  });

  const { data: lastBettorAmountData } = useReadContract({
    address: BOZO_CONTRACT_ADDRESS,
    abi: bozoAbi,
    functionName: 'lastBettorAmount',
    chainId: arbitrum.id,
    query: {
      refetchInterval: 15000,
    },
  });

  const { data: lastBettorAddressData } = useReadContract({
    address: BOZO_CONTRACT_ADDRESS,
    abi: bozoAbi,
    functionName: 'lastBettorAddress',
    chainId: arbitrum.id,
    query: {
      refetchInterval: 15000,
    },
  });

  const { data: deadlineData } = useReadContract({
    address: BOZO_CONTRACT_ADDRESS,
    abi: bozoAbi,
    functionName: 'deadline',
    chainId: arbitrum.id,
    query: {
      refetchInterval: 15000,
    },
  });

  const { data: assetSymbolData } = useReadContract({
    address: poolAssetAddress ?? ZERO_ADDRESS,
    abi: erc20Abi,
    functionName: 'symbol',
    chainId: arbitrum.id,
    query: {
      enabled: Boolean(poolAssetAddress),
    },
  });

  const { data: assetDecimalsData } = useReadContract({
    address: poolAssetAddress ?? ZERO_ADDRESS,
    abi: erc20Abi,
    functionName: 'decimals',
    chainId: arbitrum.id,
    query: {
      enabled: Boolean(poolAssetAddress),
    },
  });

  useEffect(() => {
    if (typeof poolAssetData === 'string') {
      setPoolAssetAddress(poolAssetData as `0x${string}`);
    }
  }, [poolAssetData]);

  useEffect(() => {
    if (typeof assetSymbolData === 'string' && assetSymbolData.length > 0) {
      setHomeToken(assetSymbolData);
    }
  }, [assetSymbolData]);

  useEffect(() => {
    if (typeof assetDecimalsData === 'number') {
      setAssetDecimals(assetDecimalsData);
    }
  }, [assetDecimalsData]);

  const poolSizeTokens = useMemo(() => {
    if (typeof poolSizeData === 'bigint') {
      try {
        return formatUnits(poolSizeData, assetDecimals);
      } catch (error) {
        console.error('Failed to format pool size:', error);
      }
    }
    return '0';
  }, [assetDecimals, poolSizeData]);

  const tokenPriceUsd = useMemo(() => {
    const normalizedToken = homeToken.toUpperCase();
    if (normalizedToken.includes('USDC')) {
      return 1;
    }

    return DEFAULT_TOKEN_PRICE_USD;
  }, [homeToken]);

  const poolSizeUsd = useMemo(() => {
    const numericAmount = parseFloat(poolSizeTokens);
    if (Number.isFinite(numericAmount)) {
      return numericAmount * tokenPriceUsd;
    }
    return 0;
  }, [poolSizeTokens, tokenPriceUsd]);

  const lastBettorAmountTokens = useMemo(() => {
    if (typeof lastBettorAmountData === 'bigint') {
      try {
        return formatUnits(lastBettorAmountData, assetDecimals);
      } catch (error) {
        console.error('Failed to format last bettor amount:', error);
      }
    }
    return '0';
  }, [assetDecimals, lastBettorAmountData]);

  const lastBettorAmountUsd = useMemo(() => {
    const numericAmount = parseFloat(lastBettorAmountTokens);
    if (Number.isFinite(numericAmount)) {
      return numericAmount * tokenPriceUsd;
    }
    return 0;
  }, [lastBettorAmountTokens, tokenPriceUsd]);

  const minToResetUsd = useMemo(() => {
    if (lastBettorAmountUsd > 0) {
      return lastBettorAmountUsd * MIN_RESET_PREMIUM_MULTIPLIER;
    }
    return HARD_CODED_MIN_RESET_USD;
  }, [lastBettorAmountUsd]);

  const lastBettorAddress = useMemo(() => {
    if (typeof lastBettorAddressData === 'string' && lastBettorAddressData.length > 0) {
      return lastBettorAddressData as `0x${string}`;
    }

    if (typeof lastBettorAddressData === 'bigint') {
      const hex = lastBettorAddressData.toString(16).padStart(40, '0');
      return `0x${hex.slice(-40)}` as `0x${string}`;
    }

    return ZERO_ADDRESS;
  }, [lastBettorAddressData]);

  const deadlineIso = useMemo(() => {
    if (typeof deadlineData === 'bigint') {
      const deadlineMs = Number(deadlineData) * 1000;
      if (Number.isFinite(deadlineMs) && deadlineMs > 0) {
        return new Date(deadlineMs).toISOString();
      }
    }
    return fallbackDeadline;
  }, [deadlineData, fallbackDeadline]);

  const displayPot = useMemo(() => {
    const numericAmount = parseFloat(poolSizeTokens);
    if (Number.isFinite(numericAmount)) {
      const formatOptions: Intl.NumberFormatOptions =
        numericAmount < 1
          ? { minimumFractionDigits: 2, maximumFractionDigits: 6 }
          : { minimumFractionDigits: 2, maximumFractionDigits: 2 };

      return `${numericAmount.toLocaleString('en-US', formatOptions)} ${homeToken}`;
    }
    return `0 ${homeToken}`;
  }, [homeToken, poolSizeTokens]);

  const timeRemaining = useMemo(() => getTimeRemaining(deadlineIso), [deadlineIso, currentTime]);
  const gameStatus: GameState['status'] = 'Active';
  const isGameClosed = gameStatus === 'Closed';
  const isGameActive = gameStatus === 'Active';
  const isGamePaused = false;
  const poolAssetDisplay = poolAssetAddress ? formatAddress(poolAssetAddress) : 'Unknown';

  const loadGame = useCallback(() => {
    // Game state is derived directly from contract reads, so this is a no-op placeholder.
    return undefined;
  }, []);

  const game: GameState = useMemo(
    () => ({
      potTokenAmount: poolSizeTokens,
      potUsd: poolSizeUsd,
      minPct: 0.01,
      minToResetUsd,
      deadline: deadlineIso,
      lastDepositor: {
        address: lastBettorAddress,
      },
      status: gameStatus,
      homeToken,
      chain: 'arbitrum',
      nextGameStartsAt: undefined,
    }),
    [deadlineIso, homeToken, minToResetUsd, poolSizeTokens, poolSizeUsd, gameStatus, lastBettorAddress]
  );

  const fetchBlockTimestamps = useCallback(
    async (events: Array<{ blockNumber?: bigint }>) => {
      if (!publicClient) {
        return new Map<bigint, string>();
      }

      const blockNumbers = Array.from(
        new Set(
          events
            .map((event) => event.blockNumber)
            .filter((blockNumber): blockNumber is bigint => typeof blockNumber === 'bigint')
        )
      );

      if (blockNumbers.length === 0) {
        return new Map<bigint, string>();
      }

      const blocks = await Promise.all(
        blockNumbers.map((blockNumber) => publicClient.getBlock({ blockNumber }))
      );

      const blockTimestamps = new Map<bigint, string>();
      blocks.forEach((block, index) => {
        const timestamp = Number(block.timestamp) * 1000;
        blockTimestamps.set(blockNumbers[index], new Date(timestamp).toISOString());
      });

      return blockTimestamps;
    },
    [publicClient]
  );

  const loadDeposits = useCallback(
    async ({ isInitial = false }: { isInitial?: boolean } = {}) => {
      if (!publicClient) {
        return;
      }

      if (isInitial) {
        setIsInitialLoading(true);
      }

      try {
        await refreshComments().catch((err) => {
          console.error('Failed to refresh comments:', err);
        });

        const latestBlock = await publicClient.getBlockNumber();
        const fromBlock =
          latestBlock > DEPOSIT_LOOKBACK_BLOCKS ? latestBlock - DEPOSIT_LOOKBACK_BLOCKS : 0n;

        const events = await publicClient.getContractEvents({
          address: config.contracts.bozo as `0x${string}`,
          abi: depositEventAbi,
          eventName: 'DepositMade',
          fromBlock,
          toBlock: latestBlock,
        });

        const recentEvents = events.slice(-100);
        const blockTimestamps = await fetchBlockTimestamps(recentEvents);

        const depositsFromEvents = [...recentEvents]
          .reverse()
          .map((event) => {
            if (!event.transactionHash || !event.blockNumber) {
              return null;
            }

            const recipient = event.args?.recipient as string | undefined;
            const amountRaw = event.args?.amount;
            const poolRaw = event.args?.currentPool;
            if (!recipient) {
              return null;
            }

            const timestampIso = blockTimestamps.get(event.blockNumber);
            if (!timestampIso) {
              return null;
            }

            const amount = typeof amountRaw === 'bigint' ? amountRaw : 0n;
            const pool = typeof poolRaw === 'bigint' ? poolRaw : 0n;

            const amountToken = formatUnits(amount, assetDecimals);
            const potAfterToken = formatUnits(pool, assetDecimals);

            return {
              ts: timestampIso,
              address: recipient,
              amountToken,
              amountUsd: parseFloat(amountToken),
              potAfterUsd: parseFloat(potAfterToken),
              txHash: event.transactionHash,
            } satisfies Deposit;
          })
          .filter((deposit): deposit is Deposit => deposit !== null);

        setDeposits(depositsFromEvents);
      } catch (error) {
        console.error('Failed to load deposits:', error);
      } finally {
        if (isInitial) {
          setIsInitialLoading(false);
        }
      }
    },
    [assetDecimals, fetchBlockTimestamps, publicClient, refreshComments]
  );

  const loadWinners = useCallback(async () => {
    if (!publicClient) {
      return;
    }

    try {
      const latestBlock = await publicClient.getBlockNumber();
      const fromBlock =
        latestBlock > WINNER_LOOKBACK_BLOCKS ? latestBlock - WINNER_LOOKBACK_BLOCKS : 0n;

      const events = await publicClient.getContractEvents({
        address: config.contracts.bozo as `0x${string}`,
        abi: winnerEventAbi,
        eventName: 'WinnerChosen',
        fromBlock,
        toBlock: latestBlock,
      });

      const recentEvents = events.slice(-100);
      const blockTimestamps = await fetchBlockTimestamps(recentEvents);

      const winnersFromEvents = [...recentEvents]
        .reverse()
        .map((event) => {
          if (!event.transactionHash || !event.blockNumber) {
            return null;
          }

          const recipient = event.args?.recipient as string | undefined;
          const amountRaw = event.args?.amount;
          const isLotteryRaw = event.args?.isLottery;

          if (!recipient) {
            return null;
          }

          const timestampIso = blockTimestamps.get(event.blockNumber);
          if (!timestampIso) {
            return null;
          }

          const amount = typeof amountRaw === 'bigint' ? amountRaw : 0n;
          const amountToken = formatUnits(amount, assetDecimals);

          return {
            ts: timestampIso,
            address: recipient,
            amountToken,
            amountUsd: parseFloat(amountToken),
            isLottery: Boolean(isLotteryRaw),
            txHash: event.transactionHash,
          } satisfies WinnerEvent;
        })
        .filter((winner): winner is WinnerEvent => winner !== null);

      setWinnerEvents(winnersFromEvents);
    } catch (error) {
      console.error('Failed to load winners:', error);
    }
  }, [assetDecimals, fetchBlockTimestamps, publicClient]);

  useEffect(() => {
    loadGame();

    const timerInterval = setInterval(() => {
      setCurrentTime(Date.now());
    }, 1000);

    return () => {
      clearInterval(timerInterval);
    };
  }, [loadGame]);

  useEffect(() => {
    if (!publicClient) {
      return;
    }

    loadDeposits({ isInitial: true });

    const interval = setInterval(() => {
      loadDeposits();
    }, 5000);

    return () => {
      clearInterval(interval);
    };
  }, [publicClient, loadDeposits]);

  useEffect(() => {
    if (!publicClient) {
      return;
    }

    loadWinners();

    const interval = setInterval(() => {
      loadWinners();
    }, 10000);

    return () => {
      clearInterval(interval);
    };
  }, [publicClient, loadWinners]);

  const yourActivity = useMemo(() => {
    if (!accountAddress) {
      return [] as PlayerActivityItem[];
    }

    const normalizedAddress = accountAddress.toLowerCase();

    const depositActivity: PlayerActivityItem[] = deposits
      .filter((deposit) => deposit.address.toLowerCase() === normalizedAddress)
      .map((deposit) => ({
        ts: deposit.ts,
        type: 'deposit',
        amountToken: deposit.amountToken,
        amountUsd: deposit.amountUsd,
        txHash: `${deposit.txHash}-deposit`,
      }));

    const winnerActivity: PlayerActivityItem[] = winnerEvents
      .filter((winner) => winner.address.toLowerCase() === normalizedAddress)
      .map((winner) => ({
        ts: winner.ts,
        type: winner.isLottery ? 'lottery' : 'winner',
        amountToken: winner.amountToken,
        amountUsd: winner.amountUsd,
        txHash: `${winner.txHash}-${winner.isLottery ? 'lottery' : 'winner'}`,
      }));

    return [...depositActivity, ...winnerActivity].sort((a, b) => {
      return new Date(b.ts).getTime() - new Date(a.ts).getTime();
    });
  }, [accountAddress, deposits, winnerEvents]);

  const handleShare = async () => {
    const text = 'RIP BOZO 🤡';
    const shareText = `${text}\n${window.location.href}`;

    try {
      if (navigator.clipboard && navigator.clipboard.writeText) {
        await navigator.clipboard.writeText(shareText);
        toast.success('Link copied to clipboard');
      } else {
        toast.success('Share: ' + shareText, {
          duration: 5000
        });
      }
    } catch (error) {
      toast.success('Share: ' + shareText, {
        duration: 5000
      });
    }
  };

  const formatTime = (ts: string) => {
    const date = new Date(ts);
    const now = Date.now();
    const diff = now - date.getTime();
    const minutes = Math.floor(diff / 60000);

    if (minutes < 1) return 'JUST NOW';
    if (minutes === 1) return '1 MIN AGO';
    if (minutes < 60) return `${minutes} MINS AGO`;
    const hours = Math.floor(minutes / 60);
    if (hours === 1) return '1 HOUR AGO';
    if (hours < 24) return `${hours} HOURS AGO`;
    const days = Math.floor(hours / 24);
    if (days === 1) return '1 DAY AGO';
    return `${days} DAYS AGO`;
  };

  if (isInitialLoading && deposits.length === 0) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <Loader2 className="w-8 h-8 animate-spin text-[#FF4B4B]" />
      </div>
    );
  }

  // Show end game screen if game is closed
  if (isGameClosed && winners) {
    return (
      <div className="min-h-screen bg-background relative overflow-hidden">
        {/* Decorative elements */}
        <div className="absolute top-8 left-8 w-32 h-32 rounded-full bg-[#FF4B4B] opacity-20 blur-3xl"></div>
        <div className="absolute top-8 right-8 w-32 h-32 rounded-full bg-[#F6C445] opacity-20 blur-3xl"></div>

        {/* Top Nav */}
        <header className="relative z-10 border-b border-border/50 bg-background/80 backdrop-blur-sm">
          <div className="container mx-auto px-4 py-4">
            <div className="flex items-center justify-between max-w-6xl mx-auto">
              <div className="flex items-center gap-6">
                <div className="flex items-center gap-2">
                  <div className="w-8 h-8 rounded-full bg-[#FF4B4B] flex items-center justify-center">
                    <div className="w-4 h-4 rounded-full bg-[#FFF2E1]" />
                  </div>
                  <span className="text-foreground tracking-wider">BOZO</span>
                </div>
                <nav className="flex items-center gap-6">
                  <button
                    onClick={() => navigate('/')}
                    className="text-sm text-foreground hover:text-[#F6C445] transition-colors"
                  >
                    GAME
                  </button>
                  <button
                    type="button"
                    disabled
                    className="text-sm text-muted-foreground cursor-not-allowed"
                  >
                    LEADERBOARD
                  </button>
                  <button
                    onClick={() => navigate('/faq')}
                    className="text-sm text-muted-foreground hover:text-foreground transition-colors"
                  >
                    FAQ
                  </button>
                  <button
                    onClick={() => setHowItWorksOpen(true)}
                    className="text-sm text-muted-foreground hover:text-foreground transition-colors flex items-center gap-1"
                  >
                    <HelpCircle className="w-4 h-4" />
                    HOW IT WORKS
                  </button>
                </nav>
              </div>
            </div>
          </div>
        </header>

        <EndGameScreen
          winners={winners}
          nextGameStartsAt={new Date(Date.now() + 300000)}
          homeToken={homeToken}
        />

        <HowItWorksDialog open={howItWorksOpen} onOpenChange={setHowItWorksOpen} />
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-background relative overflow-hidden">
      {/* Decorative Clown Nose - Top Left */}
      <div className="absolute top-8 left-8 w-32 h-32 rounded-full bg-[#FF4B4B] opacity-20 blur-3xl"></div>

      {/* Decorative Clown Nose - Top Right */}
      <div className="absolute top-8 right-8 w-32 h-32 rounded-full bg-[#F6C445] opacity-20 blur-3xl"></div>

      {/* Top Nav */}
      <header className="relative z-10 border-b border-border/50 bg-background/80 backdrop-blur-sm">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between max-w-6xl mx-auto">
            <div className="flex items-center gap-6">
              <div className="flex items-center gap-2">
                <div className="w-8 h-8 rounded-full bg-[#FF4B4B] flex items-center justify-center">
                  <div className="w-4 h-4 rounded-full bg-[#FFF2E1]" />
                </div>
                <span className="text-foreground tracking-wider">BOZO</span>
              </div>
              <nav className="flex items-center gap-6">
                <button
                  onClick={() => navigate('/')}
                  className="text-sm text-foreground hover:text-[#F6C445] transition-colors"
                >
                  GAME
                </button>
                <button
                  type="button"
                  disabled
                  className="text-sm text-muted-foreground cursor-not-allowed"
                >
                  LEADERBOARD
                </button>
                <button
                  onClick={() => navigate('/faq')}
                  className="text-sm text-muted-foreground hover:text-foreground transition-colors"
                >
                  FAQ
                </button>
                <button
                  onClick={() => setHowItWorksOpen(true)}
                  className="text-sm text-muted-foreground hover:text-foreground transition-colors flex items-center gap-1"
                >
                  <HelpCircle className="w-4 h-4" />
                  HOW IT WORKS
                </button>
              </nav>
            </div>

            <ConnectButton />
          </div>
        </div>
      </header>

      {/* Game Status Alert */}
      {isGamePaused && (
        <div className="container mx-auto px-4 mt-4 max-w-6xl">
          <Alert className="bg-[#FF4B4B]/10 border-[#FF4B4B]">
            <AlertTriangle className="h-4 w-4 text-[#FF4B4B]" />
            <AlertDescription className="text-foreground">
              Deposits paused. Check back soon.
            </AlertDescription>
          </Alert>
        </div>
      )}

      {/* Main Content */}
      <div className="container mx-auto px-4 py-12 max-w-6xl">
        {/* Top Section - Pot & Timer */}
        <div className="grid grid-cols-2 gap-8 mb-8">
          {/* Lottery Pool */}
          <div className="text-left">
            <div className="flex items-center gap-2 mb-2">
              <div className="w-2 h-2 rounded-full bg-[#2ED4B7] animate-pulse"></div>
              <span className="text-sm text-muted-foreground tracking-wider">LOTTERY POOL</span>
            </div>
            <div className="font-mono text-6xl text-[#F6C445] tracking-tight">{displayPot}</div>
            <div className="text-sm text-muted-foreground mt-1">
              AWARDED ACROSS TEN RANDOM BOZOS
            </div>
          </div>

          {/* Time Remaining */}
          <div className="text-right">
            <div className="text-sm text-muted-foreground mb-2 tracking-wider">TIME REMAINING</div>
            <div className={`font-mono text-6xl tracking-tight ${
              timeRemaining.total < 60000 ? 'text-[#FF4B4B]' : 'text-[#F6C445]'
            }`}>
              {timeRemaining.formatted}
            </div>
          </div>
        </div>

        {/* BOZO Button */}
        <Button
          onClick={() => setBozoModalOpen(true)}
          disabled={!isConnected || !isGameActive}
          className="w-full h-16 bg-[#F6C445] hover:bg-[#F6C445]/90 text-[#0E1020] text-xl tracking-widest mb-12"
        >
          {isConnected ? 'BOZO' : 'CONNECT TO BOZO'}
        </Button>

        {/* Activity Feed */}
        <div className="bg-[#1a1d32]/50 rounded-lg border border-border/50 overflow-hidden">
          <Tabs defaultValue="latest" className="w-full">
            <div className="border-b border-border/50 px-6 py-4">
              <div className="flex items-center justify-between">
                <h3 className="text-foreground tracking-wider">BOZOS</h3>
                <TabsList className="bg-transparent h-auto p-0 gap-6">
                  <TabsTrigger
                    value="latest"
                    className="bg-transparent data-[state=active]:bg-transparent data-[state=active]:text-[#2ED4B7] data-[state=active]:shadow-none text-muted-foreground px-0"
                  >
                    LATEST
                  </TabsTrigger>
                  <TabsTrigger
                    value="winners"
                    className="bg-transparent data-[state=active]:bg-transparent data-[state=active]:text-[#2ED4B7] data-[state=active]:shadow-none text-muted-foreground px-0"
                  >
                    WINNERS
                  </TabsTrigger>
                  <TabsTrigger
                    value="yours"
                    className="bg-transparent data-[state=active]:bg-transparent data-[state=active]:text-[#2ED4B7] data-[state=active]:shadow-none text-muted-foreground px-0"
                  >
                    YOURS
                  </TabsTrigger>
                </TabsList>
              </div>
            </div>

            <TabsContent value="latest" className="mt-0">
              <div className="divide-y divide-border/30">
                {deposits.slice(0, 10).map((deposit, index) => {
                  const commentText = getCommentForTxHash(deposit.txHash);
                  return (
                  <div
                    key={deposit.txHash}
                    className="px-6 py-4 hover:bg-[#252840]/50 transition-colors"
                  >
                    <div className="flex items-start justify-between gap-4">
                      <div className="flex items-start gap-3 flex-1 min-w-0">
                        <div className="text-sm font-mono text-muted-foreground w-6 mt-1">
                          #{deposits.length - index}
                        </div>
                        <Avatar className="w-8 h-8 mt-1">
                          <AvatarFallback className="bg-[#FF4B4B] text-[#FFF2E1] text-xs">
                            {deposit.handle?.[0]?.toUpperCase() || deposit.address.slice(2, 4).toUpperCase()}
                          </AvatarFallback>
                        </Avatar>
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center gap-2 mb-1">
                            <div className="text-sm text-foreground font-mono">
                              {deposit.handle || formatAddress(deposit.address)}
                            </div>
                            {index === 0 && (
                              <div className="flex items-center gap-1">
                                <div className="w-2 h-2 rounded-full bg-[#2ED4B7]"></div>
                                <span className="text-xs text-[#2ED4B7] tracking-wider">LEADER</span>
                              </div>
                            )}
                          </div>
                          {commentText && (
                            <div className="text-sm text-foreground/90 bg-[#252840]/80 rounded px-3 py-2 mb-2 mt-2">
                              &quot;{commentText}&quot;
                            </div>
                          )}
                        </div>
                      </div>
                      <div className="text-right flex-shrink-0">
                        <div className="text-sm font-mono text-foreground">
                          {formatTokenAmount(deposit.amountToken, 2)} ${homeToken}
                        </div>
                        <div className="text-xs text-muted-foreground">
                          {formatTime(deposit.ts)}
                        </div>
                      </div>
                    </div>
                  </div>
                  );
                })}
              </div>
            </TabsContent>

            <TabsContent value="winners" className="mt-0">
              <div className="divide-y divide-border/30">
                {winnerEvents.length === 0 ? (
                  <div className="px-6 py-12 text-center text-muted-foreground">
                    No winners have been recorded yet.
                  </div>
                ) : (
                  winnerEvents.map((winner, index) => (
                    <div
                      key={`${winner.txHash}-${index}`}
                      className="px-6 py-4 hover:bg-[#252840]/50 transition-colors"
                    >
                      <div className="flex items-start justify-between gap-4">
                        <div className="flex items-start gap-3 flex-1 min-w-0">
                          <div className="text-sm font-mono text-muted-foreground w-6 mt-1">
                            #{winnerEvents.length - index}
                          </div>
                          <Avatar className="w-8 h-8 mt-1">
                            <AvatarFallback className="bg-[#2ED4B7] text-[#0E1020] text-xs">
                              {winner.address.slice(2, 4).toUpperCase()}
                            </AvatarFallback>
                          </Avatar>
                          <div className="flex-1 min-w-0">
                            <div className="flex items-center gap-2 mb-1">
                              <div className="text-sm text-foreground font-mono">
                                {formatAddress(winner.address)}
                              </div>
                              <div className="flex items-center gap-1">
                                <div
                                  className={`w-2 h-2 rounded-full ${
                                    winner.isLottery ? 'bg-[#F6C445]' : 'bg-[#2ED4B7]'
                                  }`}
                                ></div>
                                <span
                                  className={`text-xs tracking-wider ${
                                    winner.isLottery ? 'text-[#F6C445]' : 'text-[#2ED4B7]'
                                  }`}
                                >
                                  {winner.isLottery ? 'LOTTERY WINNER' : 'TOP BOZO'}
                                </span>
                              </div>
                            </div>
                          </div>
                        </div>
                        <div className="text-right flex-shrink-0">
                          <div className="text-sm font-mono text-foreground">
                            {formatTokenAmount(winner.amountToken, 2)} ${homeToken}
                          </div>
                          <div className="text-xs text-muted-foreground">{formatTime(winner.ts)}</div>
                        </div>
                      </div>
                    </div>
                  ))
                )}
              </div>
            </TabsContent>

            <TabsContent value="yours" className="mt-0">
              {!isConnected ? (
                <div className="px-6 py-12 text-center text-muted-foreground">
                  Connect wallet to view your activity
                </div>
              ) : yourActivity.length === 0 ? (
                <div className="px-6 py-12 text-center text-muted-foreground">
                  No activity recorded for this wallet yet
                </div>
              ) : (
                <div className="divide-y divide-border/30">
                  {yourActivity.map((activity) => (
                    <div
                      key={activity.txHash}
                      className="px-6 py-4 hover:bg-[#252840]/50 transition-colors"
                    >
                      <div className="flex items-start justify-between gap-4">
                        <div className="flex items-start gap-3 flex-1 min-w-0">
                          <Avatar className="w-8 h-8 mt-1">
                            <AvatarFallback className="bg-[#FF4B4B] text-[#FFF2E1] text-xs">
                              {accountAddress?.slice(2, 4).toUpperCase()}
                            </AvatarFallback>
                          </Avatar>
                          <div className="flex-1 min-w-0">
                            <div className="flex items-center gap-2 mb-1">
                              <div className="text-sm text-foreground font-mono">
                                {formatAddress(accountAddress as `0x${string}`)}
                              </div>
                              <span
                                className={`text-xs tracking-wider ${
                                  activity.type === 'deposit'
                                    ? 'text-[#FF4B4B]'
                                    : activity.type === 'winner'
                                    ? 'text-[#2ED4B7]'
                                    : 'text-[#F6C445]'
                                }`}
                              >
                                {activity.type === 'deposit'
                                  ? 'DEPOSIT'
                                  : activity.type === 'winner'
                                  ? 'WINNER'
                                  : 'LOTTERY WINNER'}
                              </span>
                            </div>
                          </div>
                        </div>
                        <div className="text-right flex-shrink-0">
                          <div className="text-sm font-mono text-foreground">
                            {formatTokenAmount(activity.amountToken, 2)} ${homeToken}
                          </div>
                          <div className="text-xs text-muted-foreground">{formatTime(activity.ts)}</div>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </TabsContent>
          </Tabs>
        </div>

        {/* Info Footer */}
        <div className="mt-8 text-center text-xs text-muted-foreground space-y-1">
          <div>
            Min deposit: {formatUsd(minToResetUsd)} • Pool asset: {homeToken} ({poolAssetDisplay}) • 80% to winner • 20% to 10 random bozos
          </div>
          <div>
            <a
              href="https://github.com/stylus-developers-guild/bobcat-sdk/tree/trunk/examples/004-bozo"
              target="_blank"
              rel="noreferrer noopener"
              className="text-[#2ED4B7] hover:text-[#2ED4B7]/80 underline"
            >
              View the project on GitHub
            </a>
            <span className="mx-2 text-muted-foreground/80">•</span>
            <span className="text-[#FF4B4B]">This project is not audited.</span>
          </div>
          <div className="text-muted-foreground/80">
            This example was created for fun—please be careful and use your best judgment when interacting with it.
          </div>
        </div>
      </div>

      {/* Bozo Modal */}
      <BozoModal
        open={bozoModalOpen}
        onOpenChange={setBozoModalOpen}
        game={game}
        isConnected={isConnected}
        poolAssetAddress={poolAssetAddress}
        assetDecimals={assetDecimals}
        tokenPriceUsd={tokenPriceUsd}
      />

      <HowItWorksDialog open={howItWorksOpen} onOpenChange={setHowItWorksOpen} />
    </div>
  );
}
