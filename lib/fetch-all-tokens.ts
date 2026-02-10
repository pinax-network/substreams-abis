/**
 * Batch-fetch ABIs and Solidity source for all tokens in the registry.
 */
import { existsSync, readFileSync } from "fs";
import { resolve, dirname } from "path";
import { fetchToken } from "./fetch-token";

const RATE_LIMIT_SLEEP = 500; // ms between tokens

function sleep(ms: number): Promise<void> {
  return new Promise((r) => setTimeout(r, ms));
}

interface TokenEntry {
  name: string;
  address: string;
  chain: string;
  duplicate?: boolean;
  market_cap_rank?: number;
}

interface TokenRegistry {
  tokens: TokenEntry[];
}

// Built-in fallback registry
const BUILTIN_TOKENS: TokenEntry[] = [
  // --- ETH Mainnet ---
  { name: "SHIB", address: "0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE", chain: "eth" },
  { name: "LINK", address: "0x514910771AF9Ca656af840dff83E8264EcF986CA", chain: "eth" },
  { name: "UNI", address: "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984", chain: "eth" },
  { name: "MATIC", address: "0x7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0", chain: "eth" },
  { name: "ARB", address: "0xB50721BCf8d664c30412Cfbc6cf7a15145234ad1", chain: "eth" },
  { name: "LDO", address: "0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32", chain: "eth" },
  { name: "AAVE", address: "0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9", chain: "eth" },
  { name: "MKR", address: "0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2", chain: "eth" },
  { name: "CRV", address: "0xD533a949740bb3306d119CC777fa900bA034cd52", chain: "eth" },
  { name: "PEPE", address: "0x6982508145454Ce325dDbE47a25d4ec3d2311933", chain: "eth" },
  { name: "APE", address: "0x4d224452801ACEd8B2F0aebE155379bb5D594381", chain: "eth" },
  { name: "SNX", address: "0xC011a73ee8576Fb46F5E1c5751cA3B9Fe0af2a6F", chain: "eth" },
  { name: "COMP", address: "0xc00e94Cb662C3520282E6f5717214004A7f26888", chain: "eth" },
  { name: "GRT", address: "0xc944E90C64B2c07662A292be6244BDf05Cda44a7", chain: "eth" },
  { name: "FET", address: "0xaea46A60368A7bD060eec7DF8CBa43b7EF41Ad85", chain: "eth" },
  { name: "DAI", address: "0x6B175474E89094C44Da98b954EedeAC495271d0F", chain: "eth" },
  { name: "WETH", address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", chain: "eth" },
  { name: "WBTC", address: "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599", chain: "eth" },
  { name: "stETH", address: "0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84", chain: "eth" },
  { name: "RPL", address: "0xD33526068D116cE69F19A9ee46F0bd304F21A51f", chain: "eth" },
  { name: "ENS", address: "0xC18360217D8F7Ab5e7c516566761Ea12Ce7F9D72", chain: "eth" },
  { name: "LRC", address: "0xBBbbCA6A901c926F240b89EacB641d8Aec7AEafD", chain: "eth" },
  { name: "BAT", address: "0x0D8775F648430679A709E98d2b0Cb6250d2887EF", chain: "eth" },
  { name: "1INCH", address: "0x111111111117dC0aa78b770fA6A738034120C302", chain: "eth" },
  { name: "SUSHI", address: "0x6B3595068778DD592e39A122f4f5a5cF09C90fE2", chain: "eth" },
  { name: "YFI", address: "0x0bc529c00C6401aEF6D220BE8C6Ea1667F6Ad93e", chain: "eth" },
  { name: "BLUR", address: "0x5283D291DBCF85356A21bA090E6db59121208b44", chain: "eth" },
  { name: "IMX", address: "0xF57e7e7C23978C3cAEC3C3548E3D615c346e79fF", chain: "eth" },
  { name: "RNDR", address: "0x6De037ef9aD2725EB40118Bb1702EBb27e4Aeb24", chain: "eth" },
  { name: "FXS", address: "0x3432B6A60D23Ca0dFCa7761B7ab56459D9C964D0", chain: "eth" },
  { name: "PENDLE", address: "0x808507121B80c02388fAd14726482e061B8da827", chain: "eth" },
  { name: "WLD", address: "0x163f8C2467924be0ae7B5347228CABF260318753", chain: "eth" },
  { name: "ENA", address: "0x57e114B691Db790C35207b2e685D4A43181e6061", chain: "eth" },
  { name: "EIGEN", address: "0xec53bF9167f50cDEB3Ae105f56099aaaB9061F83", chain: "eth" },
  // --- Base ---
  { name: "AERO", address: "0x940181a94A35A4569E4529A3CDfB74e38FD98631", chain: "base" },
  { name: "BRETT", address: "0x532f27101965dd16442E59d40670FaF5eBB142E4", chain: "base" },
  { name: "DEGEN", address: "0x4ed4E862860beD51a9570b96d89aF5E1B0Efefed", chain: "base" },
  { name: "TOSHI", address: "0xAC1Bd2486aAf3B5C0fc3Fd868558b082a531B2B4", chain: "base" },
  { name: "WELL", address: "0xA88594D404727625A9437C3f886C7643872296AE", chain: "base" },
  { name: "MORPHO", address: "0xBAa5CC21fd487B8Fcc2F632f3F4E8D37262a0842", chain: "base" },
  { name: "VIRTUAL", address: "0x0b3e328455c4059EEb9e3f84b5543F74E24e7E1b", chain: "base" },
  { name: "cbBTC", address: "0xcbB7C0000aB88B473b1f5aFd9ef808440eed33Bf", chain: "base" },
  { name: "USDbC", address: "0xd9aAEc86B65D86f6A7B5B1b0c42FFA531710b6CA", chain: "base" },
  { name: "HIGHER", address: "0x0578d8A44db98B23BF096A382e016e29a5Ce0ffe", chain: "base" },
];

export interface FetchAllOptions {
  chain?: string;
  missing?: boolean;
  noDuplicates?: boolean;
  registry?: string;
  apiKey?: string;
  abiDir?: string;
}

export interface FetchAllResult {
  total: number;
  fetched: number;
  skipped: number;
  failed: number;
}

export async function fetchAllTokens(opts: FetchAllOptions = {}): Promise<FetchAllResult> {
  const repoRoot = resolve(dirname(new URL(import.meta.url).pathname), "..");
  const abiDir = opts.abiDir ?? resolve(repoRoot, "abi/erc20-tokens");
  const registryFile = opts.registry ?? resolve(repoRoot, "scripts/token-registry.json");

  let tokens: TokenEntry[] = [];

  // Try JSON registry first
  if (existsSync(registryFile)) {
    try {
      const data = JSON.parse(readFileSync(registryFile, "utf-8")) as TokenRegistry;
      if (data.tokens) {
        tokens = data.tokens;
        console.log(`Using registry: ${registryFile}`);
      }
    } catch {
      console.log(`Warning: Failed to parse ${registryFile}`);
    }
  }

  // Fall back to built-in registry
  if (tokens.length === 0) {
    console.log("No registry file found. Using built-in fallback registry.");
    console.log("Run 'bun run cli.ts build-registry' to generate a full registry.\n");
    tokens = BUILTIN_TOKENS;
  }

  // Apply filters
  if (opts.chain) {
    tokens = tokens.filter((t) => t.chain === opts.chain);
  }
  if (opts.noDuplicates) {
    tokens = tokens.filter((t) => !t.duplicate);
  }

  // Sort by market cap rank if available
  tokens.sort((a, b) => (a.market_cap_rank ?? 9999) - (b.market_cap_rank ?? 9999));

  const totalTokens = tokens.length;

  console.log("================================================");
  console.log("  Token ABI Batch Fetcher");
  console.log(`  Registry: ${totalTokens} tokens`);
  if (opts.chain) console.log(`  Chain filter: ${opts.chain}`);
  if (opts.missing) console.log("  Mode: missing only");
  if (opts.noDuplicates) console.log("  Skipping cross-chain duplicates");
  console.log("================================================\n");

  const result: FetchAllResult = { total: 0, fetched: 0, skipped: 0, failed: 0 };

  for (const token of tokens) {
    result.total++;

    // Skip if already downloaded and --missing flag is set
    if (opts.missing && existsSync(resolve(abiDir, `${token.name}.json`))) {
      console.log(`SKIP: ${token.name} (already exists)`);
      result.skipped++;
      continue;
    }

    console.log(`\n[${result.total}/${totalTokens}] Fetching ${token.name} (${token.chain})...`);
    try {
      const fetchResult = await fetchToken({
        tokenName: token.name,
        address: token.address,
        chain: token.chain,
        apiKey: opts.apiKey,
        abiDir,
      });
      if (fetchResult.abiSaved) {
        result.fetched++;
      } else {
        result.failed++;
      }
    } catch (err: any) {
      console.log(`  FAILED: ${token.name} - ${err.message}`);
      result.failed++;
    }

    // Rate limit between tokens
    await sleep(RATE_LIMIT_SLEEP);
  }

  console.log("\n================================================");
  console.log("  Results");
  console.log(`  Total:   ${result.total}`);
  console.log(`  Fetched: ${result.fetched}`);
  console.log(`  Skipped: ${result.skipped}`);
  console.log(`  Failed:  ${result.failed}`);
  console.log("================================================");

  return result;
}
