import { createContext, useCallback, useContext, useEffect, useMemo, useState, type ReactNode } from 'react';
import { usePublicClient } from 'wagmi';
import { config } from '../lib/config';
import { DEPOSIT_LOOKBACK_BLOCKS, depositEventAbi } from '../lib/depositEvents';
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

const COMMENTS_QUERY = `
  query Comments {
    comments {
      wallet
      content
      txHash
    }
  }
`;

async function performFetch(signal?: AbortSignal): Promise<BozoComment[]> {
  const response = await fetch(config.graphqlUrl, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ query: COMMENTS_QUERY }),
    signal,
  });

  if (!response.ok) {
    throw new Error(`Failed to fetch comments: ${response.status} ${response.statusText}`);
  }

  const payload = await response.json();

  if (payload?.errors?.length) {
    const firstMessage = payload.errors[0]?.message || 'GraphQL error fetching comments';
    throw new Error(firstMessage);
  }

  const commentData = payload?.data?.comments;
  if (!Array.isArray(commentData)) {
    return [];
  }

  return commentData
    .filter((entry: any): entry is BozoComment =>
      entry &&
      typeof entry.wallet === 'string' &&
      typeof entry.content === 'string' &&
      typeof entry.txHash === 'string'
    )
    .map((entry) => ({
      wallet: entry.wallet,
      content: entry.content,
      txHash: entry.txHash,
    }));
}

export function CommentsProvider({ children }: { children: ReactNode }) {
  const publicClient = usePublicClient();
  const [comments, setComments] = useState<BozoComment[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<Error | null>(null);

  const loadComments = useCallback(
    async (signal?: AbortSignal) => {
      const [graphComments, txHashes] = await Promise.all([
        performFetch(signal),
        (async () => {
          if (!publicClient) {
            return new Set<string>();
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
            for (const event of events) {
              if (event.transactionHash) {
                hashes.add(event.transactionHash.toLowerCase());
              }
            }
            return hashes;
          } catch (err) {
            console.error('Failed to load deposit events for comments reconciliation:', err);
            return new Set<string>();
          }
        })(),
      ]);

      if (txHashes.size === 0) {
        return graphComments;
      }

      return graphComments.filter((comment) => txHashes.has(comment.txHash.toLowerCase()));
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
      map.set(comment.wallet.toLowerCase(), comment.content);
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
