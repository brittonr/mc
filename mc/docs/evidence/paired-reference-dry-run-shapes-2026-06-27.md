# Paired-reference dry-run shapes evidence

## Question
Can `vanilla-combat-reference-parity` and `vanilla-combat-armor-reference-parity` expose deterministic dry-run receipt shapes without promoting live Paper/Valence parity claims?

## Inspected evidence
- `compat/config/scenario-manifest.ncl` now assigns both paired-reference rows to `mc-compat-paired-reference-dry-run-shapes` with `receipt_shape_check = true`.
- `compat/runner/src/receipts.rs` writes a `paired_reference_dry_run_shape` block only for selected dry-run paired-reference scenarios.
- `tools/check_paired_reference_dry_run_shapes.rs` validates normalized receipt shapes with positive fixtures for both scenarios and negative fixtures for invalid backend fields, missing tolerance fields, wrong backend labels, and overbroad live-parity claims.
- Generated rows in `docs/evidence/mc-compat-scenario-index.generated.md` and `docs/scenario-commands.generated.md` name `mc-compat-paired-reference-dry-run-shapes` for both paired-reference scenarios.

## Decision
The dry-run shape contract is reviewable and deterministic. It records scenario identity, `paper-reference`, `valence`, dry-run revision placeholders, metric names, tolerance fields, `dry-run-shape-not-compared`, and non-claim strings.

## Non-claims
This evidence is shape-only dry-run coverage. It does not claim live Paper/Valence comparator success, exact Mojang/vanilla parity, full combat parity, full armor parity, public-server safety, production readiness, broad Minecraft compatibility, full CTF correctness, or full survival correctness. Live paired comparator receipts remain the parity promotion source.

## Validation
- `docs/evidence/run-logs/2026-06-27/paired-reference-dry-run-shapes-implementation.run.log` records formatter coverage for non-generated touched Rust, checker self-test fixtures, and the focused runner receipt test.
- `docs/evidence/run-logs/2026-06-27/paired-reference-dry-run-shapes-nix.run.log` records the focused Nix checks for the new dry-run shape check, scenario manifest, and generated-surface freshness.
