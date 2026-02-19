# Skill: Build and Test

## How It Works

`build.rs` uses `substreams-ethereum-abigen` to walk `./abi/` recursively and generate Rust bindings for every `.json` file. Generated `.rs` files go to the parallel path under `./src/`.

## Build

```bash
cargo build
```

This runs `build.rs` first (generates `.rs` from ABI JSONs), then compiles. Takes ~1-2 minutes.

## Test

```bash
cargo test
```

Tests are in `tests/` — they verify that ABI decoding works for specific protocols.

## Wasm Check

```bash
cargo check --target wasm32-unknown-unknown
```

Substreams runs on WASM, so always ensure the crate compiles for `wasm32-unknown-unknown`.

## Common Build Failures

### Missing mod.rs entry
```
error[E0583]: file not found for module `newprotocol`
```
**Fix:** Add `pub mod newprotocol;` to the parent `mod.rs`.

### Module name mismatch
```
error[E0583]: file not found for module `my_contract`
```
**Fix:** The module name in `mod.rs` must match the generated `.rs` filename (lowercased file stem). Check what `build.rs` actually generated in `src/`.

### Rust keyword collision
```
error: expected identifier, found keyword `yield`
```
**Fix:** Use raw identifier: `pub mod r#yield;`

## Toolchain

- **Rust version:** 1.85 (specified in `rust-toolchain.toml`)
- **Target:** `wasm32-unknown-unknown` (for Substreams)
- **CI:** Check + Test (no fmt — generated code isn't formatted)

## Before Pushing

Always run:
1. `cargo build` — verifies ABI generation + compilation
2. `cargo test` — verifies decoding works
3. `cargo check --target wasm32-unknown-unknown` — verifies WASM compatibility
