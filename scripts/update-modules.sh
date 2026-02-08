#!/usr/bin/env bash
#
# After fetching new ABIs, this script:
# 1. Runs cargo build to generate Rust code from ABI JSONs
# 2. Auto-generates mod.rs for src/erc20_tokens/ based on existing .rs files
#
# Usage:
#   ./scripts/update-modules.sh
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
TOKENS_SRC="$REPO_ROOT/src/erc20_tokens"
TOKENS_ABI="$REPO_ROOT/abi/erc20-tokens"

echo "=== Step 1: Building to generate Rust code from ABIs ==="
cd "$REPO_ROOT"
cargo build 2>&1

echo ""
echo "=== Step 2: Generating src/erc20_tokens/mod.rs ==="

MOD_FILE="$TOKENS_SRC/mod.rs"

# Collect all .rs files (not mod.rs) and subdirectories with mod.rs
{
  echo "// @generated"
  echo "// Auto-generated module declarations for ERC-20 tokens."
  echo "// Re-run ./scripts/update-modules.sh after adding new ABIs."
  echo ""

  # Direct .rs files -> pub mod {name};
  for f in "$TOKENS_SRC"/*.rs; do
    base="$(basename "$f" .rs)"
    [ "$base" = "mod" ] && continue
    echo "pub mod $base;"
  done

  # Subdirectories with mod.rs -> pub mod {dir};
  for d in "$TOKENS_SRC"/*/; do
    [ -d "$d" ] || continue
    dir_name="$(basename "$d")"
    if [ -f "$d/mod.rs" ]; then
      echo "pub mod $dir_name;"
    fi
  done
} > "$MOD_FILE"

echo "Generated $MOD_FILE with modules:"
grep "^pub mod" "$MOD_FILE" | sed 's/^/  /'

echo ""
echo "=== Step 3: Verifying build ==="
cargo build 2>&1

echo ""
echo "=== Done ==="
echo "New token modules are ready. Import them as:"
echo "  use substreams_abis::erc20_tokens::<token_name>;"
