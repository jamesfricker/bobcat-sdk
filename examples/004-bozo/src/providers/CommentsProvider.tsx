import { createContext, useCallback, useContext, useEffect, useMemo, useState, type ReactNode } from 'react';
import { config } from '../lib/config';
import type { BozoComment } from '../types';

type CommentsContextValue = {
  comments: BozoComment[];
  loading: boolean;
  error: Error | null;
  refresh: () => Promise<BozoComment[]>;
  getCommentForWallet: (wallet?: string | null) => string | undefined;
};

const CommentsContext = createContext<CommentsContextValue | undefined>(undefined);

const COMMENTS_QUERY = `
  query Comments {
    comments {
      wallet
      content
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
      entry && typeof entry.wallet === 'string' && typeof entry.content === 'string'
    )
    .map((entry) => ({
      wallet: entry.wallet,
      content: entry.content,
    }));
}

export function CommentsProvider({ children }: { children: ReactNode }) {
  const [comments, setComments] = useState<BozoComment[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<Error | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    try {
      const data = await performFetch();
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
  }, []);

  useEffect(() => {
    const controller = new AbortController();

    const load = async () => {
      setLoading(true);
      try {
        const data = await performFetch(controller.signal);
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
  }, []);

  const commentsByWallet = useMemo(() => {
    const map = new Map<string, string>();
    for (const comment of comments) {
      map.set(comment.wallet.toLowerCase(), comment.content);
    }
    return map;
  }, [comments]);

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
    () => ({ comments, loading, error, refresh, getCommentForWallet }),
    [comments, loading, error, refresh, getCommentForWallet]
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
