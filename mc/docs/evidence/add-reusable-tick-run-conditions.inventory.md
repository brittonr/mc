# Add reusable tick run conditions inventory

## Scope

Owner subtree: `servers/valence/`.

Hyperion-to-Valence classification: not applicable. No Hyperion code, behavior, or concepts informed this change.

## Selected periodic behavior inventory

| Candidate | Tick source | Named interval | Previous due condition | Event readers | Mutation target | Classification | Decision and evidence impact |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `examples/particles.rs::manage_particles` | `Server::current_tick()` | `PARTICLE_CADENCE` / `PARTICLE_CADENCE_TICK_COUNT` | `server.current_tick() % 10 == 0` | None | `ChunkLayer::play_particle`, action bar, local particle index | Pure periodic no-op | Migrated to `manage_particles.run_if(every_ticks(PARTICLE_CADENCE))`; not-due ticks skip the body at schedule level. Covered by `add-reusable-tick-run-conditions.changed-examples.final.run.log`. |
| `examples/equipment.rs::randomize_equipment` | `Server::current_tick()` with current `Server::tick_rate()` | `once_per_second()` / current server tick rate | `ticks % server.tick_rate() == 0` | None | Non-client `Equipment` components | Pure periodic no-op | Migrated to `randomize_equipment.run_if(once_per_second())`; tick-rate changes are evaluated from the current `Server` resource. Covered by helper and example logs. |
| `examples/game_of_life.rs::update_board` board advance portion | `Server::current_tick()` | `LIFE_STEP_CADENCE` / `LIFE_STEP_CADENCE_TICK_COUNT` | `board.playing && server.current_tick() % 2 == 0` | None in advance portion | `LifeBoard::update` | Pure periodic no-op after split | Split into `advance_board` and `render_board`; only `advance_board` is gated by `board_is_playing` plus `every_ticks(LIFE_STEP_CADENCE)`. Rendering remains every tick, preserving presentation. Covered by changed-example tests. |
| `examples/game_of_life.rs::toggle_cell_on_dig` and `pause_on_crouch` | Event stream, not tick cadence | Not applicable | Event-reader drains every update | `DiggingEvent`, `SneakEvent` | `LifeBoard`, action bar | Event-reader drain behavior | Left ungated to avoid stale event replay. Covered by helper stale-event-reader test and schedule-hygiene log. |
| `examples/bench_players.rs::print_tick_time` | `Server::current_tick()` plus `Instant::elapsed()` | Existing half-tick-rate expression | `tick % (server.tick_rate() / 2) == 0` | None | stdout diagnostics only | Wall-clock measurement | Left unchanged because output reports elapsed wall-clock tick time; no reusable tick run condition claim. |

## Non-selected scan notes

The scan also found `examples/player_list.rs` periodic display/despawn changes and `examples/potions.rs` status-effect active-tick arithmetic. Those systems were not selected for this change because the proposal called out the particles/equipment/game_of_life/bench_players slice, and the potion arithmetic is effect-duration policy rather than reusable example tick cadence. They remain non-claims for this archive.

## Contract summary

`TickCadence` is a pure value with a non-zero tick interval and explicit `TickPhase`. `current_tick_is_due` is the deterministic core. `every_ticks`, `once_per_second`, and `once_per_second_with_phase` are thin Bevy run-condition shells over `Option<Res<Server>>`; missing `ServerPlugin` fails closed instead of panicking. `TickCadence::try_from_ticks` returns typed `InvalidTickInterval` errors for zero, negative, and overflow intervals.

## Verification evidence

- Baseline before helper edits: `docs/evidence/run-logs/2026-06-25/add-reusable-tick-run-conditions.valence-server-common-baseline.pre.run.log`.
- Helper contract and negative tests: `docs/evidence/run-logs/2026-06-25/add-reusable-tick-run-conditions.valence-server-common.after-clippy-fix.run.log`.
- Changed example behavior tests: `docs/evidence/run-logs/2026-06-25/add-reusable-tick-run-conditions.changed-examples.after-clippy-fix.run.log`.
- Formatting: `docs/evidence/run-logs/2026-06-25/add-reusable-tick-run-conditions.valence-fmt-check.after-clippy-fix.run.log`.
- Focused helper clippy: `docs/evidence/run-logs/2026-06-25/add-reusable-tick-run-conditions.valence-server-common-clippy-nodeps.after-clippy-fix.run.log`.
- Schedule hygiene: `docs/evidence/run-logs/2026-06-25/add-reusable-tick-run-conditions.schedule-hygiene.final.run.log`.
