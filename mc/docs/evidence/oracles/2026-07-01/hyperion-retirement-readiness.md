# Hyperion checkout retirement readiness

## Question

Can the local `hyperion/` checkout be removed now that selected integration surfaces have been promoted into `servers/valence/`?

## Inspected evidence

- Audit log: `docs/evidence/run-logs/2026-07-01/hyperion-retirement-readiness.audit.run.log`.
- Parent tracking check: `git -C /home/brittonr/git ls-files -- mc/hyperion` reported `parent_hyperion_tracked_count=0`.
- Nested checkout check: `hyperion/.git` and `hyperion/.jj` are present.
- Nested status check: `jj status` in `hyperion/` reported working-copy changes in `crates/hyperion-game-modes/src/composition.rs` and `events/bedwars/src/lib.rs`.
- Reference audit: first-order references outside `hyperion/` reported `first_order_reference_file_count=627`.
- Accepted spec audit: accepted Cairn specs reported `accepted_spec_reference_file_count=5`.
- Layout/config blockers include `AGENTS.md`, `docs/architecture.md`, `docs/layout-checklist.md`, and `compat/config/component-registry.ncl`, all of which still classify `hyperion/` as an independent nested repo.

## Decision

Do not delete `hyperion/` in this change.

The selected Valence integration work promoted contracts, receipts, metadata, and examples, but it did not retire all Hyperion-owned behavior, evidence, accepted requirements, docs, or local nested-repo state. Because the parent repository does not track `mc/hyperion`, deleting the directory would be a local workspace destruction rather than a reviewable parent-repo diff. Because the nested checkout currently has working-copy changes, deletion would also discard uncommitted Hyperion work.

## Owner

Future Hyperion retirement work should be owned by a dedicated Cairn change that first proves all live references are intentionally migrated, rejected, or no longer required.

## Next action

Keep `hyperion/` in place. Before any future physical deletion, satisfy a retirement gate that proves: no accepted spec requires Hyperion-local behavior; docs/config no longer classify it as a supported nested repo; promoted evidence no longer cites live Hyperion paths as required review inputs; the nested checkout has no uncommitted work or has been explicitly backed up; and the deletion mechanism is intentionally local or represented by a tracked parent-repo change.
