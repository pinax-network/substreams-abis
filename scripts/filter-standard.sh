#!/usr/bin/env bash
#
# Filter out ERC-20 token ABIs that only contain standard events.
#
# Standard events (ignored during filtering):
#   Transfer, Approval, OwnershipTransferred, EIP712DomainChanged
#
# Tokens with ONLY these events are considered "standard" and can be removed,
# since the generic ERC-20 ABI already covers them.
#
# Usage:
#   ./scripts/filter-standard.sh              # Dry run (report only)
#   ./scripts/filter-standard.sh --delete     # Remove standard-only ABIs
#   ./scripts/filter-standard.sh --move DIR   # Move standard-only ABIs to DIR
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
ABI_DIR="$REPO_ROOT/abi/erc20-tokens"

MODE="dry-run"
MOVE_DIR=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --delete) MODE="delete"; shift ;;
    --move) MODE="move"; MOVE_DIR="$2"; shift 2 ;;
    --help|-h)
      echo "Usage: filter-standard.sh [--delete | --move DIR]"
      echo "  (no flags)  Dry run - report which tokens would be removed"
      echo "  --delete    Remove standard-only ABI and .sol files"
      echo "  --move DIR  Move standard-only files to DIR"
      exit 0
      ;;
    *) echo "Unknown option: $1"; exit 1 ;;
  esac
done

# Standard ERC-20 events to ignore
STANDARD_EVENTS="Transfer|Approval|OwnershipTransferred|EIP712DomainChanged|Initialized|Upgraded|AdminChanged|BeaconUpgraded"

TOTAL=0
STANDARD=0
NONSTANDARD=0
ERRORS=0

STANDARD_LIST=()
NONSTANDARD_LIST=()

echo "================================================"
echo "  ERC-20 ABI Standard Event Filter"
echo "  Mode: $MODE"
echo "================================================"
echo ""

for abi_file in "$ABI_DIR"/*.json; do
  [ -f "$abi_file" ] || continue
  TOKEN=$(basename "$abi_file" .json)
  TOTAL=$((TOTAL + 1))

  # Extract event names
  EVENTS=$(jq -r '[.[] | select(.type=="event") | .name] | join(",")' "$abi_file" 2>/dev/null) || {
    echo "  ERROR: Failed to parse $TOKEN.json"
    ERRORS=$((ERRORS + 1))
    continue
  }

  # Filter out standard events
  NON_STANDARD=""
  IFS=',' read -ra EVENT_ARRAY <<< "$EVENTS"
  for evt in "${EVENT_ARRAY[@]}"; do
    evt=$(echo "$evt" | xargs) # trim whitespace
    [ -z "$evt" ] && continue
    if ! echo "$evt" | grep -qxE "$STANDARD_EVENTS"; then
      NON_STANDARD="${NON_STANDARD:+$NON_STANDARD, }$evt"
    fi
  done

  if [ -z "$NON_STANDARD" ]; then
    STANDARD=$((STANDARD + 1))
    STANDARD_LIST+=("$TOKEN")

    case "$MODE" in
      dry-run)
        echo "  STANDARD: $TOKEN (events: ${EVENTS:-none})"
        ;;
      delete)
        rm -f "$abi_file"
        rm -f "$ABI_DIR/$TOKEN.sol"
        echo "  DELETED:  $TOKEN"
        ;;
      move)
        mkdir -p "$MOVE_DIR"
        mv "$abi_file" "$MOVE_DIR/"
        [ -f "$ABI_DIR/$TOKEN.sol" ] && mv "$ABI_DIR/$TOKEN.sol" "$MOVE_DIR/"
        echo "  MOVED:    $TOKEN -> $MOVE_DIR/"
        ;;
    esac
  else
    NONSTANDARD=$((NONSTANDARD + 1))
    NONSTANDARD_LIST+=("$TOKEN")
    echo "  KEEP:     $TOKEN (non-standard: $NON_STANDARD)"
  fi
done

echo ""
echo "================================================"
echo "  Results"
echo "  Total:        $TOTAL"
echo "  Standard:     $STANDARD ($([ "$MODE" = "dry-run" ] && echo "would be removed" || echo "removed/moved"))"
echo "  Non-standard: $NONSTANDARD (kept)"
echo "  Errors:       $ERRORS"
echo "================================================"

if [ "$MODE" = "dry-run" ] && [ "$STANDARD" -gt 0 ]; then
  echo ""
  echo "Run with --delete to remove standard-only ABIs, or --move DIR to archive them."
fi
