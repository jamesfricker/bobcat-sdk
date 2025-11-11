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
    bozo: (env.VITE_BOZO_CONTRACT as `0x${string}`) || '0x0b8f1939481a337488aae1146063ecacd03462a1',
  },

  relay: {
    apiUrl: env.VITE_RELAY_API_URL || 'https://api.relay.link',
    source: env.VITE_RELAY_SOURCE || 'ripbozo.lol',
    logLevel: env.VITE_RELAY_LOG_LEVEL || 'error',
  },

  // Testing flags
  testEndGameScreen: false // Set to true to preview the end game screen
};
