# Entity metadata diff tracking audit

## Question

Does Valence need Hyperion-style entity metadata diff tracking changes before the `audit-entity-metadata-diff-tracking` Cairn can archive?

## Requirement IDs

- `r[valence_hyperion_integration.metadata_diff.audit]`
- `r[valence_hyperion_integration.metadata_diff.invariants]`
- `r[valence_hyperion_integration.metadata_diff.core]`
- `r[valence_hyperion_integration.metadata_diff.tests]`
- `r[valence_hyperion_integration.metadata_diff.wiring]`
- `r[valence_hyperion_integration.metadata_diff.validation]`

## Inspected Hyperion inventory

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/crates/hyperion/src/simulation/metadata/mod.rs` (`MetadataChanges`, `component_and_track`, `encode_non_default_components`) | reference | `audit-entity-metadata-diff-tracking` | Hyperion shows a component/previous-value diff model and non-default spawn metadata encoding, but Valence generated metadata remains the source of truth. | `servers/valence/crates/valence_entity/src/tracked_data.rs`, tests, docs | No code copied. Hyperion uses its own Bevy schedule, `Prev<T>`, nightly crate context, and Valence protocol wrappers. | `docs/evidence/audit-entity-metadata-diff-tracking-valence-entity-lib-tests.run.log` | No Hyperion compatibility, production-scale, or vanilla parity claim. |
| `hyperion/crates/hyperion/src/egress/sync_entity_state.rs` (`entity_metadata_sync`) | reference | `audit-entity-metadata-diff-tracking` | Hyperion clears `MetadataChanges` through an egress-only view after broadcasting. Valence already has `ClearEntityChangesSet` and `clear_tracked_data_changes`; the audit added stale-update cleanup coverage. | `servers/valence/crates/valence_entity/src/lib.rs`, `tracked_data.rs` tests | No runtime, networking, or compose behavior copied. | `docs/evidence/audit-entity-metadata-diff-tracking-valence-entity-lib-tests.run.log` | No default runtime replacement or broad packet-delivery claim. |
| `hyperion/crates/hyperion/src/egress/channel.rs` spawn subscription metadata path | reference | `audit-entity-metadata-diff-tracking` | Hyperion sends non-default metadata after entity subscription/spawn bytes. Valence already writes spawn packets before initialization metadata; the audit added packet ordering coverage. | `servers/valence/crates/valence_entity/src/query.rs` test | No packet encoder or channel code copied. | `docs/evidence/audit-entity-metadata-diff-tracking-valence-entity-lib-tests.run.log` | No client-visible vanilla parity or full entity behavior claim. |

## Inspected Valence paths

- `servers/valence/crates/valence_entity/build.rs`: generated metadata systems update `TrackedData` from changed generated metadata components.
- `servers/valence/crates/valence_entity/src/tracked_data.rs`: packet-ready init/update metadata byte caches.
- `servers/valence/crates/valence_entity/src/query.rs`: spawn/update packet ordering for non-client entities.
- `servers/valence/crates/valence_entity/src/lib.rs`: `UpdateTrackedDataSet`, `ClearEntityChangesSet`, and despawned entity-manager cleanup.
- `servers/valence/crates/valence_server/src/client.rs`: player/client self-entity tracked-data init/update packet shell.

## Audit decision

Valence already has a generated metadata source of truth and explicit ECS shells for spawn initialization, incremental updates, and clearing queued update data. The audit found three bounded gaps in `TrackedData` itself and selected Valence-owned fixes:

1. Repeated same-index updates in one flush window could queue duplicate metadata entries through the public `append_update_value` helper. The implementation now replaces the earlier queued entry and emits the final encoded value once per index.
2. Index `0xff` was only protected by `debug_assert!`, which does not fail closed in release builds. The implementation now validates the reserved terminator index before mutating bytes and exposes `try_*` helpers with deterministic errors.
3. Value encoding failures could leave partially appended bytes. The implementation now encodes into a scratch entry first and mutates packet caches only after successful encoding.

No Valence generated metadata definitions were forked, no Hyperion code was copied, and no Valence default entity type behavior was broadened beyond the tracked-data byte-cache fixes above.

## Invariants

- Unchanged/default `TrackedData` emits no spawn or incremental metadata packet bytes.
- Non-default spawn metadata is present in initialization metadata and is sent after the spawn packet.
- Defaults are suppressed from spawn initialization by removing that metadata index from `init_data`.
- A visible-client reset to a default value is still emitted as an incremental update.
- Same-tick/same-flush repeated updates for one metadata index emit the final encoded value once.
- Invalid index `0xff` fails closed before byte-cache mutation.
- Encoding failures fail closed before byte-cache mutation.
- `clear_update_values` removes queued incremental update bytes and update-entry bookkeeping.
- Despawned entity IDs are removed from `EntityManager` so stale metadata is not associated with an old protocol ID.
- Spawn packet ordering remains spawn first, initialization metadata second.

## Tests and evidence

- Pre-implementation Cairn gates passed: `docs/evidence/audit-entity-metadata-diff-tracking-pre-gate-proposal.run.log`, `docs/evidence/audit-entity-metadata-diff-tracking-pre-gate-design.run.log`, `docs/evidence/audit-entity-metadata-diff-tracking-pre-gate-tasks.run.log`, and `docs/evidence/audit-entity-metadata-diff-tracking-pre-validate.run.log`.
- Smallest relevant baseline initially failed before metadata edits because `valence_entity` tests used `Uuid::new_v4()` without the `uuid/v4` feature: `docs/evidence/audit-entity-metadata-diff-tracking-baseline-valence-entity-tracked-data.run.log`.
- Deterministic UUID fixture repair made the focused baseline pass before metadata core edits: `docs/evidence/audit-entity-metadata-diff-tracking-baseline-after-uuid-repair.run.log`.
- Focused component verification after implementation passed: `docs/evidence/audit-entity-metadata-diff-tracking-valence-entity-lib-tests.run.log` and `docs/evidence/audit-entity-metadata-diff-tracking-valence-entity-lib-tests-after-fmt.run.log`.
- Package-level `valence_entity` tests, including metadata positive/negative fixtures, packet-order fixture, despawn cleanup, and deterministic UUID repair, passed: `docs/evidence/audit-entity-metadata-diff-tracking-valence-entity-package-tests.run.log`.
- Focused no-deps clippy passed for the edited crate: `docs/evidence/audit-entity-metadata-diff-tracking-valence-entity-clippy.run.log`.
- Selected compatibility-shape check passed as a dry run without live gameplay claims: `docs/evidence/audit-entity-metadata-diff-tracking-valence-smoke-dry-run.run.log`.
- Formatting passed through the available devshell cargo: `docs/evidence/audit-entity-metadata-diff-tracking-valence-fmt-rerun.run.log`; the first `cargo +nightly fmt` attempt failed because this shell exposes cargo directly rather than rustup toolchain shims: `docs/evidence/audit-entity-metadata-diff-tracking-valence-fmt.run.log`.

## Non-claims

This audit does not claim full vanilla entity behavior parity, broad Minecraft compatibility, semantic equivalence with Hyperion, production readiness, public-server safety, or new entity-type support. The change is limited to Valence-owned metadata byte-cache invariants, packet ordering coverage, and reviewable Cairn evidence.
