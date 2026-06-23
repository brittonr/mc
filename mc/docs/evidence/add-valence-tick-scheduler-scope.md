# Add Valence tick scheduler scope and integration boundary

## Question

Should Valence add a tick-keyed scheduler utility, and which Hyperion concepts may inform it?

## Inspected evidence

| Source | Classification | Owner | Valence target | Decision | Safety notes | Evidence required | Non-claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/crates/hyperion-scheduled/src/lib.rs` (`Scheduled<K, V>`) | `port` | `add-valence-tick-scheduler` | `servers/valence/crates/valence_server/src/tick_scheduler.rs` | Port the min-heap idea into Valence-owned API and tests. Do not copy the implementation unchanged because Valence needs documented same-tick ordering, cancellation/clear behavior, and a Bevy event shell. | Source is stable Rust and pure data-structure logic; Valence implementation must avoid unsafe, clocks, I/O, async runtime coupling, and Hyperion runtime assumptions. | Pure scheduler tests for ordering, not-due retention, clear/cancel, and overflow boundaries; plugin-disabled and plugin-smoke checks. | No Hyperion compatibility, runtime replacement, production-scale performance, vanilla parity, async task runtime, or wall-clock timer claim. |
| `hyperion/crates/hyperion/src/common/mod.rs` (`Global::tick`) | `reference` | `add-valence-tick-scheduler` | `valence_server::Server::current_tick` integration notes | Use only as vocabulary evidence that gameplay systems key delays by explicit ticks. | No code copied; Hyperion uses an `i64` global tick while Valence already exposes `Server::current_tick()`. | Valence shell tests must prove explicit current-tick drain behavior without changing defaults. | No claim that Valence and Hyperion ticks are semantically equivalent. |
| `hyperion/events/bedwars/src/plugin/chat.rs` (`ChatCooldown`) | `reference` | `add-valence-tick-scheduler` | docs for cooldown examples | Use as a gameplay use-case showing expiry ticks for cooldowns. | Bedwars/gameplay policy is not copied; Valence scheduler remains a generic utility. | Documentation must keep cooldown policy outside the scheduler core. | No Bedwars behavior, chat policy, or anti-spam correctness claim. |
| `servers/valence/crates/valence_server_common/src/lib.rs` (`Server`, `ServerPlugin`, `DEFAULT_TPS`) | Valence-owned | `add-valence-tick-scheduler` | `valence_server::tick_scheduler` shell | Use `Server::current_tick()` only in the thin Bevy shell. Core accepts explicit keys and never reads server state. | Existing tick resource is stable Valence API. Scheduler plugin must not be part of `DefaultPlugins` unless separately scoped. | Plugin-disabled test must show default apps do not install scheduler resources/events. | No default behavior change. |
| `servers/valence/examples/*` and `servers/valence/src/tests/*` tick patterns | Valence-owned | `add-valence-tick-scheduler` | module docs and tests | Existing patterns use `current_tick()`, `remaining_ticks`, or ad hoc modulo checks for animation/cooldowns. Scheduler docs will frame cooldowns, despawns, and temporary blocks as optional user policy. | Examples stay documentation-only unless a runnable example is changed. | Focused scheduler tests and docs checks are enough unless examples are modified. | No broad gameplay correctness or compatibility claim. |

## Decision

Port the Hyperion min-heap scheduling concept into a Valence-owned `tick_scheduler` module. The functional core is a deterministic `TickScheduler<K, V>` over explicit ordered keys, stable insertion-order tie breaking, clear/cancel operations, and an explicit sequence-overflow error. The imperative shell is optional: `TickSchedulerPlugin<E>` registers a `ServerTickScheduler<E>` resource and emits due work as Bevy events when the plugin is added by user code.

Default Valence behavior remains unchanged because the scheduler plugin is not added to `DefaultPlugins`. Gameplay policy such as cooldown duration, despawn targets, and temporary block restoration remains outside the scheduler.

## Owner

`add-valence-tick-scheduler`

## Next action

Implement `valence_server::tick_scheduler`, add pure positive/negative tests plus optional plugin smoke tests, update Valence docs, and capture focused validation logs under `docs/evidence/`.
