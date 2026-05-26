# Proposal: Bounded latency and jitter tolerance compatibility rail

## Why

Current semantic receipts prove local loopback behavior. A bounded latency/jitter wrapper tests whether a proven rail remains compatible under realistic timing variance without jumping to public load claims.

## What Changes

- Add a maintained wrapper or scenario option that runs one existing semantic rail under bounded local network delay/jitter or an equivalent deterministic timing perturbation.
- Record delay parameters in the receipt and keep public/production load claims false.
- Start with the lowest-risk existing rail, likely combat-damage or flag-score, before expanding to more semantics.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, `flake.nix`, possible local network wrapper scripts in Nix/Rust-owned tooling, README, and evidence docs.
- **Testing**: runner tests, dry-run receipt asserting latency fields, one live bounded local receipt if local delay mechanism is available.
