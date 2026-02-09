/**
 * Filter out ERC-20 token ABIs that only contain standard events.
 *
 * Standard events: Transfer, Approval, OwnershipTransferred, EIP712DomainChanged,
 * Initialized, Upgraded, AdminChanged, BeaconUpgraded
 */
import { readdirSync, readFileSync, existsSync, unlinkSync, mkdirSync, renameSync } from "fs";
import { resolve, dirname, basename } from "path";

const STANDARD_EVENTS = new Set([
  "Transfer",
  "Approval",
  "OwnershipTransferred",
  "EIP712DomainChanged",
  "Initialized",
  "Upgraded",
  "AdminChanged",
  "BeaconUpgraded",
]);

export interface FilterStandardOptions {
  mode?: "dry-run" | "delete" | "move";
  moveDir?: string;
  abiDir?: string;
}

export interface FilterStandardResult {
  total: number;
  standard: number;
  nonStandard: number;
  errors: number;
  standardTokens: string[];
  nonStandardTokens: string[];
}

export async function filterStandard(
  opts: FilterStandardOptions = {}
): Promise<FilterStandardResult> {
  const mode = opts.mode ?? "dry-run";
  const repoRoot = resolve(dirname(new URL(import.meta.url).pathname), "..");
  const abiDir = opts.abiDir ?? resolve(repoRoot, "abi/erc20-tokens");

  const result: FilterStandardResult = {
    total: 0,
    standard: 0,
    nonStandard: 0,
    errors: 0,
    standardTokens: [],
    nonStandardTokens: [],
  };

  console.log("================================================");
  console.log("  ERC-20 ABI Standard Event Filter");
  console.log(`  Mode: ${mode}`);
  console.log("================================================\n");

  if (!existsSync(abiDir)) {
    console.log(`ABI directory not found: ${abiDir}`);
    return result;
  }

  const files = readdirSync(abiDir).filter((f) => f.endsWith(".json"));

  for (const file of files) {
    const filePath = resolve(abiDir, file);
    const token = basename(file, ".json");
    result.total++;

    try {
      const abi = JSON.parse(readFileSync(filePath, "utf-8"));
      const events: string[] = abi
        .filter((entry: any) => entry.type === "event")
        .map((entry: any) => entry.name);

      // Filter out standard events
      const nonStandard = events.filter((e) => !STANDARD_EVENTS.has(e));

      if (nonStandard.length === 0) {
        result.standard++;
        result.standardTokens.push(token);

        switch (mode) {
          case "dry-run":
            console.log(`  STANDARD: ${token} (events: ${events.join(", ") || "none"})`);
            break;
          case "delete":
            unlinkSync(filePath);
            const solPath = resolve(abiDir, `${token}.sol`);
            if (existsSync(solPath)) unlinkSync(solPath);
            console.log(`  DELETED:  ${token}`);
            break;
          case "move":
            if (opts.moveDir) {
              mkdirSync(opts.moveDir, { recursive: true });
              renameSync(filePath, resolve(opts.moveDir, file));
              const solMovePath = resolve(abiDir, `${token}.sol`);
              if (existsSync(solMovePath)) {
                renameSync(solMovePath, resolve(opts.moveDir, `${token}.sol`));
              }
              console.log(`  MOVED:    ${token} -> ${opts.moveDir}/`);
            }
            break;
        }
      } else {
        result.nonStandard++;
        result.nonStandardTokens.push(token);
        console.log(`  KEEP:     ${token} (non-standard: ${nonStandard.join(", ")})`);
      }
    } catch (err: any) {
      console.log(`  ERROR: Failed to parse ${token}.json - ${err.message}`);
      result.errors++;
    }
  }

  console.log("\n================================================");
  console.log("  Results");
  console.log(`  Total:        ${result.total}`);
  console.log(
    `  Standard:     ${result.standard} (${mode === "dry-run" ? "would be removed" : "removed/moved"})`
  );
  console.log(`  Non-standard: ${result.nonStandard} (kept)`);
  console.log(`  Errors:       ${result.errors}`);
  console.log("================================================");

  if (mode === "dry-run" && result.standard > 0) {
    console.log("\nRun with --delete to remove standard-only ABIs, or --move DIR to archive them.");
  }

  return result;
}
