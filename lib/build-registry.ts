/**
 * Build token-registry.json from CoinGecko API.
 *
 * Queries CoinGecko for top tokens by market cap, cross-references with
 * contract addresses per chain, and outputs a registry JSON file.
 */
import { writeFileSync } from "fs";
import { resolve, dirname } from "path";

const CHAIN_TARGETS: Record<string, { chain: string; target: number }> = {
  ethereum: { chain: "eth", target: 500 },
  base: { chain: "base", target: 100 },
  "binance-smart-chain": { chain: "bsc", target: 100 },
  "polygon-pos": { chain: "polygon", target: 50 },
  "arbitrum-one": { chain: "arbitrum", target: 50 },
  avalanche: { chain: "avalanche", target: 50 },
  unichain: { chain: "unichain", target: 50 },
};

const RATE_LIMIT_MS = 12000;
const MAX_RETRIES = 5;
const PAGES_TO_FETCH = 8;

function sleep(ms: number): Promise<void> {
  return new Promise((r) => setTimeout(r, ms));
}

async function fetchJSON(url: string, apiKey?: string): Promise<any> {
  const headers: Record<string, string> = {
    Accept: "application/json",
    "User-Agent": "substreams-abis/1.0 (https://github.com/pinax-network/substreams-abis)",
  };
  if (apiKey) {
    headers["x-cg-demo-api-key"] = apiKey;
  }

  const resp = await fetch(url, { headers });
  if (resp.status === 429) {
    throw new Error("RATE_LIMITED");
  }
  if (!resp.ok) {
    const text = await resp.text();
    throw new Error(`HTTP ${resp.status}: ${text.substring(0, 200)}`);
  }
  return resp.json();
}

async function fetchWithRetry(url: string, label: string, apiKey?: string): Promise<any> {
  for (let attempt = 1; attempt <= MAX_RETRIES; attempt++) {
    try {
      console.error(`  Fetching ${label}... (attempt ${attempt})`);
      return await fetchJSON(url, apiKey);
    } catch (err: any) {
      if (err.message === "RATE_LIMITED" && attempt < MAX_RETRIES) {
        const backoff = RATE_LIMIT_MS * Math.pow(2, attempt - 1);
        console.error(`  Rate limited, waiting ${backoff / 1000}s...`);
        await sleep(backoff);
      } else if (attempt < MAX_RETRIES) {
        console.error(`  Error: ${err.message}, retrying...`);
        await sleep(RATE_LIMIT_MS);
      } else {
        throw err;
      }
    }
  }
}

function sanitizeSymbol(symbol: string): string {
  return symbol
    .toUpperCase()
    .replace(/[^A-Z0-9]/g, "_")
    .replace(/_+/g, "_")
    .replace(/^_|_$/g, "")
    .substring(0, 20);
}

export interface BuildRegistryOptions {
  output?: string;
  apiKey?: string;
}

export async function buildRegistry(opts: BuildRegistryOptions = {}): Promise<void> {
  const repoRoot = resolve(dirname(new URL(import.meta.url).pathname), "..");
  const outputPath = opts.output ?? resolve(repoRoot, "scripts/token-registry.json");

  console.error("=== Building Token Registry from CoinGecko ===\n");

  // Step 1: Fetch coin rankings (market cap order)
  console.error("Step 1: Fetching coin rankings...");
  const rankedCoins: any[] = [];
  for (let page = 1; page <= PAGES_TO_FETCH; page++) {
    const url = `https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=250&page=${page}`;
    const data = await fetchWithRetry(url, `page ${page}/${PAGES_TO_FETCH}`, opts.apiKey);
    rankedCoins.push(...data);
    if (page < PAGES_TO_FETCH) await sleep(RATE_LIMIT_MS);
  }
  console.error(`  Got ${rankedCoins.length} ranked coins\n`);

  // Step 2: Fetch all coins with platform addresses
  console.error("Step 2: Fetching coin list with platform addresses...");
  await sleep(RATE_LIMIT_MS);
  const coinList = await fetchWithRetry(
    "https://api.coingecko.com/api/v3/coins/list?include_platform=true",
    "coin list with platforms",
    opts.apiKey
  );
  console.error(`  Got ${coinList.length} coins with platform data\n`);

  // Build lookup: coingecko_id -> platforms
  const platformMap = new Map<string, { symbol: string; platforms: Record<string, string> }>();
  for (const coin of coinList) {
    if (coin.platforms && Object.keys(coin.platforms).length > 0) {
      platformMap.set(coin.id, {
        symbol: coin.symbol,
        platforms: coin.platforms,
      });
    }
  }

  // Step 3: Cross-reference
  console.error("Step 3: Cross-referencing rankings with chain addresses...");
  const registry: any[] = [];
  const seenNames = new Set<string>();

  const chainOrder = [
    "ethereum",
    "base",
    "binance-smart-chain",
    "polygon-pos",
    "arbitrum-one",
    "avalanche",
    "unichain",
  ];

  for (const platform of chainOrder) {
    const config = CHAIN_TARGETS[platform];
    if (!config) continue;

    let count = 0;
    for (const ranked of rankedCoins) {
      if (count >= config.target) break;

      const coinData = platformMap.get(ranked.id);
      if (!coinData) continue;

      const address = coinData.platforms[platform];
      if (!address || address.trim() === "") continue;

      const symbol = sanitizeSymbol(coinData.symbol || ranked.symbol);
      if (!symbol) continue;

      const alreadyExists = seenNames.has(symbol);

      registry.push({
        name: symbol,
        address: address.trim(),
        chain: config.chain,
        coingecko_id: ranked.id,
        market_cap_rank: ranked.market_cap_rank,
        duplicate: alreadyExists,
      });

      if (!alreadyExists) {
        seenNames.add(symbol);
      }

      count++;
    }

    console.error(`  ${platform}: ${count} tokens (target: ${config.target})`);
  }

  // Step 4: Output
  const output = {
    generated_at: new Date().toISOString(),
    total_entries: registry.length,
    unique_names: seenNames.size,
    chains: Object.fromEntries(
      chainOrder.map((p) => [
        CHAIN_TARGETS[p].chain,
        registry.filter((t: any) => t.chain === CHAIN_TARGETS[p].chain).length,
      ])
    ),
    tokens: registry,
  };

  writeFileSync(outputPath, JSON.stringify(output, null, 2) + "\n");
  console.error(`\nWrote ${outputPath}`);
  console.error(`  Total entries: ${registry.length}, Unique names: ${seenNames.size}`);
  console.error(
    `  Duplicates (same name, different chain): ${registry.filter((t: any) => t.duplicate).length}`
  );
}
