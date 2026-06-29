# Extract Valence entity attribute core: responsibilities

## Question
What current Valence entity responsibilities are in scope for `extract-valence-entity-attribute-core`, and where should side effects remain?

## Inspected evidence
- `servers/valence/AGENTS.md`, `README.md`, `CONTRIBUTING.md`, and `crates/valence_entity/README.md` define the Valence workflow and entity metadata policy.
- `servers/valence/crates/valence_entity/src/attributes.rs` owns attribute instances, modifier maps, computed values, tracked attribute packet properties, and changed-attribute bookkeeping.
- `servers/valence/crates/valence_entity/src/active_status_effects.rs` owns queued status-effect changes, active effect ordering, duration/expiry behavior, and effect update summaries.
- `servers/valence/crates/valence_entity/src/tracked_data.rs` owns metadata encoding, reserved terminator validation, init/update byte caches, final-value-per-index updates, and fail-closed encoding errors.
- `servers/valence/crates/valence_entity/src/hitbox.rs` owns Bevy schedule wiring plus hitbox sizing/AABB decisions for selected entity state.
- `servers/valence/crates/valence_entity/src/flags.rs` owns generated bit accessors for entity metadata flags.
- `servers/valence/crates/valence_entity/src/query.rs` owns packet-writing shells for entity spawn/update queries and movement/status/animation packet predicates.
- Baseline focused test log: `docs/evidence/run-logs/2026-06-28/extract-valence-entity-attribute-core.baseline-valence-entity-tests.run.log` records `cargo test -p valence_entity --lib` with `exit_status=0`.

## Decision
The owner subtree is `servers/valence/crates/valence_entity`. Attribute math, status-effect insertion/expiry decisions, tracked-data cache transitions, selected hitbox sizing/AABB decisions, flag bit transitions, and query packet predicates are deterministic cores over explicit inputs. Bevy component mutation, schedule wiring, packet writes, warnings/logging, and public component APIs remain in existing shells.

## Non-claims
This evidence only covers the entity-core extraction and focused Valence entity checks. It does not promote broad Minecraft compatibility, full vanilla entity parity, production readiness, public-server safety, full CTF correctness, or full survival correctness.

## Next action
Close the Cairn change only after focused tests, affected Valence checks, Cairn gates, Cairn validation, task-evidence validation, accepted-spec sync, and archive all pass with tracked logs.
