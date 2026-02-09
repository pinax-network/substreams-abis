/**
 * After fetching new ABIs, this module:
 * 1. Runs cargo build to generate Rust code from ABI JSONs
 * 2. Auto-generates mod.rs for src/erc20_tokens/ based on existing .rs files
 */
import { readdirSync, statSync, existsSync, writeFileSync } from "fs";
import { resolve, dirname, basename } from "path";
import { execSync } from "child_process";

export interface UpdateModulesOptions {
  repoRoot?: string;
}

export async function updateModules(opts: UpdateModulesOptions = {}): Promise<void> {
  const repoRoot = opts.repoRoot ?? resolve(dirname(new URL(import.meta.url).pathname), "..");
  const tokensSrc = resolve(repoRoot, "src/erc20_tokens");

  console.log("=== Step 1: Building to generate Rust code from ABIs ===");
  execSync("cargo build", { cwd: repoRoot, stdio: "inherit" });

  console.log("\n=== Step 2: Generating src/erc20_tokens/mod.rs ===");
  const modFile = resolve(tokensSrc, "mod.rs");

  const lines: string[] = [
    "// @generated",
    "// Auto-generated module declarations for ERC-20 tokens.",
    "// Re-run 'bun run cli.ts update-modules' after adding new ABIs.",
    "",
  ];

  // Direct .rs files -> pub mod {name};
  if (existsSync(tokensSrc)) {
    const files = readdirSync(tokensSrc).filter(
      (f) => f.endsWith(".rs") && f !== "mod.rs"
    );
    for (const f of files.sort()) {
      const name = basename(f, ".rs");
      lines.push(`pub mod ${name};`);
    }

    // Subdirectories with mod.rs -> pub mod {dir};
    const dirs = readdirSync(tokensSrc).filter((d) => {
      const fullPath = resolve(tokensSrc, d);
      return statSync(fullPath).isDirectory() && existsSync(resolve(fullPath, "mod.rs"));
    });
    for (const d of dirs.sort()) {
      lines.push(`pub mod ${d};`);
    }
  }

  writeFileSync(modFile, lines.join("\n") + "\n");
  console.log(`Generated ${modFile} with modules:`);
  lines.filter((l) => l.startsWith("pub mod")).forEach((l) => console.log(`  ${l}`));

  console.log("\n=== Step 3: Verifying build ===");
  execSync("cargo build", { cwd: repoRoot, stdio: "inherit" });

  console.log("\n=== Done ===");
  console.log("New token modules are ready. Import them as:");
  console.log("  use substreams_abis::erc20_tokens::<token_name>;");
}
