/**
 * Chain configuration for block explorer APIs.
 */

export interface ChainConfig {
  apiBase: string;
  explorer: string;
}

export const SUPPORTED_CHAINS: Record<string, ChainConfig> = {
  eth: {
    apiBase: "https://api.etherscan.io/api",
    explorer: "etherscan.io",
  },
  ethereum: {
    apiBase: "https://api.etherscan.io/api",
    explorer: "etherscan.io",
  },
  mainnet: {
    apiBase: "https://api.etherscan.io/api",
    explorer: "etherscan.io",
  },
  base: {
    apiBase: "https://api.basescan.org/api",
    explorer: "basescan.org",
  },
  arbitrum: {
    apiBase: "https://api.arbiscan.io/api",
    explorer: "arbiscan.io",
  },
  arb: {
    apiBase: "https://api.arbiscan.io/api",
    explorer: "arbiscan.io",
  },
  optimism: {
    apiBase: "https://api-optimistic.etherscan.io/api",
    explorer: "optimistic.etherscan.io",
  },
  op: {
    apiBase: "https://api-optimistic.etherscan.io/api",
    explorer: "optimistic.etherscan.io",
  },
  polygon: {
    apiBase: "https://api.polygonscan.com/api",
    explorer: "polygonscan.com",
  },
  matic: {
    apiBase: "https://api.polygonscan.com/api",
    explorer: "polygonscan.com",
  },
  bsc: {
    apiBase: "https://api.bscscan.com/api",
    explorer: "bscscan.com",
  },
  bnb: {
    apiBase: "https://api.bscscan.com/api",
    explorer: "bscscan.com",
  },
  avalanche: {
    apiBase: "https://api.routescan.io/v2/network/mainnet/evm/43114/etherscan/api",
    explorer: "routescan.io (avalanche)",
  },
  avax: {
    apiBase: "https://api.routescan.io/v2/network/mainnet/evm/43114/etherscan/api",
    explorer: "routescan.io (avalanche)",
  },
  unichain: {
    apiBase: "https://unichain.blockscout.com/api",
    explorer: "unichain.blockscout.com",
  },
};

export function getChainConfig(chain: string): ChainConfig {
  const config = SUPPORTED_CHAINS[chain];
  if (!config) {
    const supported = [...new Set(Object.values(SUPPORTED_CHAINS).map((c) => c.explorer))];
    throw new Error(
      `Unknown chain '${chain}'. Supported: ${Object.keys(SUPPORTED_CHAINS).join(", ")}`
    );
  }
  return config;
}
