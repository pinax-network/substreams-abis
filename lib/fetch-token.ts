/**
 * Fetch ABI and Solidity source for a token contract from Etherscan/Basescan.
 */
import { existsSync, mkdirSync, writeFileSync } from "fs";
import { resolve, dirname } from "path";
import { API_BASE, getChainConfig } from "./chains";

const RATE_LIMIT_SLEEP = 250; // ms

function sleep(ms: number): Promise<void> {
  return new Promise((r) => setTimeout(r, ms));
}

export interface FetchTokenOptions {
  tokenName: string;
  address: string;
  chain?: string;
  apiKey?: string;
  abiDir?: string;
}

export interface FetchTokenResult {
  tokenName: string;
  address: string;
  chain: string;
  abiSaved: boolean;
  sourceSaved: boolean;
  abiPath?: string;
  sourcePath?: string;
  contractName?: string;
  compiler?: string;
  error?: string;
}

export async function fetchToken(opts: FetchTokenOptions): Promise<FetchTokenResult> {
  const chain = opts.chain ?? "eth";
  const config = getChainConfig(chain);
  const repoRoot = resolve(dirname(new URL(import.meta.url).pathname), "..");
  const abiDir = opts.abiDir ?? resolve(repoRoot, "abi/erc20-tokens");

  mkdirSync(abiDir, { recursive: true });

  const apiKeyParam = opts.apiKey ? `&apikey=${opts.apiKey}` : "";
  const chainParam = `chainid=${config.chainId}`;
  const result: FetchTokenResult = {
    tokenName: opts.tokenName,
    address: opts.address,
    chain,
    abiSaved: false,
    sourceSaved: false,
  };

  console.log(`Fetching ${opts.tokenName} (${opts.address}) from ${config.explorer}...`);

  // --- Fetch ABI ---
  console.log("  -> Fetching ABI...");
  const abiUrl = `${API_BASE}?${chainParam}&module=contract&action=getabi&address=${opts.address}${apiKeyParam}`;
  try {
    const abiResp = await fetch(abiUrl);
    const abiData = (await abiResp.json()) as { status: string; message: string; result: string };

    if (abiData.status === "1") {
      const abi = JSON.parse(abiData.result);
      const abiPath = resolve(abiDir, `${opts.tokenName}.json`);
      writeFileSync(abiPath, JSON.stringify(abi, null, 2) + "\n");
      result.abiSaved = true;
      result.abiPath = abiPath;
      console.log(`  -> Saved ABI to abi/erc20-tokens/${opts.tokenName}.json`);
    } else {
      console.log(`  -> ERROR fetching ABI: ${abiData.message}`);
      console.log(`     (Contract may not be verified on ${config.explorer})`);
      result.error = `ABI fetch failed: ${abiData.message}`;
    }
  } catch (err: any) {
    console.log(`  -> ERROR fetching ABI: ${err.message}`);
    result.error = `ABI fetch error: ${err.message}`;
  }

  await sleep(RATE_LIMIT_SLEEP);

  // --- Fetch Source Code ---
  console.log("  -> Fetching source code...");
  const srcUrl = `${API_BASE}?${chainParam}&module=contract&action=getsourcecode&address=${opts.address}${apiKeyParam}`;
  try {
    const srcResp = await fetch(srcUrl);
    const srcData = (await srcResp.json()) as {
      status: string;
      message: string;
      result: Array<{ SourceCode: string; ContractName: string; CompilerVersion: string }>;
    };

    if (srcData.status === "1") {
      const sourceCode = srcData.result[0]?.SourceCode;
      const contractName = srcData.result[0]?.ContractName;
      const compiler = srcData.result[0]?.CompilerVersion;

      result.contractName = contractName;
      result.compiler = compiler;

      if (sourceCode && sourceCode !== "null" && sourceCode !== "") {
        const solPath = resolve(abiDir, `${opts.tokenName}.sol`);

        if (sourceCode.startsWith("{") && sourceCode.endsWith("}")) {
          // Multi-file Solidity source
          const cleaned = sourceCode.replace(/^\{\{/, "").replace(/\}\}$/, "");
          try {
            const parsed = JSON.parse(cleaned);
            if (parsed.sources) {
              const entries = Object.entries(parsed.sources) as [string, { content: string }][];
              const mainEntry = entries.find(([key]) => key.includes(contractName!));
              const content = mainEntry?.[1]?.content ?? entries[0]?.[1]?.content;
              if (content) {
                writeFileSync(solPath, content);
                result.sourceSaved = true;
                result.sourcePath = solPath;
                console.log(
                  `  -> Saved source to abi/erc20-tokens/${opts.tokenName}.sol (contract: ${contractName}, compiler: ${compiler})`
                );
              } else {
                console.log("  -> Could not extract main source from multi-file contract");
              }
            } else {
              writeFileSync(solPath, sourceCode);
              result.sourceSaved = true;
              result.sourcePath = solPath;
              console.log(
                `  -> Saved source to abi/erc20-tokens/${opts.tokenName}.sol (contract: ${contractName}, compiler: ${compiler})`
              );
            }
          } catch {
            writeFileSync(solPath, sourceCode);
            result.sourceSaved = true;
            result.sourcePath = solPath;
            console.log(
              `  -> Saved source to abi/erc20-tokens/${opts.tokenName}.sol (contract: ${contractName}, compiler: ${compiler})`
            );
          }
        } else {
          // Single-file source
          writeFileSync(solPath, sourceCode);
          result.sourceSaved = true;
          result.sourcePath = solPath;
          console.log(
            `  -> Saved source to abi/erc20-tokens/${opts.tokenName}.sol (contract: ${contractName}, compiler: ${compiler})`
          );
        }
      } else {
        console.log("  -> No source code available (contract not verified or source not public)");
      }
    } else {
      console.log(`  -> ERROR fetching source: ${srcData.message}`);
    }
  } catch (err: any) {
    console.log(`  -> ERROR fetching source: ${err.message}`);
  }

  console.log(`Done: ${opts.tokenName}`);
  return result;
}
