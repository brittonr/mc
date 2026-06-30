# Checker crate migration scoreboard inventory

## Selected wave

- Change: `2026-06-30-migrate-loose-checkers-to-crate`.
- Selected checker: `tools/check_scoreboard_team_packet_family.rs`.
- Reason: the checker was already backed by shared key-value framework helpers, had a direct evidence KV input, and carried positive plus negative self-test fixtures.
- Evidence input: `docs/evidence/scoreboard-team-packet-family-2026-06-06.kv`.
- Stable flake wiring: `nix/checks.nix` compiles `tools/check_scoreboard_team_packet_family.rs` and runs `--self-test` plus the evidence KV; the wrapper path is preserved.
- New crate binary: `check-scoreboard-team-packet-family`.
- Compatibility wrapper: `tools/check_scoreboard_team_packet_family.rs` delegates to `tools/checkers/src/checkers/scoreboard_team_packet_family.rs` through the shared CLI/key-value shell.

## Current crate-migrated checker rows

- `tools/check_block_entity_sign_packet_family.rs` -> `check-block-entity-sign-packet-family`.
- `tools/check_inventory_stack_split_merge_evidence.rs` -> `check-inventory-stack-split-merge-evidence`.
- `tools/check_inventory_drag_transactions_evidence.rs` -> `check-inventory-drag-transactions-evidence`.
- `tools/check_scoreboard_team_packet_family.rs` -> `check-scoreboard-team-packet-family`.

## Remaining loose Rust checker priority

- `tools/check_movement_packet_family.rs` remains the framework-backed Rust debt row. It needs a separate migration because its checker core also validates packet-inventory TSV rows and should receive modeled TSV fixtures instead of an ad hoc filesystem scan.
- Other `tools/check_*.rs` gates remain outside the crate in this wave. Future migration priority is: touched gates first, reusable key-value evidence gates next, then manifest/filesystem-heavy gates after their inputs are modeled as pure core data.

## Legacy Python gate priority

Legacy Python gates listed in `tools/checkers/README.md` remain untouched debt. They must migrate to Rust or Steel before behavior changes, or receive a Cairn waiver with owner, reason, non-claim impact, and next action.

## Non-claim impact

This wave changes checker ownership only. It does not add live compatibility evidence, semantic equivalence, production readiness, public-server safety, full CTF correctness, full survival correctness, broad Minecraft compatibility, or vanilla/reference parity claims.
