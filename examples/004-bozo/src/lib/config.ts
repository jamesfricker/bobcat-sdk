// Environment configuration

const env = typeof import.meta !== 'undefined' && import.meta.env ? import.meta.env : {};

export const config = {
  // Home chain (Arbitrum only)
  homeChain: 'arbitrum' as const,

  // API URL
  apiUrl: env.VITE_API_URL || 'http://localhost:3001/api',

  // GraphQL endpoint for Bozo data
  graphqlUrl: env.VITE_GRAPHQL_URL || 'http://localhost:8080/query',

  // Farcaster Hub
  fcHub: env.VITE_FC_HUB || '',

  // Image base URL
  imgBase: env.VITE_IMG_BASE,

  // Chain IDs
  chainIds: {
    arbitrum: 42161,
  },

  // RPC URLs (for development)
  rpcUrls: {
    arbitrum: 'https://arb1.arbitrum.io/rpc',
  },

  contracts: {
    bozo: (env.VITE_BOZO_CONTRACT as `0x${string}`) || '0x3421264e413489b1e69ae84ace8c33c6cb7809ff',
  },

  // Testing flags
  testEndGameScreen: false // Set to true to preview the end game screen
};
