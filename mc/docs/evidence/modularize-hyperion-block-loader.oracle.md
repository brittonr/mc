# Modularize Hyperion block loader evidence checkpoint

## Question

Does `modularize-hyperion-block-loader` stay Hyperion-owned while separating testable loader cores from runtime shell behavior?

## Inspected evidence

- Change package: `cairn/changes/modularize-hyperion-block-loader/{proposal.md,design.md,tasks.md}` and the `valence-hyperion-integration` change spec.
- Workspace guidance: root `AGENTS.md`, `README.md`, `docs/check-tiers.md`, `docs/hyperion-integration-boundaries.md`, plus Hyperion `README.md`, `CONTRIBUTING.md`, `AGENTS.md`, and `.agent/napkin.md`.
- Hyperion nested repo state before edits: `jj status` reported no Hyperion working-copy changes.
- Pre-change baseline: `docs/evidence/run-logs/2026-06-29/modularize-hyperion-block-loader.baseline-hyperion-loader-tests-nix.run.log` passed the existing loader tests from the Hyperion root. The direct host-cargo attempt in `docs/evidence/run-logs/2026-06-29/modularize-hyperion-block-loader.baseline-hyperion-loader-tests.run.log` failed before crate tests because host stable Cargo ignored the pinned nightly toolchain; the devshell rerun is the accepted baseline.
- Post-change checks: final Hyperion loader tests, Hyperion formatting, and the pure-core shell-boundary grep logs under `docs/evidence/run-logs/2026-06-29/`.
- Nested Hyperion implementation commit: `41ccc1e642f6` (`split block loader decisions from runtime shell`).

## Current block-loader responsibilities captured

- Runtime shell responsibilities: Tokio load scheduling, duplicate request suppression, region resource lookup, chunk decompression buffers, raw Anvil chunk reads, packet encoding/compression, trace/warn logging, and channel sends.
- Parse responsibilities: NBT section extraction, section-Y validation, block and biome palette conversion, packed palette decoding, light array validation, block entity extraction, and `ColumnData` population.
- Pure planning responsibilities after the change: load-request decisions, missing-region and chunk-height decisions, parse summaries, packed-palette plans, section-Y validation, palette length validation, and storage update plans.

## Hyperion-to-Valence classification

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Hyperion block-loader modules under the nested Hyperion repo | reference | `modularize-hyperion-block-loader` | The change is implemented and validated inside Hyperion only; it does not copy loader behavior into Valence or change Valence defaults. | none | Nightly/Hyperion runtime code remains in Hyperion; pure planning functions are tested in place; IO/runtime effects remain in the shell module. | Hyperion focused loader tests and shell-boundary grep logs under `docs/evidence/run-logs/2026-06-29/`. | No Valence adoption, no broad Minecraft compatibility claim, no semantic equivalence claim beyond supported Hyperion loader behavior, no production-readiness/public-server-safety/full CTF or survival correctness claim. |

## Decision

The implementation is Hyperion-local. The accepted scope remains modularity and testability of the nested Hyperion block loader; no Valence integration, compatibility evidence, or production-readiness claim is promoted.

## Owner

Cairn change `modularize-hyperion-block-loader`.

## Next action

Use the focused Hyperion test, formatting, shell-boundary, Cairn gate, task-evidence, sync, archive, and validation logs as closeout evidence. Keep non-claims intact unless a separate accepted aggregate gate promotes them.
