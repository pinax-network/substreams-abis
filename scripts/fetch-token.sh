#!/usr/bin/env bash
#
# Fetch ABI and Solidity source for a token contract from Etherscan/Basescan.
#
# Usage:
#   ./scripts/fetch-token.sh <TOKEN_NAME> <CONTRACT_ADDRESS> [CHAIN]
#
# Arguments:
#   TOKEN_NAME       - Token symbol in CAPS (e.g. SHIB, LINK, UNI)
#   CONTRACT_ADDRESS - The 0x... contract address
#   CHAIN            - "eth" (default) or "base"
#
# Examples:
#   ./scripts/fetch-token.sh SHIB 0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE
#   ./scripts/fetch-token.sh AERO 0x940181a94A35A4569E4529A3CDfB74e38FD98631 base
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
ABI_DIR="$REPO_ROOT/abi/evm/tokens"

TOKEN_NAME="${1:?Usage: fetch-token.sh <TOKEN_NAME> <ADDRESS> [CHAIN]}"
ADDRESS="${2:?Usage: fetch-token.sh <TOKEN_NAME> <ADDRESS> [CHAIN]}"
CHAIN="${3:-eth}"

# Rate limit: Etherscan free tier = 5 req/sec
RATE_LIMIT_SLEEP=0.25

case "$CHAIN" in
  eth|ethereum|mainnet)
    API_BASE="https://api.etherscan.io/api"
    EXPLORER="etherscan.io"
    ;;
  base)
    API_BASE="https://api.basescan.org/api"
    EXPLORER="basescan.org"
    ;;
  arbitrum|arb)
    API_BASE="https://api.arbiscan.io/api"
    EXPLORER="arbiscan.io"
    ;;
  optimism|op)
    API_BASE="https://api-optimistic.etherscan.io/api"
    EXPLORER="optimistic.etherscan.io"
    ;;
  polygon|matic)
    API_BASE="https://api.polygonscan.com/api"
    EXPLORER="polygonscan.com"
    ;;
  bsc|bnb)
    API_BASE="https://api.bscscan.com/api"
    EXPLORER="bscscan.com"
    ;;
  *)
    echo "Error: Unknown chain '$CHAIN'. Supported: eth, base, arbitrum, optimism, polygon, bsc"
    exit 1
    ;;
esac

# Optional API key from environment
API_KEY_PARAM=""
if [ -n "${ETHERSCAN_API_KEY:-}" ]; then
  API_KEY_PARAM="&apikey=$ETHERSCAN_API_KEY"
fi

mkdir -p "$ABI_DIR"

echo "Fetching $TOKEN_NAME ($ADDRESS) from $EXPLORER..."

# --- Fetch ABI ---
echo "  -> Fetching ABI..."
ABI_URL="${API_BASE}?module=contract&action=getabi&address=${ADDRESS}${API_KEY_PARAM}"
ABI_RESPONSE=$(curl -s "$ABI_URL")
ABI_STATUS=$(echo "$ABI_RESPONSE" | jq -r '.status')
ABI_MESSAGE=$(echo "$ABI_RESPONSE" | jq -r '.message')

if [ "$ABI_STATUS" = "1" ]; then
  echo "$ABI_RESPONSE" | jq -r '.result' | jq '.' > "$ABI_DIR/$TOKEN_NAME.json"
  echo "  -> Saved ABI to abi/evm/tokens/$TOKEN_NAME.json"
else
  echo "  -> ERROR fetching ABI: $ABI_MESSAGE"
  echo "     (Contract may not be verified on $EXPLORER)"
fi

sleep "$RATE_LIMIT_SLEEP"

# --- Fetch Source Code ---
echo "  -> Fetching source code..."
SRC_URL="${API_BASE}?module=contract&action=getsourcecode&address=${ADDRESS}${API_KEY_PARAM}"
SRC_RESPONSE=$(curl -s "$SRC_URL")
SRC_STATUS=$(echo "$SRC_RESPONSE" | jq -r '.status')

if [ "$SRC_STATUS" = "1" ]; then
  SOURCE_CODE=$(echo "$SRC_RESPONSE" | jq -r '.result[0].SourceCode')
  CONTRACT_NAME=$(echo "$SRC_RESPONSE" | jq -r '.result[0].ContractName')
  COMPILER=$(echo "$SRC_RESPONSE" | jq -r '.result[0].CompilerVersion')

  if [ -n "$SOURCE_CODE" ] && [ "$SOURCE_CODE" != "null" ] && [ "$SOURCE_CODE" != "" ]; then
    # Handle multi-file source (starts with {{ for JSON format)
    if [[ "$SOURCE_CODE" == "{"* ]] && [[ "$SOURCE_CODE" == *"}"* ]]; then
      # Multi-file Solidity source - extract the main contract
      # Try to find the main contract file in the sources
      CLEANED=$(echo "$SOURCE_CODE" | sed 's/^{{//' | sed 's/}}$//')
      if echo "$CLEANED" | jq -e '.sources' > /dev/null 2>&1; then
        # Standard JSON input format
        MAIN_SOURCE=$(echo "$CLEANED" | jq -r ".sources | to_entries | map(select(.key | test(\"$CONTRACT_NAME\"))) | .[0].value.content // empty")
        if [ -z "$MAIN_SOURCE" ]; then
          # Fallback: get the first source file
          MAIN_SOURCE=$(echo "$CLEANED" | jq -r '.sources | to_entries | .[0].value.content // empty')
        fi
        if [ -n "$MAIN_SOURCE" ]; then
          echo "$MAIN_SOURCE" > "$ABI_DIR/$TOKEN_NAME.sol"
          echo "  -> Saved source to abi/evm/tokens/$TOKEN_NAME.sol (contract: $CONTRACT_NAME, compiler: $COMPILER)"
        else
          echo "  -> Could not extract main source from multi-file contract"
        fi
      else
        # Plain multi-file - just save as-is
        echo "$SOURCE_CODE" > "$ABI_DIR/$TOKEN_NAME.sol"
        echo "  -> Saved source to abi/evm/tokens/$TOKEN_NAME.sol (contract: $CONTRACT_NAME, compiler: $COMPILER)"
      fi
    else
      # Single-file source
      echo "$SOURCE_CODE" > "$ABI_DIR/$TOKEN_NAME.sol"
      echo "  -> Saved source to abi/evm/tokens/$TOKEN_NAME.sol (contract: $CONTRACT_NAME, compiler: $COMPILER)"
    fi
  else
    echo "  -> No source code available (contract not verified or source not public)"
  fi
else
  echo "  -> ERROR fetching source: $(echo "$SRC_RESPONSE" | jq -r '.message')"
fi

echo "Done: $TOKEN_NAME"
