# v1.0.0

**Major release** — Complete reorganization of ABI categories and massive expansion of protocol coverage.

## 🏗️ Breaking Changes

All module paths have been reorganized into logical categories. Update your imports:

| Old Path | New Path |
|----------|----------|
| `seaport::` | `dex::seaport::` |
| `dca_dot_fun::` | `dex::dca_dot_fun::` |
| `polymarket::` | `prediction::polymarket::` |
| `ens::` | `naming::ens::` |
| `token::` | `standard::` |
| `erc20_tokens::` | `tokens::erc20::` |
| `erc721_tokens::` | `tokens::erc721::` |
| `yield_::` | `r#yield::` |

## 📁 New Category Structure

```
abi/
├── bridge/        — Cross-chain bridges (Across, Stargate, LayerZero)
├── dex/           — Decentralized exchanges (19 protocols)
├── lending/       — Lending protocols (Aave, Compound, Morpho)
├── naming/        — Name services (ENS)
├── oracle/        — Price oracles (Chainlink)
├── perps/         — Perpetual exchanges (GMX V2)
├── prediction/    — Prediction markets (Polymarket)
├── restaking/     — Restaking (EigenLayer)
├── stablecoin/    — Stablecoin protocols (Maker/Sky)
├── staking/       — Liquid staking (Lido, Rocket Pool, Coinbase)
├── standard/      — Token standards (ERC-20, ERC-721, ERC-1155, WETH9, etc.)
├── tokens/        — Specific token ABIs (80+ ERC-20, 7 ERC-721 collections)
└── yield/         — Yield protocols (Convex)
```

## ✨ New Protocols

### DEXes
- **Aerodrome** (Base) — Router, PoolFactory
- **Velodrome V2** (Optimism) — Router, PoolFactory
- **Trader Joe V2.1** (Avalanche) — LBRouter, LBFactory
- **Camelot** (Arbitrum) — Router, Factory
- **SushiSwap V1** — Factory, Pair
- **PancakeSwap V2 & V3** — Factory, Pair/Pool
- **1inch V5 & V6** — AggregationRouter
- **0x** — ExchangeProxy
- **DODO V2** — RouteProxy
- **Kyber Elastic** — Factory

### Lending
- **Aave V3** — Pool, Oracle, AToken, VariableDebtToken, AddressesProvider, DataProvider
- **Compound V3** — Comet (USDC)
- **Morpho Blue**

### Bridges
- **Across** — SpokePool
- **Stargate** — Router
- **LayerZero** — Endpoint, UltraLightNodeV2

### Oracles
- **Chainlink** — OffchainAggregator, FeedRegistry

### Staking & Restaking
- **Lido** — stETH, wstETH, WithdrawalQueue
- **Rocket Pool** — rETH
- **Coinbase** — cbETH
- **EigenLayer** — StrategyManager, DelegationManager

### Perps
- **GMX V2** (Arbitrum) — EventEmitter, Router

### Stablecoins
- **Maker/Sky** — Vat, DaiJoin, DSRManager

### Yield
- **Convex** — Booster, BaseRewardPool

### Tokens
- **WETH9** — Native wrapped ETH with deposit/withdrawal events

## 🔧 Other Changes
- Removed `cargo fmt` CI check (generated code from prost isn't formatted)
- Multi-chain ABI support: Base, Optimism, Avalanche, Arbitrum contracts included

## 📊 Coverage

- **13 categories**
- **40+ protocols**
- **100+ contract ABIs**
- **80+ ERC-20 token ABIs** (including USDC/USDT variants across chains)
- **7 ERC-721 collection ABIs**
