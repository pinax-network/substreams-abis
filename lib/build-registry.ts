/**
 * Build token-registry.json from the CoinGecko Pro API.
 *
 * Fetches full coin list with platform addresses via /coins/list and keeps
 * only coins that have addresses on supported chains.
 */
import { writeFileSync } from "fs";
import { resolve, dirname } from "path";

/** CoinGecko platform ID → short chain name used in the registry. */
const CHAINS: Record<string, string> = {
  ethereum: "eth",
  base: "base",
  "binance-smart-chain": "bsc",
  "polygon-pos": "polygon",
  "arbitrum-one": "arbitrum",
  avalanche: "avalanche",
  unichain: "unichain",
  "optimistic-ethereum": "optimism",
  solana: "solana",
  tron: "tron",
};

const MAX_RETRIES = 10;
const RETRY_DELAY_MS = 10000;

function sleep(ms: number): Promise<void> {
  return new Promise((r) => setTimeout(r, ms));
}

async function fetchJSON(url: string, apiKey?: string): Promise<any> {
  const headers: Record<string, string> = {
    Accept: "application/json",
    "User-Agent": "substreams-abis/1.0 (https://github.com/pinax-network/substreams-abis)",
  };
  if (apiKey) {
    headers["x-cg-pro-api-key"] = apiKey;
  } else {
    console.error("No CoinGecko API key provided");
    process.exit(1);
  }

  for (let attempt = 1; attempt <= MAX_RETRIES; attempt++) {
    const resp = await fetch(url, { headers });

    if (resp.status === 429 && attempt < MAX_RETRIES) {
      const backoff = RETRY_DELAY_MS * attempt;
      console.error(`  Rate limited, waiting ${backoff / 1000}s... (attempt ${attempt}/${MAX_RETRIES})`);
      await sleep(backoff);
      continue;
    }
    if (!resp.ok) {
      const text = await resp.text();
      if (attempt < MAX_RETRIES) {
        console.error(`  HTTP ${resp.status}, retrying... (attempt ${attempt}/${MAX_RETRIES})`);
        await sleep(RETRY_DELAY_MS);
        continue;
      }
      throw new Error(`HTTP ${resp.status}: ${text.substring(0, 200)}`);
    }
    return resp.json();
  }
}

export interface BuildRegistryOptions {
  output?: string;
  apiKey?: string;
}

export async function buildRegistry(opts: BuildRegistryOptions = {}): Promise<void> {
  const repoRoot = resolve(dirname(new URL(import.meta.url).pathname), "..");
  const outputPath = opts.output ?? resolve(repoRoot, "scripts/token-registry.json");

  console.error("=== Building Token Registry from CoinGecko ===\n");

  // --- Fetch coin list with platform addresses ---
  console.error("Fetching coin list with platform addresses...");
  const coinList: Array<{
    id: string;
    symbol: string;
    name: string;
    platforms?: Record<string, string>;
  }> = await fetchJSON(
    "https://api.coingecko.com/api/v3/coins/list?include_platform=true",
    opts.apiKey
  );
  console.error(`  Got ${coinList.length} coins\n`);

  // Filter to coins with addresses on supported chains
  interface RegistryEntry {
    id: string;
    symbol: string;
    name: string;
    platforms: Record<string, string>;
  }
  const registry: RegistryEntry[] = [];

  for (const coin of coinList) {
    if (!coin.platforms) continue;
    const filtered: Record<string, string> = {};
    for (const [platform, address] of Object.entries(coin.platforms)) {
      if (!CHAINS[platform] || !address || address.trim() === "") continue;
      filtered[CHAINS[platform]] = address.trim();
    }
    if (Object.keys(filtered).length > 0) {
      registry.push({
        id: coin.id,
        symbol: coin.symbol,
        name: coin.name,
        platforms: filtered,
      });
    }
  }

  // Per-chain counts for summary
  const chainCounts: Record<string, number> = {};
  for (const chain of Object.values(CHAINS)) chainCounts[chain] = 0;
  for (const entry of registry) {
    for (const chain of Object.keys(entry.platforms)) {
      chainCounts[chain] = (chainCounts[chain] ?? 0) + 1;
    }
  }
  for (const [chain, count] of Object.entries(chainCounts)) {
    console.error(`  ${chain}: ${count} tokens`);
  }

  // --- Output ---
  const output = {
    generated_at: new Date().toISOString(),
    total: registry.length,
    chains: chainCounts,
    tokens: registry,
  };

  writeFileSync(outputPath, JSON.stringify(output, null, 2) + "\n");
  console.error(`\nWrote ${outputPath}`);
  console.error(`  Total tokens: ${registry.length}`);
}
