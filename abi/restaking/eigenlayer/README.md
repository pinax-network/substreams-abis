# EigenLayer

Restaking protocol enabling Ethereum stakers to opt-in to securing additional services (AVSs) by restaking their ETH or LSTs.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| StrategyManager | Various deployments | Ethereum | — |
| DelegationManager | Various deployments | Ethereum | — |

## Key Events

- **`Deposit`** — Emitted when a user deposits assets into a restaking strategy
- **`StakerDelegated`** — Emitted when a staker delegates to an operator
- **`StakerUndelegated`** — Emitted when a staker undelegates from an operator
- **`OperatorRegistered`** — Emitted when a new operator registers with EigenLayer
- **`OperatorSharesIncreased`** — Emitted when an operator's delegated shares increase

## Overview

### StrategyManager
Manages user deposits into restaking strategies. Users deposit LSTs or other assets which are then available for restaking across AVSs.

### DelegationManager
Handles delegation of restaked assets from stakers to operators. Manages the operator registry, delegation relationships, slashing, and withdrawal queuing.

## Links

- [Website](https://eigenlayer.xyz)
- [Docs](https://docs.eigenlayer.xyz)
- [GitHub](https://github.com/Layr-Labs)
