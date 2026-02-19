# Agent Skills

Instructions for AI agents working on this repository.

## Skills

| Skill | Description |
|-------|-------------|
| [add-abi.md](add-abi.md) | How to add new protocol or token ABIs |
| [abi-naming.md](abi-naming.md) | Naming conventions, special cases, merge vs. keep rules |
| [build-and-test.md](build-and-test.md) | Build system, testing, common failures |
| [pr-workflow.md](pr-workflow.md) | Branch naming, PR process, releases |

## Quick Reference

### Adding a new DEX ABI
1. Fetch ABI from block explorer → `abi/dex/{protocol}/{Contract}.json`
2. Create `src/dex/{protocol}/mod.rs` with `pub mod contractname;`
3. Add `pub mod {protocol};` to `src/dex/mod.rs`
4. `cargo build && cargo test`
5. Create/update `abi/dex/{protocol}/README.md` with contract info and key events
6. Open PR with `DenisCarriere` as reviewer

### Adding a new token ABI
1. Fetch ABI → `abi/tokens/erc20/{TOKEN}.json`
2. Add `pub mod tokenname;` to `src/tokens/erc20/mod.rs`
3. `cargo build && cargo test`
4. Update `abi/tokens/erc20/README.md` — add to token table + multi-chain deployments
5. Open PR

### Category structure
```
abi/
├── bridge/        — Cross-chain bridges
├── dex/           — DEXes and swap protocols
├── lending/       — Lending/borrowing
├── naming/        — Name services
├── oracle/        — Price oracles
├── perps/         — Perpetual exchanges
├── prediction/    — Prediction markets
├── restaking/     — Restaking
├── stablecoin/    — Stablecoin systems
├── staking/       — Liquid staking
├── standard/      — Token standards (ERC-20, ERC-721, etc.)
├── tokens/        — Specific token ABIs
└── yield/         — Yield protocols
```
