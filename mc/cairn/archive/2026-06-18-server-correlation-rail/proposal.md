## Why

Resource-pack status and sign-editor rows now have bounded Stevenarella drivers, but both remain fixture-bounded blockers because no maintained server-correlation rail verifies that a local backend observed the expected client response. Without a reusable correlation contract, each future live promotion risks one-off receipts, inconsistent non-claims, and overbroad packet-row claims.

## What Changes

- Add a deterministic server-correlation receipt contract for owned-local Valence/Paper rails.
- Add a Rust checker that validates positive and negative receipt fixtures for resource-pack status and sign-editor update correlation.
- Wire the checker into the flake checks so future promotions must carry a maintained server-correlation receipt before changing live status.
- Record reviewable evidence and keep current resource-pack/sign-editor packet rows blocked until real live receipts exist.

## Impact

- **Files**: `tools/check_server_correlation_receipts.rs`, `flake.nix`, `docs/evidence/**`, and `cairn/specs/mc-compatibility/spec.md`.
- **Testing**: checker self-tests, fixture checks, targeted-packet nonpromotion checks, evidence-manifest/task-evidence checks, Cairn gates, and final validation.
