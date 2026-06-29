# Modularize Hyperion simulation shell evidence

## Question

Can `hyperion/crates/hyperion/src/simulation/mod.rs` be split into focused Hyperion-owned modules without promoting Valence adoption or broad compatibility claims?

## Inspected evidence

- Change package: `cairn/changes/modularize-hyperion-simulation-shell/` proposal, design, tasks, and `specs/valence-hyperion-integration/spec.md`.
- Workspace guardrails: `AGENTS.md`, `README.md`, `docs/check-tiers.md`, `docs/hyperion-integration-boundaries.md`, `hyperion/README.md`, `hyperion/CONTRIBUTING.md`, `hyperion/AGENTS.md`, and `hyperion/.agent/napkin.md`.
- Pre-edit Hyperion VCS state: `jj status` from `hyperion/` reported no changes in pueue task 80 before implementation.
- Hyperion implementation commit: `a70294d81498d10597d4ee82f4b786957828c514` (`separate simulation decisions from runtime shells`).
- Baseline tests: `docs/evidence/run-logs/2026-06-29/modularize-hyperion-simulation-shell.baseline-hyperion-simulation-tests-nix.run.log` passed with 28 simulation-filtered tests.
- Final focused checks: `docs/evidence/run-logs/2026-06-29/modularize-hyperion-simulation-shell.post-hyperion-fmt-check.run.log` and `docs/evidence/run-logs/2026-06-29/modularize-hyperion-simulation-shell.post-decision-lint-simulation-tests.run.log`.

## Current simulation shell responsibilities captured

Before the split, `simulation/mod.rs` owned these mixed responsibilities:

- public child module declarations for blocks, handlers, inventory, metadata, packet, packet state, skin, and utilities;
- lookup/resources for stream IDs, player UUIDs, and in-game names;
- domain components and geometry helpers for players, positions, rotations, movement, entity size, chunk position, flight, health, XP, visibility, owners, and teleportation;
- lifecycle observers for player initialization, player removal, and UUID initialization;
- packet-facing adapters for duplicate-name disconnects, pending teleport packets, flight ability packets, proxy egress, and channel packet subscription requests;
- `SimPlugin` system/plugin/event registration;
- diagnostics mixed into lifecycle and packet shell branches.

The implementation split those responsibilities into focused Hyperion-local modules:

- `state.rs`: lookup/resource state;
- `domain.rs`: domain components, geometry, movement, and rotation helpers;
- `decisions.rs`: pure state transition, packet summary, empty-world, and update-plan decisions with positive and negative tests;
- `diagnostics.rs`: deterministic diagnostic message formatting;
- `lifecycle.rs`: Bevy observer shell for player/UUID lifecycle;
- `packet_adapters.rs`: packet emission/proxy-facing shell;
- `registration.rs`: `SimPlugin` observer/plugin/event registration shell;
- `mod.rs`: compatibility re-export surface and child module declarations.

## Decision

This change is Hyperion-owned nested-repo work. The Hyperion source classification for Valence integration is `reference` only with `valence_target=none`; direct Valence `adopt` and `port` are rejected for this change unless a separate accepted integration Cairn reclassifies a specific source. No Valence core behavior, default behavior, optional plugin behavior, mc-compat row, public-server safety claim, production-readiness claim, Hyperion compatibility claim, semantic-equivalence claim, or broad Minecraft compatibility claim is promoted.

The pure core decisions are testable without Bevy world setup, packet I/O, network/proxy state, tracing, or schedule wiring. ECS mutation, packet emission, network/proxy integration, diagnostics, and plugin/event registration remain in shell modules.

## Owner

Cairn change `modularize-hyperion-simulation-shell`.

## Next action

Keep the change scoped to Hyperion. Any future Valence use of Hyperion simulation code or concepts must create a separate integration Cairn with inventory rows from `docs/hyperion-integration-boundaries.md` before implementation.

## Non-claims

This evidence does not claim Valence adoption, production-scale behavior, Hyperion compatibility, mc-compat evidence, broad Minecraft compatibility, semantic equivalence, public-server safety, full CTF correctness, or full survival correctness.

## Triage notes

- Direct host `cargo test -p hyperion simulation:: --lib` failed because the host cargo used stable Rust; `docs/evidence/run-logs/2026-06-29/modularize-hyperion-simulation-shell.baseline-hyperion-simulation-tests.run.log` records that toolchain mismatch.
- `cargo +nightly-2025-02-22 ...` failed because the host cargo is not rustup-managed; `docs/evidence/run-logs/2026-06-29/modularize-hyperion-simulation-shell.baseline-hyperion-simulation-tests-nightly.run.log` records that command-selection mismatch.
- Package-level `cargo clippy -p hyperion --lib -- -D warnings` was attempted after the refactor. After fixing refactor-owned diagnostics, the remaining failures are existing loader-module lints in `simulation/blocks/loader/*`; `docs/evidence/run-logs/2026-06-29/modularize-hyperion-simulation-shell.post-decision-lint-clippy.run.log` records the unrelated remaining diagnostics.
