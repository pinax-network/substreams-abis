/**
 * Chain configuration for the Etherscan V2 API.
 *
 * V2 uses a single endpoint (https://api.etherscan.io/v2/api) with a
 * `chainid` parameter instead of separate per-chain base URLs.
 */

export const API_BASE = "https://api.etherscan.io/v2/api";

export interface ChainConfig {
  chainId: number;
  explorer: string;
}

export const SUPPORTED_CHAINS: Record<string, ChainConfig> = {
  eth:       { chainId: 1,     explorer: "etherscan.io" },
  ethereum:  { chainId: 1,     explorer: "etherscan.io" },
  mainnet:   { chainId: 1,     explorer: "etherscan.io" },
  base:      { chainId: 8453,  explorer: "basescan.org" },
  arbitrum:  { chainId: 42161, explorer: "arbiscan.io" },
  arb:       { chainId: 42161, explorer: "arbiscan.io" },
  optimism:  { chainId: 10,    explorer: "optimistic.etherscan.io" },
  op:        { chainId: 10,    explorer: "optimistic.etherscan.io" },
  polygon:   { chainId: 137,   explorer: "polygonscan.com" },
  matic:     { chainId: 137,   explorer: "polygonscan.com" },
  bsc:       { chainId: 56,    explorer: "bscscan.com" },
  bnb:       { chainId: 56,    explorer: "bscscan.com" },
  avalanche: { chainId: 43114, explorer: "snowtrace.io" },
  avax:      { chainId: 43114, explorer: "snowtrace.io" },
  unichain:  { chainId: 130,   explorer: "unichain.blockscout.com" },
};

export function getChainConfig(chain: string): ChainConfig {
  const config = SUPPORTED_CHAINS[chain];
  if (!config) {
    throw new Error(
      `Unknown chain '${chain}'. Supported: ${Object.keys(SUPPORTED_CHAINS).join(", ")}`
    );
  }
  return config;
}
