import { useCallback, useEffect, useMemo, useState } from 'react';
import { Button } from './ui/button';
import { Label } from './ui/label';
import { Input } from './ui/input';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from './ui/select';
import { Alert, AlertDescription } from './ui/alert';
import { Loader2, ArrowRightLeft, Info, Sparkles, RefreshCcw } from 'lucide-react';
import { toast } from 'sonner@2.0.3';
import {
  configureDynamicChains,
  execute,
  getClient,
  getQuote,
  type Execute as RelayExecute,
  type RelayChain,
  type ProgressData,
} from '@relayprotocol/relay-sdk';
import { formatUnits, zeroAddress, keccak256, stringToHex } from 'viem';
import { arbitrum } from 'wagmi/chains';
import { useWalletClient } from 'wagmi';
import { config as appConfig } from '../lib/config';
import { bozoAbi } from '../lib/bozoAbi';

type ChainCurrency = NonNullable<RelayChain['currency']>;
type ChainToken = NonNullable<RelayChain['featuredTokens']>[0];

type RelayToken = {
  key: string;
  address: string;
  symbol: string;
  name: string;
  decimals: number;
  isNative: boolean;
};

type TokenSelection =
  | ({
      type: 'list';
    } & RelayToken)
  | {
      type: 'custom';
      address: string;
      symbol: string;
      decimals: number;
    };

interface RelaySwapSectionProps {
  enabled: boolean;
  poolAssetAddress: `0x${string}` | null;
  assetDecimals: number;
  destinationSymbol: string;
  accountAddress?: `0x${string}`;
  defaultOriginChainId?: number;
  depositAmount: string;
  depositAmountWei: bigint | null;
  comment: string;
  onPrefillAmount: (amount: string) => void;
}

const ZERO_TOKEN_KEY = `${zeroAddress.toLowerCase()}`;
const ZERO_BYTES32 =
  '0x0000000000000000000000000000000000000000000000000000000000000000' as const;

const getTokenKey = (chainId: number, tokenAddress?: string | null) => {
  const address = (tokenAddress ?? zeroAddress).toLowerCase();
  return `${chainId}:${address}`;
};

const normalizeToken = (
  chain: RelayChain,
  token: Partial<ChainToken | ChainCurrency>,
  fallbackSymbol: string,
  isNative?: boolean,
): RelayToken | null => {
  const address = (token.address ?? zeroAddress).toLowerCase();
  const decimals = typeof token.decimals === 'number' ? token.decimals : 18;
  const symbol = token.symbol?.trim() || fallbackSymbol;
  const name = token.name?.trim() || symbol;
  const metadata =
    typeof token === 'object' && token !== null && 'metadata' in token
      ? (token as ChainToken).metadata
      : undefined;

  if (!symbol) {
    return null;
  }

  return {
    key: getTokenKey(chain.id, token.address),
    address,
    symbol,
    name,
    decimals,
    isNative: Boolean(isNative ?? metadata?.isNative ?? address === zeroAddress),
  };
};

