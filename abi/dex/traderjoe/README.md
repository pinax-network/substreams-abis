# Trader Joe

Leading DEX on Avalanche featuring the Liquidity Book (LB) model with bin-based concentrated liquidity.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| LBRouter | `0xb4315e873dBcf96Ffd0acd8EA43f689D8c20fB30` | Avalanche | [Snowtrace](https://snowtrace.io/address/0xb4315e873dBcf96Ffd0acd8EA43f689D8c20fB30) |
| LBFactory | Various deployments | Multi-chain | — |
| LBPair | Various deployments (per pool) | Multi-chain | — |

## Key Events

### LBPair (Pool)

- **`Swap`** — Emitted on every trade through a Liquidity Book pool
- **`DepositedToBins`** — Emitted when liquidity is added across bins
- **`WithdrawnFromBins`** — Emitted when liquidity is removed from bins
- **`CompositionFees`** — Emitted when composition fees are collected per bin
- **`CollectedProtocolFees`** — Emitted when protocol fees are collected
- **`FlashLoan`** — Emitted on flash loan execution

### LBFactory

- **`LBPairCreated`** — Emitted when a new Liquidity Book pair is deployed

## Packed `bytes32` Encoding

Trader Joe's Liquidity Book uses a gas optimization where **two `uint128` values are packed into a single `bytes32`**:

```
bytes32 layout: [ upper 128 bits (tokenX) | lower 128 bits (tokenY) ]
```

This applies to the following fields in events:

| Event | Field | Upper 128 bits | Lower 128 bits |
|-------|-------|---------------|----------------|
| Swap | `amountsIn` | tokenX amount in | tokenY amount in |
| Swap | `amountsOut` | tokenX amount out | tokenY amount out |
| Swap | `totalFees` | tokenX fees | tokenY fees |
| Swap | `protocolFees` | tokenX protocol fees | tokenY protocol fees |
| CompositionFees | `totalFees` | tokenX fees | tokenY fees |
| CompositionFees | `protocolFees` | tokenX protocol fees | tokenY protocol fees |

### Decoding

```rust
// Solidity encoding
bytes32 packed = bytes32(uint256(x) << 128 | uint256(y));

// Rust decoding
let x = BigInt::from_unsigned_bytes_be(&packed[..16]);  // upper 128 bits = tokenX
let y = BigInt::from_unsigned_bytes_be(&packed[16..]);   // lower 128 bits = tokenY
```

### Swap Direction

In a swap, only **one side** of each packed value is non-zero:

```
AVAX → USDC swap:
  amountsIn:  [ 100000000 (AVAX) |     0          ]
  amountsOut: [     0            | 200000000 (USDC)]

USDC → AVAX swap:
  amountsIn:  [     0            | 200000000 (USDC)]
  amountsOut: [ 100000000 (AVAX) |     0           ]
```

The fee fields follow the same pattern — fees are denominated in the **input token**.

## Overview

### LBPair
The core pool contract. Each pair has a configurable `binStep` that determines the price granularity. Trades happen across discrete price bins with zero slippage within each bin.

### LBRouter
The swap router for Liquidity Book pools. Routes trades through bin-based liquidity.

### LBFactory
Factory deploying new Liquidity Book pairs with configurable bin steps and fee parameters.

## Links

- [Website](https://traderjoexyz.com)
- [Docs](https://docs.traderjoexyz.com)
- [GitHub](https://github.com/traderjoe-xyz)
