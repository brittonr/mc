# Stevenarella block registry separation oracle

## Question
Did `separate-stevenarella-block-registry` separate Stevenarella block registry data from hand-authored runtime behavior while preserving the existing block API claim boundary?

## Inspected evidence
- `clients/stevenarella/AGENTS.md` and `clients/stevenarella/README.md` for client workflow and supported check scope.
- `clients/stevenarella/blocks/src/lib.rs` before the split: crate root mixed `WorldAccess`, `VanillaIDMap`, the block definition macro invocation, material/collision/model helpers, public variant types, and unit tests.
- `docs/evidence/run-logs/2026-06-29/separate-stevenarella-block-registry-baseline-block-tests.run.log` records the pre-edit block baseline.
- `docs/evidence/run-logs/2026-06-29/separate-stevenarella-block-registry-focused-block-tests.run.log` records the post-split positive, negative, and generated-data freshness tests.
- `docs/evidence/run-logs/2026-06-29/separate-stevenarella-block-registry-affected-dry-runs.run.log` records the affected wrapper dry-runs.

## Decision
Adopt the split within the Stevenarella block crate only. Declarative block data now lives in `clients/stevenarella/blocks/src/runtime/generated.rs`, `VanillaIDMap` lookup behavior lives in `clients/stevenarella/blocks/src/id_map.rs`, hand-authored runtime helpers and public support types live in `clients/stevenarella/blocks/src/runtime.rs`, and crate-root compatibility re-exports remain in `clients/stevenarella/blocks/src/lib.rs`.

The checked-in declarative block data has no external generator in this change. It is treated as snapshot-owned data and guarded by `generated::check_generated_data_freshness`, with a fail-closed stale-sample test.

## Owner
Stevenarella block crate (`clients/stevenarella/blocks`). No Hyperion or Valence code/concepts were adopted, ported, or used as reference for this change.

## Next action
Archive only after Cairn gates, task-evidence checks, evidence manifests, sync/archive dry-runs, sync/archive execution, and final validation pass. This evidence preserves the existing non-claim boundary: registry organization and safety only, not broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, or full CTF/survival correctness.
