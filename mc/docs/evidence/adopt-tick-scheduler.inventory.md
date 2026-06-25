# Adopt tick scheduler for Valence gameplay delays — inventory and classification

## Scope and non-claims

This inventory covers selected Valence example and fixture timing behavior inspected for `adopt-tick-scheduler-for-valence-gameplay-delays`.
No Hyperion code, design, or behavior was used. The only migrated behavior in this change is the standalone `servers/valence/examples/combat.rs` attack cooldown.
CTF compatibility fixture timing remains unchanged; no broad Minecraft compatibility, vanilla parity, production readiness, public-server safety, full CTF correctness, or full survival correctness claim is made.

## Selected timing behavior

| Path / owner | Current tick source | Due condition | Mutation target | Cancellation / stale behavior | Classification | Decision and evidence impact |
| --- | --- | --- | --- | --- | --- | --- |
| `servers/valence/examples/combat.rs` attack cooldown | `Server::current_tick()` | Victim may be attacked again after `ATTACK_COOLDOWN_TICKS` | Victim `CombatState` and knockback/sound/status mutation | Scheduler event carries victim entity plus generation; missing target, duplicate event, or stale generation fails closed | Scheduler-suitable | Migrated to `ServerTickScheduler<AttackCooldownExpired>` owned by a local opt-in plugin. Positive due-work and negative cancellation, stale target, duplicate event, invalid tick, and plugin-disabled tests live in `cargo test --example combat`. |
| `servers/valence/examples/ctf.rs` combat cooldown | `Server::current_tick()` | Victim may be attacked again after the CTF combat window | CTF player state, combat milestones, flag/resource side effects | Existing state is tied to compatibility probes and score/flag milestone order | Intentionally custom policy for this change | Left unchanged to preserve CTF fixture receipt semantics and non-claims. A future CTF-specific change would need selected mc-compat rails before migration. |
| `servers/valence/examples/game_of_life.rs` board update cadence | `Server::current_tick()` modulo cadence | Board updates every selected simulation cadence | Board cells and chunk blocks | Continuous simulation cadence, not one-shot delayed work | Intentionally custom policy | Left unchanged; scheduler would not improve cancellation/stale-target behavior. |
| `servers/valence/examples/player_list.rs` display/despawn cadence | `Server::current_tick()` modulo cadence | Demo display changes and entry despawn happen on recurring ticks | Player list display names and demo entries | Recurring demonstration state, not isolated delayed cleanup | Intentionally custom policy | Left unchanged; migration would change demo pacing without correctness benefit. |
| `servers/valence/examples/particles.rs` particle cadence | `Server::current_tick()` modulo cadence | Particle demo emits every selected cadence | Client particle packets and local index | Continuous demo cycling; index wrap is local state | Intentionally custom policy | Left unchanged; not explicit one-shot delayed work. |
| `servers/valence/examples/equipment.rs` random equipment cadence | `Server::current_tick()` and configured tick rate | Random equipment refresh once per server-rate interval | Equipment inventory sync | Recurring demo policy | Intentionally custom policy | Left unchanged; scheduler would only re-encode recurring cadence. |
| `servers/valence/examples/parkour.rs` combo window | `SystemTime` wall-clock milliseconds | Combo reset depends on elapsed wall-clock time and path progress | Per-client score/combo/block state | Wall-clock measurement and random block generation | Wall-clock / unsuitable | Left outside `TickScheduler`, which is explicitly tick-keyed and not wall-clock/async scheduling. |
| `servers/valence/tools/stresser` spawn cooldown | Tokio wall-clock sleep | Sessions spawn after async sleep intervals | External client sessions | Async/background load tooling | Wall-clock or async work | Left outside Valence tick scheduler. |
| `servers/valence/crates/valence_server/src/tick_scheduler.rs` core/plugin | Caller-supplied key or `Server::current_tick()` in optional shell | Work drains at or before explicit key | Scheduled values or Bevy events | Caller owns cancellation/stale validation | Existing scheduler core | No scheduler-core policy was added; combat policy values remain in the combat example shell. |

## Requirement mapping

- r[valence_bevy_ecs.scheduler_adoption.inventory]: the table records selected owner, tick source, due condition, mutation target, cancellation/stale behavior, classification, and evidence impact.
- r[valence_bevy_ecs.scheduler_adoption.classification]: each selected behavior is classified before migration.
- r[valence_bevy_ecs.scheduler_adoption.policy]: migrated policy duration remains `ATTACK_COOLDOWN_TICKS` in `examples/combat.rs`; the scheduler stores and drains typed work only.
