# Substreams ABIs — Copilot Skills

## Project Structure

- **ABI JSON files** live in `abi/evm/` (and `abi/tvm/`), organized by protocol/token.
- **Generated Rust files** live in `src/evm/` (and `src/tvm/`), mirroring the ABI folder structure.
- `build.rs` uses `substreams-ethereum-abigen` to auto-generate `.rs` files from `.json` ABIs. It walks `./abi/` recursively, lowercases the file stem, and writes to the parallel path under `./src/`.
- Each subfolder needs a hand-maintained `mod.rs` to wire up the generated files.

## Naming Conventions

- ABI filenames with hyphens (e.g. `usdc-polygon-bridged.json`) generate `.rs` files with hyphens (`usdc-polygon-bridged.rs`). Since hyphens are invalid in Rust identifiers, use the `#[path]` attribute in `mod.rs`:

  ```rust
  #[path = "usdc-polygon-bridged.rs"]
  pub mod usdc_polygon_bridged;
  ```

- macOS has a **case-insensitive filesystem**. A folder `USDC/` and a file `usdc.rs` will conflict when Rust tries to resolve `pub mod usdc;`. Solutions:
  - Use lowercase folder names in `abi/` so `build.rs` generates lowercase folders in `src/`.
  - Or rename one side to avoid the collision.
- Prefer **descriptive names** over raw Solidity contract names. For example:
  - `UChildAdministrableERC20` → `usdc-polygon-bridged` (Polygon PoS bridged USDC)
  - `ArbitrumExtensionV2` → keep as-is if already clear, or rename to `usdt-arbitrum`

## When to Merge vs. Keep Separate ABIs

### Merge (keep the superset) when

- One ABI is a **strict superset** of another (identical events + all functions included + extras).
- Example: **USDC FiatTokenV2_1 ⊂ FiatTokenV2_2** — V2_2 has identical events and all V2_1 functions plus `initializeV2_2` and overloaded signatures. Safe to drop V2_1.

### Keep separate when

- Contracts have **different event signatures** (different topic hashes), even if event names look similar.
  - Example: USDT v0.4.18 uses `AddedBlackList`/`RemovedBlackList` while v0.8.4 uses `BlockPlaced`/`BlockReleased` — different names = different keccak256 hashes.
- Contracts have **different admin/role models** (e.g. OpenZeppelin AccessControl vs. single-owner pattern).
  - Example: USDC FiatTokenV2_2 (owner/masterMinter) vs. Polygon bridged USDC (RoleGranted/RoleRevoked).
- Contracts have **chain-specific functionality** (bridge mint/burn, L2 extensions, meta-transactions).
  - Example: USDT Arbitrum Extension has `bridgeMint`/`bridgeBurn`, OFT integration, EIP-3009 authorization — none of which exist in the Ethereum v0.8.4 contract.
- Neither ABI is a **superset** of the other — each has unique functions/events.
  - Example: USDT v0.8.4 has `addPrivilegedContract`/`removePrivilegedContract` which the Arbitrum version dropped.

### Comparison checklist for any smart contract

1. **Extract events** from each ABI variant. Compare names — same name ≠ same signature if parameter types differ.
2. **Extract functions** from each ABI variant. Check for strict subset relationships.
3. If ABI_A ⊂ ABI_B (all events and functions in A exist in B), safe to drop A.
4. If both have unique items, keep separate.
5. Pay special attention to: bridged/wrapped variants, chain-specific extensions, version upgrades that change admin patterns.

## Common Token ABI Patterns

### USDC variants

- **FiatTokenV2_2** (Circle native): owner/masterMinter pattern, mint/burn, EIP-2612 permit, EIP-3009 authorization. Deployed on Ethereum mainnet. Superset of V2_1.
- **Polygon bridged (UChildAdministrableERC20)**: OpenZeppelin AccessControl roles, deposit/withdraw (bridge), meta-transactions. No mint/burn.

### USDT variants

- **v0.4.18** (Ethereum L1 legacy): Solidity 0.4, BlackList/BlackFunds, Pause/Unpause, fee params (basisPointsRate, maximumFee), Deprecate/Issue pattern. Non-standard transfer/approve (no bool return).
- **v0.8.4** (modern): Solidity 0.8, BlockPlaced/BlockReleased, EIP-2612 permit, PrivilegedContract system. No fee mechanism.
- **Arbitrum Extension**: Builds on v0.8.4 + L2 bridge (bridgeMint/bridgeBurn), EIP-3009 (transferWithAuthorization, receiveWithAuthorization), OFT/LayerZero integration. Drops PrivilegedContract.

### General rule of thumb

- **Same chain, version upgrade, strict superset** → merge (keep latest)
- **Different chain or bridge variant** → always keep separate
- **Same chain but different architecture** → keep separate
