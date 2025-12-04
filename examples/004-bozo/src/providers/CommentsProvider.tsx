import { createContext, useCallback, useContext, useEffect, useMemo, useState, type ReactNode } from 'react';
import { usePublicClient } from 'wagmi';
import { config } from '../lib/config';
import { DEPOSIT_LOOKBACK_BLOCKS, depositEventAbi } from '../lib/depositEvents';
import { fetchComments } from '../lib/commentsApi';
import { bozoAbi } from '../lib/bozoAbi';
import type { BozoComment } from '../types';

type CommentsContextValue = {
  comments: BozoComment[];
  loading: boolean;
  error: Error | null;
  refresh: () => Promise<BozoComment[]>;
  getCommentForTxHash: (txHash?: string | null) => string | undefined;
  getCommentForWallet: (wallet?: string | null) => string | undefined;
};

const CommentsContext = createContext<CommentsContextValue | undefined>(undefined);

const COMMENTS_FROM = 0;
const COMMENTS_LIMIT = 50;

export function CommentsProvider({ children }: { children: ReactNode }) {
  const publicClient = usePublicClient();
  const [comments, setComments] = useState<BozoComment[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<Error | null>(null);

  const loadComments = useCallback(
    async (signal?: AbortSignal) => {
      const [graphComments, eventMetadata] = await Promise.all([
        (async () => {
          if (!publicClient) {
            return [] as BozoComment[];
          }

          try {
            const epochResult = await publicClient.readContract({
              address: config.contracts.bozo as `0x${string}`,
              abi: bozoAbi,
              functionName: 'currentEpoch',
            });

            const epochNumber =
              typeof epochResult === 'bigint' ? Number(epochResult) : null;
            if (epochNumber === null || !Number.isSafeInteger(epochNumber)) {
              return [] as BozoComment[];
            }

            return fetchComments({
              epoch: epochNumber,
              from: COMMENTS_FROM,
              limit: COMMENTS_LIMIT,
              signal,
            });
          } catch (err) {
            console.error('Failed to load comments from GraphQL:', err);
            return [] as BozoComment[];
          }
        })(),
        (async () => {
          if (!publicClient) {
            return {
              txHashes: new Set<string>(),
              txToWallet: new Map<string, string>(),
            };
          }

          try {
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

            const hashes = new Set<string>();
            const txToWallet = new Map<string, string>();
            for (const event of events) {
              if (event.transactionHash) {
                const txHash = event.transactionHash.toLowerCase();
                hashes.add(txHash);
                const recipient = event.args?.recipient as string | undefined;
                if (recipient) {
                  txToWallet.set(txHash, recipient);
                }
              }
            }
            return { txHashes: hashes, txToWallet };
          } catch (err) {
            console.error('Failed to load deposit events for comments reconciliation:', err);
            return {
              txHashes: new Set<string>(),
              txToWallet: new Map<string, string>(),
            };
          }
        })(),
      ]);

      const { txHashes, txToWallet } = eventMetadata;
      const commentsWithWallet = graphComments.map((comment) => {
        const txHash = comment.txHash.toLowerCase();
        return {
          ...comment,
          wallet: comment.wallet || txToWallet.get(txHash),
        };
      });

      if (txHashes.size === 0) {
        return commentsWithWallet;
      }

      return commentsWithWallet.filter((comment) => txHashes.has(comment.txHash.toLowerCase()));
    },
    [publicClient]
  );

  const refresh = useCallback(async () => {
    setLoading(true);
    try {
      const data = await loadComments();
      setComments(data);
      setError(null);
      return data;
    } catch (err) {
      const errorInstance = err instanceof Error ? err : new Error('Failed to refresh comments');
      setError(errorInstance);
      throw errorInstance;
    } finally {
      setLoading(false);
    }
  }, [loadComments]);

  useEffect(() => {
    const controller = new AbortController();

    const load = async () => {
      setLoading(true);
      try {
        const data = await loadComments(controller.signal);
        setComments(data);
        setError(null);
      } catch (err) {
        if ((err as DOMException)?.name === 'AbortError') {
          return;
        }
        setError(err instanceof Error ? err : new Error('Failed to fetch comments'));
      } finally {
        setLoading(false);
      }
    };

    load();

    return () => {
      controller.abort();
    };
  }, [loadComments]);

  const commentsByTxHash = useMemo(() => {
    const map = new Map<string, string>();
    for (const comment of comments) {
      map.set(comment.txHash.toLowerCase(), comment.content);
    }
    return map;
  }, [comments]);

  const commentsByWallet = useMemo(() => {
    const map = new Map<string, string>();
    for (const comment of comments) {
      if (comment.wallet) {
        map.set(comment.wallet.toLowerCase(), comment.content);
      }
    }
    return map;
  }, [comments]);

  const getCommentForTxHash = useCallback(
    (txHash?: string | null) => {
      if (!txHash) {
        return undefined;
      }
      return commentsByTxHash.get(txHash.toLowerCase());
    },
    [commentsByTxHash]
  );

  const getCommentForWallet = useCallback(
    (wallet?: string | null) => {
      if (!wallet) {
        return undefined;
      }
      return commentsByWallet.get(wallet.toLowerCase());
    },
    [commentsByWallet]
  );

  const value = useMemo<CommentsContextValue>(
    () => ({ comments, loading, error, refresh, getCommentForTxHash, getCommentForWallet }),
    [comments, loading, error, refresh, getCommentForTxHash, getCommentForWallet]
  );

  return <CommentsContext.Provider value={value}>{children}</CommentsContext.Provider>;
}

export function useComments() {
  const context = useContext(CommentsContext);
  if (!context) {
    throw new Error('useComments must be used within a CommentsProvider');
  }
  return context;
}
