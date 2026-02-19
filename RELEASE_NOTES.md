# v1.1.0

## ✨ New Protocols

### DEXes
- **CurveFi** — StableSwap (3pool), CryptoSwap (TriCrypto2), MetaPool Registry, CryptoSwap Factory
- **Bancor V3** — Network, NetworkInfo
- **Bancor Carbon** — Carbon Controller (on-chain order book DEX)
- **Balancer V2** — Vault (the main swap entry point)
- **Hashflow** — RFQ-based Router
- **ParaSwap** — Augustus Swapper V6.2
- **WOOFi** — WooRouterV2 (Arbitrum)
- **KyberSwap V2** — MetaAggregationRouterV2
- **Fraxswap** — TWAMM Router
- **ShibaSwap** — UniswapV2Router02

### Other
- **Maker/Sky** — Vat, DaiJoin, DSRManager (stablecoin)
- **Chainlink** — OffchainAggregator, FeedRegistry (oracle)
- **WETH9** — Wrapped ETH with deposit/withdrawal events (token)
- **Camelot** — Router, Factory (Arbitrum DEX)
- **Convex** — Booster, BaseRewardPool (yield)
- **LayerZero** — Endpoint, UltraLightNodeV2 (bridge)

## 📚 Documentation

- **Protocol READMEs** — Every protocol directory now has a README.md with contract addresses, chain deployments, key events, and documentation links
- **Token READMEs** — All 75+ ERC-20 tokens documented with multi-chain deployment addresses (via CoinGecko), plus USDC/USDT variant docs and 7 NFT collections
- **Agent Skills** — New `.agents/skills/` directory with structured instructions for AI agents: add-abi workflow, naming conventions, build/test guide, PR process

## 🔧 CI/CD

- Auto-publish to crates.io on new GitHub releases
- Removed `cargo fmt` CI check (generated code isn't formatted)

## 📊 Stats

- **13 categories**, **50+ protocols**, **120+ contract ABIs**
- **80+ ERC-20 token ABIs** with multi-chain addresses
- **7 ERC-721 collection ABIs**
