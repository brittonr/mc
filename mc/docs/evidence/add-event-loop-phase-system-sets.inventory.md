# Add event-loop phase SystemSets inventory

## Scope

This evidence note records the inventory and scope for `add-event-loop-phase-system-sets`. It is review evidence for `r[valence_bevy_ecs.event_loop_phase_sets.inventory]`, `r[valence_bevy_ecs.event_loop_phase_sets.contract]`, `r[valence_bevy_ecs.event_loop_phase_sets.wiring]`, `r[valence_bevy_ecs.event_loop_phase_sets.compatibility]`, `r[valence_bevy_ecs.event_loop_phase_sets.tests]`, and `r[valence_bevy_ecs.event_loop_phase_sets.validation]`.

Hyperion code and concepts were not used for this Valence change; no adopt/port/reference/reject classification is needed beyond this note.

## Current event-loop schedule inventory

| Surface | Current owner | Current behavior before this change |
| --- | --- | --- |
| `RunEventLoop` | `servers/valence/crates/valence_server/src/event/loop.rs` | Inserted after Bevy `PreUpdate`; drains one packet per connected client, emits raw `PacketEvent`, runs child event-loop schedules, and repeats while queued packets remain. |
| `EventLoopPreUpdate` | `EventLoopPlugin` plus packet adapter plugins | Main raw-packet handling schedule. Core adapters include movement, client command/settings, keepalive, resource-pack status, abilities, status, message, custom payload, hand swing, interactions, teleport confirmations, and action packets. |
| `EventLoopUpdate` | `EventLoopPlugin` plus downstream gameplay/examples | Gameplay-facing event-loop phase used by examples and optional observers. No selected core typed adapter is moved here by this change. |
| `EventLoopPostUpdate` | `EventLoopPlugin` plus post-packet plugins | Post-event-loop phase used by status-effect updates and optional diagnostic systems such as observability and anticheat statistics. |
| Raw packet production | `EventLoopPlugin::run_event_loop` | Produces `PacketEvent` before each child schedule pass. Raw `PacketEvent` access remains available to systems in event-loop schedules. |
| Cleanup systems | No selected concrete event-loop cleanup owner | No cleanup set is wired because this change found no concrete event-loop cleanup system in scope. Future cleanup work must add a concrete owner and tests before claiming that phase. |

## Phase-set contract

The event-loop contract adds named `EventLoopSet` variants where concrete ordering phases exist:

| Phase set | Configured schedule(s) | Stable ordering boundary |
| --- | --- | --- |
| `RawPacketObservers` | `EventLoopPreUpdate` | Runs before selected typed adapters for plugins that need deterministic raw `PacketEvent` observation. |
| `TypedAdapters` | `EventLoopPreUpdate` | Runs after raw observers and before selected domain consumers. Adapter systems own only the typed semantics they emit. |
| `DomainConsumers` | `EventLoopPreUpdate`, `EventLoopUpdate` | Runs after selected typed adapters in `EventLoopPreUpdate` and before diagnostics where configured. |
| `Diagnostics` | `EventLoopPreUpdate`, `EventLoopUpdate`, `EventLoopPostUpdate` | Runs after selected domain consumers where configured and is intended for metrics, spans, advisory observations, or diagnostics. |

The stable contract is set-to-set ordering. Ordering inside a set, ordering for systems outside these sets, and private plugin internals remain private.

## Selected wiring classification

| System | File | Classification | New set |
| --- | --- | --- | --- |
| `emit_player_action_events` | `servers/valence/crates/valence_server/src/action.rs` | Typed adapter from raw `PlayerActionC2s` packet to `PlayerActionEvent`. | `EventLoopSet::TypedAdapters` |
| `handle_player_action` | `servers/valence/crates/valence_server/src/action.rs` | Domain consumer of `PlayerActionEvent` that updates `ActionSequence` and emits `DiggingEvent`. | `EventLoopSet::DomainConsumers` |
| `emit_event_loop_pre_update_phase` | `servers/valence/crates/valence_server/src/observability.rs` | Optional diagnostic tick-phase event. | `EventLoopSet::Diagnostics` |
| `emit_event_loop_update_phase` | `servers/valence/crates/valence_server/src/observability.rs` | Optional diagnostic tick-phase event. | `EventLoopSet::Diagnostics` |
| `emit_event_loop_post_update_phase` | `servers/valence/crates/valence_server/src/observability.rs` | Optional diagnostic tick-phase event. | `EventLoopSet::Diagnostics` |
| `emit_network_packet_records` | `servers/valence/crates/valence_server/src/observability.rs` | Optional diagnostic packet counter that drains raw packet observations according to its existing disabled-mode contract. | `EventLoopSet::Diagnostics` |
| `sample_anticheat_statistics` | `servers/valence/crates/valence_server/src/anticheat.rs` | Optional advisory diagnostic sampler for packet cadence and movement metrics. | `EventLoopSet::Diagnostics` |

## Compatibility and private boundaries

Raw `PacketEvent` access is preserved. The action adapter test installs a raw observer in `EventLoopSet::RawPacketObservers` and verifies it reads the same packet while the typed adapter still emits `PlayerActionEvent`. Selected typed and diagnostic systems keep their existing schedules, event resources, and plugin opt-in behavior.

This change does not remove raw packet access, does not promote every packet semantic to a typed event, does not change network transport, and does not change default plugin membership. No broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, or full survival correctness claim is made.

## Focused tests and negative fixtures

The focused test plan covers:

- positive phase-order constants for `EventLoopPreUpdate`, `EventLoopUpdate`, and `EventLoopPostUpdate`;
- negative missing phase, ambiguous phase order, and duplicate phase fixtures in the event-loop phase-plan core;
- positive schedule graph coverage for `RawPacketObservers`, `TypedAdapters`, `DomainConsumers`, and `Diagnostics` with selected systems installed;
- negative missing-set fixture in the schedule graph assertion helper;
- negative disabled `ActionPlugin` fixture proving selected action adapter systems and events are absent when the plugin is disabled;
- raw-access preservation test proving a raw observer and typed adapter can both read the same `PacketEvent`;
- duplicate typed-action event fixture proving duplicate semantic emissions are detected by the focused action test helper;
- existing observability and anticheat tests showing diagnostics behavior remains compatible.

## Validation evidence plan

Task closeout should cite promoted logs under `docs/evidence/` for the baseline, focused Valence tests, schedule hygiene self-test/current-tree check, Valence formatting, Cairn gates, Cairn validation, task-evidence validation, sync/archive receipts, and BLAKE3 manifest freshness. Selected mc-compat live rails are not required because fixture input handling and live protocol behavior are not changed.
