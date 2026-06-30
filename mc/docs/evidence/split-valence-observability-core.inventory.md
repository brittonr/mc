# Split Valence observability core inventory

## Question

What responsibilities did `valence_server::observability` own before the split, what owner modules own them after the split, and what checks bound the parity claim for `split-valence-observability-core`?

## Owner subtree

- `servers/valence/crates/valence_server/src/observability.rs`
- `servers/valence/crates/valence_server/src/observability/*.rs`

No Hyperion code or concepts were used for this Valence-only change; no adopt/port/reference/reject Hyperion classification was required.

## Requirement IDs

- `r[valence_bevy_ecs.observability_core.boundaries]`
- `r[valence_bevy_ecs.observability_core.core]`
- `r[valence_bevy_ecs.observability_core.parity]`
- `r[valence_bevy_ecs.observability_core.positive_tests]`
- `r[valence_bevy_ecs.observability_core.negative_tests]`
- `r[valence_bevy_ecs.observability_core.validation]`

## Responsibility ownership after split

| Responsibility | Owner module | Boundary |
| --- | --- | --- |
| Runtime enable flags and config predicates | `observability/config.rs` | Pure config state; no Bevy schedule wiring beyond `Resource` derive. |
| Stable metric names, record kinds, record shape, and tick-phase classification | `observability/taxonomy.rs` | Pure taxonomy and span classification over explicit phase input. |
| Bounded label enums, label validation, sensitive field categories, and redaction output | `observability/labels.rs` | Pure label/redaction decisions; no exporter or Bevy side effects. |
| Packet ID class and serverbound packet record classification | `observability/packets.rs` | Pure packet classification over explicit packet ID and payload reference; raw payload is omitted from labels. |
| Exporter failure taxonomy, export attempt planning, and exporter result classification | `observability/export.rs` | Pure plan/outcome helpers plus a thin exporter shell that calls the adapter once. |
| Bevy event type and event-emitting systems | `observability/events.rs` | Bevy shell only: reads resources/events, drains disabled packet readers, emits `ObservabilityEvent`. |
| Plugin schedule wiring and public re-exports | `observability.rs` | Thin root adapter that preserves public API and schedule placement. |

## Dependency order

1. Boundaries and docs read.
2. Focused baseline: Cairn gates/validation, observability tests, schedule hygiene.
3. Module split and pure-core extraction.
4. Positive/negative observability tests.
5. Focused Valence and schedule validation.
6. Cairn sync/archive validation.

## Acceptance checks

- `docs/evidence/split-valence-observability-core.pre-gates.run.log`
- `docs/evidence/split-valence-observability-core.valence-observability-baseline.run.log`
- `docs/evidence/split-valence-observability-core.schedule-hygiene-baseline.run.log`
- `docs/evidence/split-valence-observability-core.valence-observability-after-split.run.log`
- `docs/evidence/split-valence-observability-core.schedule-hygiene-after-split.run.log`

## Non-claims preserved

This is an observability architecture split only. It does not promote broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, or full survival correctness claims.