export function RelaySwapSection({
  enabled,
  poolAssetAddress,
  assetDecimals,
  destinationSymbol,
  accountAddress,
  defaultOriginChainId,
  depositAmount,
  depositAmountWei,
  comment,
  onPrefillAmount,
}: RelaySwapSectionProps) {
  const { data: walletClient } = useWalletClient();

  const [chains, setChains] = useState<RelayChain[]>([]);
  const [isLoadingChains, setIsLoadingChains] = useState(false);
  const [chainError, setChainError] = useState<string | null>(null);
  const [selectedChainId, setSelectedChainId] = useState<number | null>(null);
  const [tokenSelection, setTokenSelection] = useState<TokenSelection | null>(null);
  const [customTokenAddress, setCustomTokenAddress] = useState('');
  const [customTokenSymbol, setCustomTokenSymbol] = useState('');
  const [customTokenDecimals, setCustomTokenDecimals] = useState('18');
  const [quote, setQuote] = useState<RelayExecute | null>(null);
  const [isFetchingQuote, setIsFetchingQuote] = useState(false);
  const [isExecuting, setIsExecuting] = useState(false);
  const [quoteError, setQuoteError] = useState<string | null>(null);
  const [progressAction, setProgressAction] = useState<string | null>(null);

  const normalizedCommentHash = useMemo(() => {
    const trimmed = comment.trim();
    if (!trimmed) {
      return ZERO_BYTES32;
    }

    try {
      return keccak256(stringToHex(trimmed));
    } catch (error) {
      console.error('Failed to hash comment for Relay execution:', error);
      return ZERO_BYTES32;
    }
  }, [comment]);

  const resetForm = useCallback(() => {
    setQuote(null);
    setQuoteError(null);
    setProgressAction(null);
  }, []);

  useEffect(() => {
    if (!enabled) {
      return;
    }

    let isMounted = true;
    const client = getClient();
    if (client?.chains?.length) {
      setChains(client.chains);
    }

    const loadChains = async () => {
      setIsLoadingChains(true);
      setChainError(null);
      try {
        const dynamicChains = await configureDynamicChains();
        if (!isMounted) {
          return;
        }
        if (dynamicChains?.length) {
          setChains(dynamicChains);
        }
      } catch (error) {
        if (!isMounted) {
          return;
        }
        console.error('Failed to load Relay chain data:', error);
        setChainError('Using cached Relay chain configuration. Some assets may be missing.');
      } finally {
        if (isMounted) {
          setIsLoadingChains(false);
        }
      }
    };

    void loadChains();

    return () => {
      isMounted = false;
    };
  }, [enabled]);

  useEffect(() => {
    if (!enabled || !chains.length) {
      return;
    }

    setSelectedChainId((current) => {
      if (current && chains.some((chain) => chain.id === current)) {
        return current;
      }
      if (defaultOriginChainId && chains.some((chain) => chain.id === defaultOriginChainId)) {
        return defaultOriginChainId;
      }
      return chains[0]?.id ?? null;
    });
  }, [chains, defaultOriginChainId, enabled]);

  const selectedChain = useMemo(
    () => (selectedChainId ? chains.find((chain) => chain.id === selectedChainId) ?? null : null),
    [chains, selectedChainId],
  );

  const availableTokens = useMemo(() => {
    if (!selectedChain) {
      return [] as RelayToken[];
    }

    const seen = new Set<string>();
    const tokens: RelayToken[] = [];

    const addToken = (token: RelayToken | null) => {
      if (!token) {
        return;
      }
      if (seen.has(token.key)) {
        return;
      }
      seen.add(token.key);
      tokens.push(token);
    };

    addToken(normalizeToken(selectedChain, selectedChain.currency ?? {}, selectedChain.currency?.symbol ?? 'ETH', true));

    for (const token of selectedChain.featuredTokens ?? []) {
      addToken(normalizeToken(selectedChain, token, token.symbol ?? destinationSymbol));
    }

    for (const token of selectedChain.erc20Currencies ?? []) {
      addToken(normalizeToken(selectedChain, token, token.symbol ?? destinationSymbol));
    }

    return tokens;
  }, [destinationSymbol, selectedChain]);

  useEffect(() => {
    if (!enabled) {
      return;
    }

    if (tokenSelection?.type === 'custom') {
      return;
    }

    if (!availableTokens.length) {
      setTokenSelection(null);
      return;
    }

    setTokenSelection((current) => {
      if (current && current.type === 'list') {
        const existing = availableTokens.find((token) => token.key === current.key);
        if (existing) {
          return { type: 'list', ...existing };
        }
      }
      return { type: 'list', ...availableTokens[0] };
    });
  }, [availableTokens, enabled, tokenSelection?.type]);

  useEffect(() => {
    if (!enabled) {
      resetForm();
      return;
    }

    setQuote(null);
    setQuoteError(null);
    setProgressAction(null);
  }, [enabled, resetForm]);

  const parsedCustomDecimals = useMemo(() => {
    const parsed = Number.parseInt(customTokenDecimals, 10);
    if (Number.isFinite(parsed) && parsed >= 0 && parsed <= 36) {
      return parsed;
    }
    return null;
  }, [customTokenDecimals]);

  const resolvedToken = useMemo(() => {
    if (!tokenSelection) {
      return null;
    }

    if (tokenSelection.type === 'list') {
      return tokenSelection;
    }

    if (!customTokenAddress || !customTokenSymbol || parsedCustomDecimals === null) {
      return null;
    }

    return {
      type: 'custom' as const,
      address: customTokenAddress.trim().toLowerCase(),
      symbol: customTokenSymbol.trim(),
      decimals: parsedCustomDecimals,
    };
  }, [customTokenAddress, customTokenSymbol, parsedCustomDecimals, tokenSelection]);

  const normalizedOriginCurrency = useMemo(() => {
    if (!resolvedToken) {
      return null;
    }
    if (resolvedToken.type === 'list') {
      return resolvedToken.address;
    }
    return resolvedToken.address || ZERO_TOKEN_KEY;
  }, [resolvedToken]);

  const handleGetQuote = useCallback(async () => {
    if (!enabled) {
      return;
    }

    if (!accountAddress) {
      toast.error('Connect your wallet to continue.');
      return;
    }

    if (!walletClient) {
      toast.error('Wallet client not available.');
      return;
    }

    if (!poolAssetAddress) {
      toast.error('Destination asset is still loading. Try again in a moment.');
      return;
    }

    if (!selectedChainId) {
      toast.error('Select an origin chain.');
      return;
    }

    if (!resolvedToken || !normalizedOriginCurrency) {
      toast.error('Select an origin asset to bridge.');
      return;
    }

    if (!depositAmountWei || depositAmountWei === 0n) {
      toast.error('Enter a deposit amount above before requesting a Relay quote.');
      return;
    }

    try {
      setIsFetchingQuote(true);
      setQuoteError(null);
      setProgressAction(null);

      const quoteResponse = await getQuote(
        {
          chainId: selectedChainId,
          currency: normalizedOriginCurrency,
          toChainId: arbitrum.id,
          toCurrency: poolAssetAddress ?? zeroAddress,
          tradeType: 'EXACT_OUTPUT',
          amount: depositAmountWei.toString(),
          recipient: accountAddress,
          user: accountAddress,
          wallet: walletClient,
          txs: [
            {
              abi: bozoAbi,
              address: appConfig.contracts.bozo as `0x${string}`,
              functionName: 'play',
              args: [depositAmountWei, accountAddress, normalizedCommentHash],
            },
          ],
        },
        true,
      );

      setQuote(quoteResponse);
      const spendAmount = quoteResponse.details?.currencyIn?.amountFormatted;
      const spendSymbol = quoteResponse.details?.currencyIn?.currency?.symbol;
      if (spendAmount && spendSymbol) {
        toast.success(`Quote ready. Estimated spend ${spendAmount} ${spendSymbol}.`);
      } else {
        toast.success('Quote ready.');
      }
    } catch (error) {
      console.error('Failed to fetch Relay quote:', error);
      const description = error instanceof Error ? error.message : 'Unable to fetch quote.';
      setQuote(null);
      setQuoteError(description);
      toast.error('Failed to fetch Relay quote.', { description });
    } finally {
      setIsFetchingQuote(false);
    }
  }, [
    accountAddress,
    depositAmountWei,
    enabled,
    normalizedCommentHash,
    normalizedOriginCurrency,
    poolAssetAddress,
    resolvedToken,
    selectedChainId,
    walletClient,
  ]);

  const handleExecute = useCallback(async () => {
    if (!enabled) {
      return;
    }

    if (!quote) {
      toast.error('Generate a quote before executing.');
      return;
    }

    if (!depositAmountWei || depositAmountWei === 0n) {
      toast.error('Enter a deposit amount above before executing.');
      return;
    }

    if (!walletClient) {
      toast.error('Wallet client not available.');
      return;
    }

    try {
      setIsExecuting(true);
      setProgressAction('Preparing Relay execution');

      const execution = await execute({
        quote,
        wallet: walletClient,
        onProgress: (data: ProgressData) => {
          const action = data.currentStep?.action || data.currentStep?.description;
          if (action) {
            setProgressAction(action);
          }
        },
      });

      setQuote(execution.data);

      toast.success('Relay deposit submitted. Your Bozo play is being executed on Relay.');
      onPrefillAmount('');
      resetForm();
    } catch (error) {
      console.error('Relay execution failed:', error);
      const description = error instanceof Error ? error.message : 'Unable to execute Relay transaction.';
      toast.error('Relay execution failed.', { description });
    } finally {
      setIsExecuting(false);
      setProgressAction(null);
    }
  }, [depositAmountWei, enabled, onPrefillAmount, quote, resetForm, walletClient]);

  const estimatedOutput = useMemo(() => {
    if (!quote?.details?.currencyOut?.amountFormatted) {
      return null;
    }

    return `${quote.details.currencyOut.amountFormatted} ${quote.details.currencyOut.currency?.symbol ?? destinationSymbol}`;
  }, [destinationSymbol, quote?.details?.currencyOut]);

  const estimatedInput = useMemo(() => {
    if (!quote?.details?.currencyIn?.amountFormatted) {
      return null;
    }

    const symbol = quote.details.currencyIn.currency?.symbol;
    return symbol
      ? `${quote.details.currencyIn.amountFormatted} ${symbol}`
      : quote.details.currencyIn.amountFormatted;
  }, [quote?.details?.currencyIn]);

  const depositAmountDisplay = useMemo(() => {
    const trimmed = depositAmount.trim();
    if (!trimmed) {
      return null;
    }

    return `${trimmed} ${destinationSymbol}`;
  }, [depositAmount, destinationSymbol]);

  const minimumOutput = useMemo(() => {
    if (!quote?.details?.currencyOut?.minimumAmount) {
      return null;
    }

    try {
      const decimals = quote.details.currencyOut.currency?.decimals ?? assetDecimals;
      return formatUnits(BigInt(quote.details.currencyOut.minimumAmount), decimals);
    } catch (error) {
      console.error('Failed to format minimum amount from Relay quote:', error);
      return null;
    }
  }, [assetDecimals, quote?.details?.currencyOut?.currency?.decimals, quote?.details?.currencyOut?.minimumAmount]);

  const disabledExecute =
    !quote || isExecuting || !depositAmountWei || depositAmountWei === 0n;
  const hasDepositAmount = Boolean(depositAmountWei && depositAmountWei > 0n);
  const destinationReady = Boolean(poolAssetAddress);

  if (!enabled) {
    return null;
  }

  return (
    <div className="space-y-4 rounded-lg border border-border/40 bg-[#101225] p-4">
      <div className="flex flex-col gap-1">
        <div className="flex items-center gap-2 text-foreground">
          <Sparkles className="h-4 w-4 text-[#F6C445]" />
          <h4 className="text-sm font-semibold">Play with Relay</h4>
        </div>
        <p className="text-xs text-muted-foreground">
          Swap from another asset and Relay will execute your Bozo play automatically, calling the contract on your behalf.
        </p>
      </div>

      {chainError && (
        <Alert className="bg-[#F6C445]/10 border-[#F6C445] text-sm text-foreground">
          <Info className="h-4 w-4 text-[#F6C445]" />
          <AlertDescription>{chainError}</AlertDescription>
        </Alert>
      )}

      {!destinationReady && (
        <Alert className="bg-[#FF4B4B]/10 border-[#FF4B4B] text-xs text-foreground">
          <Info className="h-4 w-4 text-[#FF4B4B]" />
          <AlertDescription>Detecting the destination asset… try again in a few seconds.</AlertDescription>
        </Alert>
      )}

      <div className="grid gap-4 sm:grid-cols-2">
        <div className="space-y-2">
          <Label className="text-sm text-muted-foreground">From chain</Label>
          <Select
            value={selectedChainId ? String(selectedChainId) : ''}
            onValueChange={(value) => {
              setSelectedChainId(Number.parseInt(value, 10));
              setTokenSelection(null);
              setQuote(null);
              setQuoteError(null);
            }}
            disabled={isLoadingChains || chains.length === 0}
          >
            <SelectTrigger className="bg-[#252840] border-0 text-left">
              <SelectValue placeholder={isLoadingChains ? 'Loading chains…' : 'Select a chain'} />
            </SelectTrigger>
            <SelectContent className="bg-[#1a1d32] border-border text-foreground">
              {chains.map((chain) => (
                <SelectItem key={chain.id} value={String(chain.id)}>
                  {chain.displayName}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="space-y-2">
          <Label className="text-sm text-muted-foreground">From asset</Label>
          <Select
            value={
              tokenSelection?.type === 'list'
                ? tokenSelection.key
                : tokenSelection?.type === 'custom'
                  ? 'custom'
                  : ''
            }
            onValueChange={(value) => {
              if (value === 'custom') {
                setTokenSelection({
                  type: 'custom',
                  address: customTokenAddress,
                  symbol: customTokenSymbol,
                  decimals: parsedCustomDecimals ?? 18,
                });
              } else {
                const next = availableTokens.find((token) => token.key === value);
                setTokenSelection(next ? { type: 'list', ...next } : null);
              }
              setQuote(null);
              setQuoteError(null);
            }}
            disabled={availableTokens.length === 0 && !tokenSelection}
          >
            <SelectTrigger className="bg-[#252840] border-0 text-left">
              <SelectValue placeholder={
                availableTokens.length === 0 ? 'No assets found' : 'Select an asset'
              } />
            </SelectTrigger>
            <SelectContent className="bg-[#1a1d32] border-border text-foreground max-h-64 overflow-y-auto">
              {availableTokens.map((token) => (
                <SelectItem key={token.key} value={token.key}>
                  {token.symbol} · {token.name}
                </SelectItem>
              ))}
              <SelectItem value="custom">Custom token…</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      {tokenSelection?.type === 'custom' && (
        <div className="grid gap-3 sm:grid-cols-3">
          <div className="space-y-2">
            <Label className="text-sm text-muted-foreground">Token address</Label>
            <Input
              value={customTokenAddress}
              onChange={(event) => setCustomTokenAddress(event.target.value)}
              placeholder="0x…"
              className="bg-[#252840] border-0"
            />
          </div>
          <div className="space-y-2">
            <Label className="text-sm text-muted-foreground">Symbol</Label>
            <Input
              value={customTokenSymbol}
              onChange={(event) => setCustomTokenSymbol(event.target.value.toUpperCase())}
              placeholder="e.g. USDC"
              className="bg-[#252840] border-0"
            />
          </div>
          <div className="space-y-2">
            <Label className="text-sm text-muted-foreground">Decimals</Label>
            <Input
              value={customTokenDecimals}
              onChange={(event) => setCustomTokenDecimals(event.target.value.replace(/[^0-9]/g, ''))}
              placeholder="18"
              className="bg-[#252840] border-0"
              inputMode="numeric"
            />
          </div>
        </div>
      )}

      <div className="space-y-2">
        <Label className="text-sm text-muted-foreground">Bozo deposit amount</Label>
        <div className="rounded-lg border border-border/40 bg-[#252840]/60 px-3 py-2 text-sm">
          {hasDepositAmount && depositAmountDisplay ? (
            <span className="text-foreground font-medium">{depositAmountDisplay}</span>
          ) : (
            <span className="text-muted-foreground">Set a deposit amount above to continue</span>
          )}
        </div>
        <p className="text-xs text-muted-foreground">
          Relay will bridge your selected asset to {destinationSymbol} and call <code>play()</code> with this amount.
        </p>
      </div>

      {quoteError && (
        <Alert className="bg-[#FF4B4B]/10 border-[#FF4B4B] text-sm text-foreground">
          <Info className="h-4 w-4 text-[#FF4B4B]" />
          <AlertDescription>{quoteError}</AlertDescription>
        </Alert>
      )}

      {quote && (
        <div className="rounded-lg border border-border/40 bg-[#252840]/60 p-4 space-y-3 text-sm">
          {estimatedInput && (
            <div className="flex items-center justify-between">
              <span className="text-muted-foreground">Estimated spend</span>
              <span className="text-foreground font-medium">{estimatedInput}</span>
            </div>
          )}
          <div className="flex items-center justify-between">
            <span className="text-muted-foreground">Estimated output</span>
            <span className="text-foreground font-medium">
              {estimatedOutput ?? `~ ${destinationSymbol}`}
            </span>
          </div>
          {minimumOutput && (
            <div className="flex items-center justify-between text-muted-foreground">
              <span>Minimum received</span>
              <span>{minimumOutput} {destinationSymbol}</span>
            </div>
          )}
          {quote.details?.timeEstimate && (
            <div className="flex items-center justify-between text-muted-foreground">
              <span>Estimated time</span>
              <span>{Math.round(quote.details.timeEstimate / 60)} min</span>
            </div>
          )}
          {progressAction && (
            <div className="flex items-center gap-2 text-[#F6C445]">
              <RefreshCcw className="h-4 w-4 animate-spin" />
              <span>{progressAction}</span>
            </div>
          )}
        </div>
      )}

      <div className="grid gap-3 sm:grid-cols-2">
        <Button
          variant="outline"
          className="w-full border-[#2ED4B7]/50 text-[#2ED4B7] hover:bg-[#2ED4B7]/10"
          onClick={handleGetQuote}
          disabled={isFetchingQuote || isExecuting || !destinationReady || !hasDepositAmount}
        >
          {isFetchingQuote ? (
            <>
              <Loader2 className="h-4 w-4 mr-2 animate-spin" /> Getting quote
            </>
          ) : (
            <>
              <ArrowRightLeft className="h-4 w-4 mr-2" /> Get Relay quote
            </>
          )}
        </Button>
        <Button
          className="w-full bg-[#F6C445] hover:bg-[#F6C445]/90 text-[#0E1020]"
          onClick={handleExecute}
          disabled={disabledExecute}
        >
          {isExecuting ? (
            <>
              <Loader2 className="h-4 w-4 mr-2 animate-spin" /> Executing on Relay…
            </>
          ) : (
            'Execute with Relay'
          )}
        </Button>
      </div>

      <p className="text-xs text-muted-foreground text-center">
        Powered by Relay. The bridge, swap, and Bozo play happen in a single flow.
      </p>
    </div>
  );
}
