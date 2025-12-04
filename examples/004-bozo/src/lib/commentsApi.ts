import { config } from './config';
import type { BozoComment } from '../types';

const COMMENTS_QUERY = `
  query Comments($epoch: Int!, $from: Int!, $limit: Int!) {
    comments(epoch: $epoch, from: $from, limit: $limit) {
      content
      txHash
    }
  }
`;

const POST_COMMENT_MUTATION = `
  mutation PostComment($epoch: Int!, $content: String!, $txHash: String!) {
    postComment(epoch: $epoch, content: $content, transactionHash: $txHash)
  }
`;

type FetchCommentsArgs = {
  epoch: number;
  from?: number;
  limit?: number;
  signal?: AbortSignal;
};

export async function fetchComments({
  epoch,
  from = 0,
  limit = 50,
  signal,
}: FetchCommentsArgs): Promise<BozoComment[]> {
  const safeLimit = Math.max(1, Math.min(limit, 50));

  const response = await fetch(config.graphqlUrl, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      query: COMMENTS_QUERY,
      variables: { epoch, from, limit: safeLimit },
    }),
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
    .filter(
      (entry: any): entry is BozoComment =>
        entry &&
        typeof entry.content === 'string' &&
        typeof entry.txHash === 'string',
    )
    .map((entry) => ({
      content: entry.content,
      txHash: entry.txHash,
      wallet: typeof entry.wallet === 'string' ? entry.wallet : undefined,
    }));
}

export async function postComment(args: {
  epoch: number;
  content: string;
  txHash: string;
}): Promise<boolean> {
  const response = await fetch(config.graphqlUrl, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      query: POST_COMMENT_MUTATION,
      variables: {
        epoch: args.epoch,
        content: args.content,
        txHash: args.txHash,
      },
    }),
  });

  if (!response.ok) {
    throw new Error(`Failed to post comment: ${response.status} ${response.statusText}`);
  }

  const payload = await response.json();
  if (payload?.errors?.length) {
    const firstMessage = payload.errors[0]?.message || 'GraphQL error posting comment';
    throw new Error(firstMessage);
  }

  const result = payload?.data?.postComment;
  return Boolean(result);
}
