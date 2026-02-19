# Skill: Build and Test

## How It Works

Generated `.rs` files are **committed to git**. There is no `build.rs` — codegen is a separate step using `tools/codegen/`.

## Codegen (only when adding new ABIs)

```bash
# Generate bindings for a specific protocol
cd tools/codegen && cargo run -- ../../abi/dex/newprotocol/

# Generate bindings for a single ABI file
cd tools/codegen && cargo run -- ../../abi/dex/newprotocol/Router.json

# Regenerate ALL bindings (slow, rarely needed)
cd tools/codegen && cargo run
```

## Build

```bash
cargo build
```

Compiles the library from the already-generated `.rs` files. No codegen happens during build.

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
**Fix:** The module name in `mod.rs` must match the generated `.rs` filename (lowercased file stem). Check what the codegen tool generated in `src/`.

### Rust keyword collision
```
error: expected identifier, found keyword `yield`
```
**Fix:** Use raw identifier: `pub mod r#yield;`

## Toolchain

- **Rust version:** 1.85 (specified in `rust-toolchain.toml`)
- **Target:** `wasm32-unknown-unknown` (for Substreams)
- **CI:** Single job — test + wasm32 check

## Before Pushing

Always run:
1. `cargo build` — verifies compilation
2. `cargo test` — verifies decoding works
3. `cargo check --target wasm32-unknown-unknown` — verifies WASM compatibility
