#!/usr/bin/env bun
/**
 * substreams-abis CLI
 *
 * Manages ABI bindings for EVM smart contracts used in Substreams projects.
 */
import "dotenv/config";
import { Command } from "commander";
import { fetchToken } from "./lib/fetch-token";
import { fetchAllTokens } from "./lib/fetch-all-tokens";
import { updateModules } from "./lib/update-modules";
import { filterStandard } from "./lib/filter-standard";
import { findKeccakMatch, computeKeccak } from "./lib/keccak";
import { buildRegistry } from "./lib/build-registry";

const program = new Command();

program
  .name("substreams-abis")
  .description("CLI tool for managing Substreams ABI bindings")
  .version("0.9.0");

// --- fetch-token ---
program
  .command("fetch-token")
  .description("Fetch ABI and Solidity source for a token contract from a block explorer")
  .argument("<token-name>", "Token symbol in CAPS (e.g. SHIB, LINK, UNI)")
  .argument("<address>", "The 0x... contract address")
  .option("-c, --chain <chain>", "Chain to fetch from", process.env.CHAIN ?? "eth")
  .option("-k, --api-key <key>", "Etherscan API key", process.env.ETHERSCAN_API_KEY)
  .option("-d, --abi-dir <dir>", "Output directory for ABI files")
  .action(async (tokenName: string, address: string, opts) => {
    await fetchToken({
      tokenName,
      address,
      chain: opts.chain,
      apiKey: opts.apiKey,
      abiDir: opts.abiDir,
    });
  });

// --- fetch-all ---
program
  .command("fetch-all")
  .description("Batch-fetch ABIs and Solidity source for all tokens in the registry")
  .option("-c, --chain <chain>", "Only fetch tokens for this chain", process.env.CHAIN)
  .option("-m, --missing", "Only fetch tokens not yet downloaded", false)
  .option("--no-duplicates", "Skip tokens that appear on multiple chains")
  .option("-r, --registry <path>", "Path to custom registry JSON file")
  .option("-k, --api-key <key>", "Etherscan API key", process.env.ETHERSCAN_API_KEY)
  .option("-d, --abi-dir <dir>", "Output directory for ABI files")
  .action(async (opts) => {
    await fetchAllTokens({
      chain: opts.chain,
      missing: opts.missing,
      noDuplicates: !opts.duplicates,
      registry: opts.registry,
      apiKey: opts.apiKey,
      abiDir: opts.abiDir,
    });
  });

// --- update-modules ---
program
  .command("update-modules")
  .description("Generate mod.rs for ERC-20 token modules and verify build")
  .option("--repo-root <dir>", "Repository root directory")
  .action(async (opts) => {
    await updateModules({ repoRoot: opts.repoRoot });
  });

// --- filter-standard ---
program
  .command("filter-standard")
  .description("Filter out ERC-20 token ABIs that only contain standard events")
  .option("--delete", "Remove standard-only ABI and .sol files")
  .option("--move <dir>", "Move standard-only files to the specified directory")
  .option("-d, --abi-dir <dir>", "ABI directory to scan")
  .action(async (opts) => {
    let mode: "dry-run" | "delete" | "move" = "dry-run";
    if (opts.delete) mode = "delete";
    else if (opts.move) mode = "move";

    await filterStandard({
      mode,
      moveDir: opts.move,
      abiDir: opts.abiDir,
    });
  });

// --- keccak ---
program
  .command("keccak")
  .description("Compute keccak256 hash of an event signature, or find a matching signature")
  .option("-s, --signature <sig>", 'Event signature to hash (e.g. "Transfer(address,address,uint256)")')
  .option("-t, --target <hash>", "Target keccak256 hash to find a match for")
  .option("-n, --names <names...>", "Event names to try when searching for a match")
  .option("--types <types...>", "Parameter types to try (comma-separated sets)")
  .action((opts) => {
    if (opts.signature) {
      const hash = computeKeccak(opts.signature);
      console.log(`${opts.signature} => ${hash}`);
      return;
    }

    if (opts.target) {
      const names = opts.names ?? [
        "TokenCreated",
        "TokenCreate",
        "NewToken",
        "TokenLaunched",
        "CreateToken",
        "TokenFactoryCreate",
      ];
      const typeSets = opts.types
        ? [opts.types.map((t: string) => t.split(",")).flat()]
        : [["address", "address", "uint256", "uint256", "string", "string"]];

      const match = findKeccakMatch({ target: opts.target, names, typeSets });
      if (match) {
        console.log(`MATCH: ${match}`);
      } else {
        console.log("No match in tested variants.");
      }
      return;
    }

    console.log("Provide --signature to hash, or --target to search. See --help for details.");
  });

// --- build-registry ---
program
  .command("build-registry")
  .description("Build token-registry.json from CoinGecko API")
  .option("-o, --output <path>", "Output file path for the registry JSON")
  .option("-k, --api-key <key>", "CoinGecko API key", process.env.COINGECKO_API_KEY)
  .action(async (opts) => {
    await buildRegistry({
      output: opts.output,
      apiKey: opts.apiKey,
    });
  });

program.parse();
