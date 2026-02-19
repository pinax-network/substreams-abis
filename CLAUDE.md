# substreams-abis

Rust library of auto-generated ABI bindings for EVM smart contracts, used in Substreams.

## Agent Instructions

See **[.agents/skills/](.agents/skills/)** for all workflows and conventions.

## Quick Start

```bash
cargo build                                    # Generate .rs from ABI JSONs + compile
cargo test                                     # Run tests
cargo check --target wasm32-unknown-unknown     # Verify WASM compatibility
```
