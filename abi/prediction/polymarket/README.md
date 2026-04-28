# Polymarket

The largest decentralized prediction market, enabling trading on the outcomes of real-world events using conditional tokens on Polygon.

## Contracts

Polymarket ABIs are scoped by protocol generation under `v1/` and `v2/`.
Contracts that have not changed for V2 remain available only under `v1/`.

| Version | Contract | Address | Chain | ABI |
|---------|----------|---------|-------|-----|
| V2 | CTFExchange | `0xE111180000d2663C0091e4f400237545B87B996B` | Polygon | `v2/CTFExchange.json` |
| V2 | NegRiskCTFExchange | `0xe2222d279d744050d28e00520010520000310F59` | Polygon | `v2/NegRiskCTFExchange.json` |
| V2 | CollateralToken (pUSD proxy) | `0xC011a7E12a19f7B1f670d46F03B03f3342E82DFB` | Polygon | `v2/CollateralToken.json` |
| V2 | CollateralToken (pUSD implementation) | `0x6bBCef9f7ef3B6C592c99e0f206a0DE94Ad0925f` | Polygon | `v2/CollateralToken.json` |
| V2 | CollateralOnramp | `0x93070a847efEf7F70739046A929D47a521F5B8ee` | Polygon | `v2/CollateralOnramp.json` |
| V2 | CollateralOfframp | `0x2957922Eb93258b93368531d39fAcCA3B4dC5854` | Polygon | `v2/CollateralOfframp.json` |
| V2 | PermissionedRamp | `0xebC2459Ec962869ca4c0bd1E06368272732BCb08` | Polygon | `v2/PermissionedRamp.json` |
| V2 | CtfCollateralAdapter | `0xADa100874d00e3331D00F2007a9c336a65009718` | Polygon | `v2/CtfCollateralAdapter.json` |
| V2 | NegRiskCtfCollateralAdapter | `0xAdA200001000ef00D07553cEE7006808F895c6F1` | Polygon | `v2/NegRiskCtfCollateralAdapter.json` |
| V1 | CTFExchange | Various deployments | Polygon | `v1/CTFExchange.json` |
| V1 | NegRiskCTFExchange | Various deployments | Polygon | `v1/NegRiskCTFExchange.json` |
| V1 | ConditionalTokens | `0x4D97DCd97eC945f40cF65F87097ACe5EA0476045` | Polygon | `v1/ConditionalTokens.json` |
| V1 | NegRiskAdapter | `0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296` | Polygon | `v1/NegRiskAdapter.json` |
| V1 | FixedProductMarketMaker | Various deployments | Polygon | `v1/FixedProductMarketMaker.json` |
| V1 | FixedProductMarketMakerFactory | Various deployments | Polygon | `v1/FixedProductMarketMakerFactory.json` |
| V1 | FeeModule | Various deployments | Polygon | `v1/FeeModule.json` |
| V1 | RelayHub | Various deployments | Polygon | `v1/RelayHub.json` |
| V1 | Gnosis Safe Factory | `0xaacfeea03eb1561c4e67d661e40682bd20e3541b` | Polygon | — |
| V1 | Polymarket Proxy Factory | `0xaB45c5A4B0c941a2F231C04C3f49182e1A254052` | Polygon | `v1/SafeProxyFactory.json` |
| V1 | UMA Adapter | `0x6A9D222616C90FcA5754cd1333cFD9b7fb6a4F74` | Polygon | `v1/UmaCtfAdapter/v2.json`, `v1/UmaCtfAdapter/v3.json` |
| V1 | UMA Optimistic Oracle | `0xCB1822859cEF82Cd2Eb4E6276C7916e692995130` | Polygon | — |

## Key Events

