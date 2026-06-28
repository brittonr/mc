# Proposal: Extract Hyperion bot packet functional core

## Why

`hyperion/tools/rust-mc-bot/src/packet_utils.rs` concentrates bot packet helpers used for load/testing workflows. Packet construction and classification should be deterministic cores separate from bot network state and IO, especially because the bot is a helper tool rather than Valence-owned code.

## What Changes

- In Hyperion's bot tool, split packet utilities into pure packet construction/classification cores and thin network/bot-state shells.
- Keep socket IO, async runtime, connection state, logging, and timing in shells.
- Preserve bot tool CLI/API behavior, packet bytes, protocol assumptions, and non-claims.
- Add positive and negative tests for packet utility cores and malformed packet inputs.

## Impact

- **Files**: Hyperion `tools/rust-mc-bot` modules, Hyperion bot tests, optional parent evidence notes, and Cairn artifacts.
- **Testing**: Hyperion baseline/focused bot tests from `hyperion/`, Cairn gates, and Cairn validation.
- **Non-claims**: Hyperion bot tooling architecture only; no Valence adoption, public-server safety, or compatibility evidence claim is promoted.
