# Proposal: Valence compat bot probe

## Why

Valence currently has smoke receipts around examples, but the next useful Hyperion import is the `rust-mc-bot` testing pattern: a bounded client-side probe that can status/login/render and later scale into load checks without depending on a human client.

## What Changes

- Add a Valence-owned compatibility bot/probe surface or runner integration that reuses the narrow Hyperion bot idea without copying unsafe public stress-tool behavior.
- Emit deterministic receipts with explicit scenario, client milestones, target address, bounded run window, and non-claims.
- Wire the probe into the existing Valence/parent mc harness so `parkour` or `ctf` can be checked with real client-side evidence.
- Add focused tests and a dry-run Nix gate for receipt shape.

## Impact

- **Files**: `valence/` test tooling or parent `tools/mc-compat-runner`, `mc/flake.nix`, and `mc/docs/evidence/` when live evidence is recorded.
- **Testing**: Rust tests for receipt/probe parsing, dry-run Nix receipt gate, then one bounded live Valence example receipt when prerequisites are present.