- **`OrderFilled`** — Emitted when a trade order is filled on the exchange
- **`OrdersMatched`** — Emitted when buy and sell orders are matched
- **`ConditionPreparation`** — Emitted when a new prediction market condition is created
- **`ConditionResolution`** — Emitted when a market outcome is resolved
- **`PositionSplit`** — Emitted when collateral is split into conditional outcome tokens

## Overview

### CTFExchange
The central limit order book exchange for trading conditional tokens (outcome shares). V1 supports limit orders with off-chain matching, cancellation, nonce management, and token registration. V2 adds the Polymarket V2 deployment at `0xE111180000d2663C0091e4f400237545B87B996B`, with order preapproval, fee receiver management, user pause controls, and updated order metadata fields.

### NegRiskCTFExchange
Exchange variant for negative risk markets where outcomes are mutually exclusive (e.g., multi-outcome elections). V2 adds the Polymarket V2 Neg Risk deployment at `0xe2222d279d744050d28e00520010520000310F59`.

### CollateralToken
pUSD collateral token used by Polymarket V2. The proxy is deployed at `0xC011a7E12a19f7B1f670d46F03B03f3342E82DFB` and the implementation is deployed at `0x6bBCef9f7ef3B6C592c99e0f206a0DE94Ad0925f`. The ABI covers ERC-20 transfer and permit flows, role-gated mint/burn controls, wrap/unwrap operations, ownership handover, and upgrade events.

### CollateralOnramp
Polymarket V2 onramp for wrapping supported collateral assets into pUSD. The ABI covers asset pause controls, admin role management, ownership handover, and wrap operations.

### CollateralOfframp
Polymarket V2 offramp for unwrapping pUSD back into supported collateral assets. The ABI covers asset pause controls, admin role management, ownership handover, and unwrap operations.

### PermissionedRamp
Polymarket V2 permissioned ramp for signed wrap and unwrap operations. The ABI covers witness management, EIP-712 domain data, nonces, asset pause controls, admin role management, ownership handover, and signed collateral movement.

### CtfCollateralAdapter
Polymarket V2 adapter between pUSD collateral and Conditional Tokens positions. The ABI covers ERC-1155 receiver hooks, split, merge, and redeem operations, collateral references, asset pause controls, admin role management, and ownership handover.

### NegRiskCtfCollateralAdapter
Polymarket V2 negative-risk adapter between pUSD collateral and Conditional Tokens positions. The ABI covers ERC-1155 receiver hooks, split, merge, redeem, and negative-risk position conversion operations, collateral references, asset pause controls, admin role management, and ownership handover.

### ConditionalTokens
The Gnosis Conditional Token Framework — ERC-1155 tokens representing positions in prediction markets. Handles splitting, merging, and redeeming outcome tokens. This core trading contract remains scoped under V1.

### NegRiskAdapter
Adapter enabling negative risk markets where buying "No" on one outcome is equivalent to buying "Yes" on all other outcomes. This core trading contract remains scoped under V1.

### Wallet Factories
V1-only wallet factory contracts used for Polymarket account infrastructure. The Gnosis Safe Factory is deployed at `0xaacfeea03eb1561c4e67d661e40682bd20e3541b`; the Polymarket Proxy Factory is deployed at `0xaB45c5A4B0c941a2F231C04C3f49182e1A254052` and uses `v1/SafeProxyFactory.json`.

### FixedProductMarketMaker
AMM-based market maker for prediction markets using a fixed product (LMSR) pricing model.

### Resolution Contracts
V1-only resolution contracts used to resolve market outcomes. The UMA Adapter is deployed at `0x6A9D222616C90FcA5754cd1333cFD9b7fb6a4F74` and uses `v1/UmaCtfAdapter/v2.json` and `v1/UmaCtfAdapter/v3.json`; the UMA Optimistic Oracle is deployed at `0xCB1822859cEF82Cd2Eb4E6276C7916e692995130`.

## Links

- [Website](https://polymarket.com)
- [Docs](https://docs.polymarket.com)
- [GitHub](https://github.com/Polymarket)
