#!/usr/bin/env bash
#
# Batch-fetch ABIs and Solidity source for all tokens in the registry.
#
# Usage:
#   ./scripts/fetch-all-tokens.sh              # Fetch all tokens
#   ./scripts/fetch-all-tokens.sh --chain eth   # Only ETH mainnet tokens
#   ./scripts/fetch-all-tokens.sh --chain base  # Only Base tokens
#   ./scripts/fetch-all-tokens.sh --missing     # Only tokens not yet downloaded
#
# Environment variables:
#   ETHERSCAN_API_KEY  - Optional API key for higher rate limits
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
ABI_DIR="$REPO_ROOT/abi/erc20-tokens"

FILTER_CHAIN=""
ONLY_MISSING=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --chain) FILTER_CHAIN="$2"; shift 2 ;;
    --missing) ONLY_MISSING=true; shift ;;
    *) echo "Unknown option: $1"; exit 1 ;;
  esac
done

# ============================================================================
# TOKEN REGISTRY
# Format: TOKEN_NAME|CONTRACT_ADDRESS|CHAIN
#
# Add new tokens here and re-run the script.
# ============================================================================
TOKENS=(
  # --- ETH Mainnet ---
  "SHIB|0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE|eth"
  "LINK|0x514910771AF9Ca656af840dff83E8264EcF986CA|eth"
  "UNI|0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984|eth"
  "MATIC|0x7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0|eth"
  "ARB|0xB50721BCf8d664c30412Cfbc6cf7a15145234ad1|eth"
  "LDO|0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32|eth"
  "AAVE|0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9|eth"
  "MKR|0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2|eth"
  "CRV|0xD533a949740bb3306d119CC777fa900bA034cd52|eth"
  "PEPE|0x6982508145454Ce325dDbE47a25d4ec3d2311933|eth"
  "APE|0x4d224452801ACEd8B2F0aebE155379bb5D594381|eth"
  "SNX|0xC011a73ee8576Fb46F5E1c5751cA3B9Fe0af2a6F|eth"
  "COMP|0xc00e94Cb662C3520282E6f5717214004A7f26888|eth"
  "GRT|0xc944E90C64B2c07662A292be6244BDf05Cda44a7|eth"
  "FET|0xaea46A60368A7bD060eec7DF8CBa43b7EF41Ad85|eth"
  "DAI|0x6B175474E89094C44Da98b954EedeAC495271d0F|eth"
  "WETH|0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2|eth"
  "WBTC|0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599|eth"
  "stETH|0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84|eth"
  "RPL|0xD33526068D116cE69F19A9ee46F0bd304F21A51f|eth"
  "ENS|0xC18360217D8F7Ab5e7c516566761Ea12Ce7F9D72|eth"
  "LRC|0xBBbbCA6A901c926F240b89EacB641d8Aec7AEafD|eth"
  "BAT|0x0D8775F648430679A709E98d2b0Cb6250d2887EF|eth"
  "1INCH|0x111111111117dC0aa78b770fA6A738034120C302|eth"
  "SUSHI|0x6B3595068778DD592e39A122f4f5a5cF09C90fE2|eth"
  "YFI|0x0bc529c00C6401aEF6D220BE8C6Ea1667F6Ad93e|eth"
  "BLUR|0x5283D291DBCF85356A21bA090E6db59121208b44|eth"
  "IMX|0xF57e7e7C23978C3cAEC3C3548E3D615c346e79fF|eth"
  "RNDR|0x6De037ef9aD2725EB40118Bb1702EBb27e4Aeb24|eth"
  "FXS|0x3432B6A60D23Ca0dFCa7761B7ab56459D9C964D0|eth"
  "PENDLE|0x808507121B80c02388fAd14726482e061B8da827|eth"
  "WLD|0x163f8C2467924be0ae7B5347228CABF260318753|eth"
  "ENA|0x57e114B691Db790C35207b2e685D4A43181e6061|eth"
  "EIGEN|0xec53bF9167f50cDEB3Ae105f56099aaaB9061F83|eth"

  # --- Base ---
  "AERO|0x940181a94A35A4569E4529A3CDfB74e38FD98631|base"
  "BRETT|0x532f27101965dd16442E59d40670FaF5eBB142E4|base"
  "DEGEN|0x4ed4E862860beD51a9570b96d89aF5E1B0Efefed|base"
  "TOSHI|0xAC1Bd2486aAf3B5C0fc3Fd868558b082a531B2B4|base"
  "WELL|0xA88594D404727625A9437C3f886C7643872296AE|base"
  "MORPHO|0xBAa5CC21fd487B8Fcc2F632f3F4E8D37262a0842|base"
  "VIRTUAL|0x0b3e328455c4059EEb9e3f84b5543F74E24e7E1b|base"
  "cbBTC|0xcbB7C0000aB88B473b1f5aFd9ef808440eed33Bf|base"
  "USDbC|0xd9aAEc86B65D86f6A7B5B1b0c42FFA531710b6CA|base"
  "HIGHER|0x0578d8A44db98B23BF096A382e016e29a5Ce0ffe|base"
)

# Counters
TOTAL=0
FETCHED=0
SKIPPED=0
FAILED=0

echo "================================================"
echo "  Token ABI Batch Fetcher"
echo "  Registry: ${#TOKENS[@]} tokens"
echo "================================================"
echo ""

for entry in "${TOKENS[@]}"; do
  IFS='|' read -r NAME ADDR CHAIN <<< "$entry"

  # Filter by chain if specified
  if [ -n "$FILTER_CHAIN" ] && [ "$CHAIN" != "$FILTER_CHAIN" ]; then
    continue
  fi

  TOTAL=$((TOTAL + 1))

  # Skip if already downloaded and --missing flag is set
  if [ "$ONLY_MISSING" = true ] && [ -f "$ABI_DIR/$NAME.json" ]; then
    echo "SKIP: $NAME (already exists)"
    SKIPPED=$((SKIPPED + 1))
    continue
  fi

  echo ""
  echo "[$TOTAL/${#TOKENS[@]}] Fetching $NAME..."
  if "$SCRIPT_DIR/fetch-token.sh" "$NAME" "$ADDR" "$CHAIN"; then
    FETCHED=$((FETCHED + 1))
  else
    echo "  FAILED: $NAME"
    FAILED=$((FAILED + 1))
  fi

  # Rate limit between tokens (Etherscan free tier: 5 req/sec, we do 2 req/token)
  sleep 0.5
done

echo ""
echo "================================================"
echo "  Results"
echo "  Total:   $TOTAL"
echo "  Fetched: $FETCHED"
echo "  Skipped: $SKIPPED"
echo "  Failed:  $FAILED"
echo "================================================"

# List all ABI files
echo ""
echo "ABI files in abi/erc20-tokens/:"
ls -1 "$ABI_DIR"/*.json 2>/dev/null | while read f; do
  echo "  $(basename "$f")"
done
