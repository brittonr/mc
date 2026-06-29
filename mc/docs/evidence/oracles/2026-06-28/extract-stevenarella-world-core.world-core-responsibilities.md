# Stevenarella world core responsibility checkpoint

## Question

What world/chunk/dimension responsibilities were present before draining `extract-stevenarella-world-core`, and what boundary should the extraction preserve?

## Inspected evidence

- `clients/stevenarella/AGENTS.md`: Stevenarella is the affected owner subtree; client code changes require focused Cargo tests and affected mc-compat dry-runs.
- `clients/stevenarella/src/world/mod.rs`: before extraction, this file owned dimension-codec NBT traversal, dimension bounds validation, chunk section indexing, palette/biome payload parsing, light packet interpretation, block update neighbor walks, block storage mutation, block-entity side effects, dirty/render invalidation, and diagnostic logging.
- `docs/evidence/run-logs/2026-06-28/extract-stevenarella-world-core.baseline-stevenarella-world-tests.run.log`: baseline `cargo test world::tests -- --nocapture` passed before core logic edits.
- `docs/evidence/run-logs/2026-06-28/extract-stevenarella-world-core.baseline-stevenarella-protocol-tests.run.log`: baseline `cargo test protocol -- --nocapture` passed before core logic edits.

## Decision

Extract deterministic decisions into `clients/stevenarella/src/world/core.rs`: dimension bounds selection/validation, chunk-section layout, palette biome/block payload shape decisions, light payload plans, block update targets, block entity action decisions, and storage write plans.

Keep byte reading, NBT traversal, packet variants, world/chunk/section mutation, render invalidation, and logging in `clients/stevenarella/src/world/mod.rs` shells.

## Owner

`clients/stevenarella/`.

## Next action

Verify the extracted core with focused positive and negative world tests, then run affected protocol and mc-compat dry-run checks before syncing and archiving the Cairn change.
