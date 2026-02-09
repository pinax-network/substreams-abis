#!/usr/bin/env node
//
// Build token-registry.json from CoinGecko API.
//
// Queries CoinGecko for top tokens by market cap, cross-references with
// contract addresses per chain, and outputs a registry JSON file.
//
// Usage:
//   node scripts/build-registry.js
//   node scripts/build-registry.js --output custom-path.json
//
// Environment:
//   COINGECKO_API_KEY  - Optional CoinGecko Pro API key for higher rate limits
//
const https = require("https");
const fs = require("fs");
const path = require("path");

// --- Configuration ---

const CHAIN_TARGETS = {
  ethereum: { chain: "eth", target: 500 },
  base: { chain: "base", target: 100 },
  "binance-smart-chain": { chain: "bsc", target: 100 },
  "polygon-pos": { chain: "polygon", target: 50 },
  "arbitrum-one": { chain: "arbitrum", target: 50 },
  avalanche: { chain: "avalanche", target: 50 },
  unichain: { chain: "unichain", target: 50 },
};

const RATE_LIMIT_MS = 12000; // 12s between calls (CoinGecko free tier: ~5 calls/min)
const MAX_RETRIES = 5;
const PAGES_TO_FETCH = 8; // 8 pages x 250 = 2000 coins

// --- Helpers ---

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function fetchJSON(url) {
  return new Promise((resolve, reject) => {
    const headers = {
      Accept: "application/json",
      "User-Agent": "substreams-abis/1.0 (https://github.com/pinax-network/substreams-abis)",
    };
    if (process.env.COINGECKO_API_KEY) {
      headers["x-cg-demo-api-key"] = process.env.COINGECKO_API_KEY;
    }
    https
      .get(url, { headers }, (res) => {
        let data = "";
        res.on("data", (chunk) => (data += chunk));
        res.on("end", () => {
          if (res.statusCode === 429) {
            reject(new Error("RATE_LIMITED"));
          } else if (res.statusCode !== 200) {
            reject(
              new Error(`HTTP ${res.statusCode}: ${data.substring(0, 200)}`)
            );
          } else {
            try {
              resolve(JSON.parse(data));
            } catch (e) {
              reject(new Error(`JSON parse error: ${e.message}`));
            }
          }
        });
        res.on("error", reject);
      })
      .on("error", reject);
  });
}

async function fetchWithRetry(url, label) {
  for (let attempt = 1; attempt <= MAX_RETRIES; attempt++) {
    try {
      console.error(`  Fetching ${label}... (attempt ${attempt})`);
      const result = await fetchJSON(url);
      return result;
    } catch (err) {
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

function sanitizeSymbol(symbol) {
  return symbol
    .toUpperCase()
    .replace(/[^A-Z0-9]/g, "_")
    .replace(/_+/g, "_")
    .replace(/^_|_$/g, "")
    .substring(0, 20);
}

// --- Main ---

async function main() {
  const args = process.argv.slice(2);
  let outputPath = path.join(__dirname, "token-registry.json");
  for (let i = 0; i < args.length; i++) {
    if (args[i] === "--output" && args[i + 1]) {
      outputPath = args[i + 1];
      i++;
    }
  }

  console.error("=== Building Token Registry from CoinGecko ===\n");

  // Step 1: Fetch coin rankings (market cap order)
  console.error("Step 1: Fetching coin rankings...");
  const rankedCoins = [];
  for (let page = 1; page <= PAGES_TO_FETCH; page++) {
    const url = `https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=250&page=${page}`;
    const data = await fetchWithRetry(url, `page ${page}/${PAGES_TO_FETCH}`);
    rankedCoins.push(...data);
    if (page < PAGES_TO_FETCH) await sleep(RATE_LIMIT_MS);
  }
  console.error(`  Got ${rankedCoins.length} ranked coins\n`);

  // Step 2: Fetch all coins with platform addresses
  console.error("Step 2: Fetching coin list with platform addresses...");
  await sleep(RATE_LIMIT_MS);
  const coinList = await fetchWithRetry(
    "https://api.coingecko.com/api/v3/coins/list?include_platform=true",
    "coin list with platforms"
  );
  console.error(`  Got ${coinList.length} coins with platform data\n`);

  // Build lookup: coingecko_id -> platforms
  const platformMap = new Map();
  for (const coin of coinList) {
    if (coin.platforms && Object.keys(coin.platforms).length > 0) {
      platformMap.set(coin.id, {
        symbol: coin.symbol,
        platforms: coin.platforms,
      });
    }
  }

  // Step 3: Cross-reference - for each chain, collect tokens by rank
  console.error("Step 3: Cross-referencing rankings with chain addresses...");
  const registry = [];
  const seenNames = new Set(); // track names to avoid duplicates across chains

  // Process chains in priority order
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

      // Use symbol + chain as unique key to allow same token on different chains
      // but deduplicate exact same name (first chain wins for the ABI file)
      const nameKey = symbol;
      const alreadyExists = seenNames.has(nameKey);

      registry.push({
        name: symbol,
        address: address.trim(),
        chain: config.chain,
        coingecko_id: ranked.id,
        market_cap_rank: ranked.market_cap_rank,
        duplicate: alreadyExists,
      });

      if (!alreadyExists) {
        seenNames.add(nameKey);
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
        registry.filter((t) => t.chain === CHAIN_TARGETS[p].chain).length,
      ])
    ),
    tokens: registry,
  };

  fs.writeFileSync(outputPath, JSON.stringify(output, null, 2) + "\n");
  console.error(`\nWrote ${outputPath}`);
  console.error(
    `  Total entries: ${registry.length}, Unique names: ${seenNames.size}`
  );
  console.error(
    `  Duplicates (same name, different chain): ${registry.filter((t) => t.duplicate).length}`
  );
}

main().catch((err) => {
  console.error(`\nFatal error: ${err.message}`);
  process.exit(1);
});
