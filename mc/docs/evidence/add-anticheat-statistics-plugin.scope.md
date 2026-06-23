# add-anticheat-statistics-plugin scope and boundary evidence

## Requirement coverage

- `r[valence_hyperion_integration.anticheat_stats.scope]`
- `r[valence_hyperion_integration.anticheat_stats.scope.bounded]`

## Selected metric scope

This change implements advisory statistics only. The selected Valence-owned metrics are:

| Metric | Event source | Window | Output | Non-goal |
| --- | --- | --- | --- | --- |
| Packet cadence | `valence_server::event_loop::PacketEvent` | Validated rolling `SampleWindowSettings` | `AnticheatStatisticsEvent` with packet-count snapshot | No packet throttling, disconnect, or public-server load claim |
| Movement delta | `valence_server::movement::MovementEvent` | Same explicit rolling window | Observation with distance snapshot | No vanilla movement legality, collision, flight, or teleport policy claim |
| Rotation delta | `valence_server::movement::MovementEvent` | Same explicit rolling window | Observation with yaw/pitch delta snapshot | No aimbot detection or enforcement claim |

The default window keeps `DEFAULT_SAMPLE_WINDOW_CAPACITY` samples across `DEFAULT_SAMPLE_WINDOW_TICKS` plugin ticks per metric and player. The plugin-local tick is supplied by the shell and passed into the pure core as an explicit `TimedSample` field.

## Hyperion integration boundary inventory

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/crates/hyperion-stats/README.md` | reference | `add-anticheat-statistics-plugin` | Metric vocabulary informed the bounded Valence metric list. | `servers/valence/crates/valence_server/src/anticheat.rs` and README docs | No code copied; used only to select packet cadence, movement delta, and rotation delta observations. | Pure core tests and plugin smoke logs under `docs/evidence/`. | No Hyperion compatibility, production anti-cheat, vanilla parity, or enforcement claim. |
| `hyperion/crates/hyperion-stats/src/lib.rs::ParallelStats` | reject | `add-anticheat-statistics-plugin` | The source uses nightly `portable_simd` and array chunk features, while the Valence scope requires stable scalar Rust first. | none | Direct adoption is rejected; Valence implements stable Welford-style scalar summaries over explicit samples. | Overflow, invalid-window, empty-window, and reset fixtures. | No SIMD performance, feature parity, or large-player scalability claim. |
| Hyperion anti-cheat application list | port | `add-anticheat-statistics-plugin` | The concept of advisory real-time metrics is reimplemented with Valence-owned types. | Optional `AnticheatStatisticsPlugin` | Pure core plus thin Bevy event adapter; no unsafe, no global clock, no default behavior change. | `cargo test -p valence_server --lib anticheat` evidence. | No bans, kicks, moderation action, public-server safety, or adversarial security claim. |

## Valence event-source audit

| source_path | selected use | reason | non_claims |
| --- | --- | --- | --- |
| `servers/valence/crates/valence_server/src/event/loop.rs::PacketEvent` | Count inbound packet samples by player entity. | Already carries explicit client entity and packet-arrival metadata. | Packet IDs and payloads are not persisted by this plugin. |
| `servers/valence/crates/valence_server/src/movement.rs::MovementEvent` | Sample movement distance and rotation delta after movement handling emits the event. | Provides current/old position and look values without reading global state. | Does not validate movement legality or mutate movement outcomes. |

## Boundaries and data retention

- The plugin is not added to `valence::DefaultPlugins`; users must add `AnticheatStatisticsPlugin` explicitly.
- The plugin emits observations only. It has no kick, ban, disconnect, teleport, inventory, or gameplay mutation path.
- In-memory retention is limited to the configured rolling samples and lifetime counters per player entity while the app keeps the plugin resource alive.
- No persistent storage, raw packet payload retention, player IP retention, or cross-session telemetry is added.
- Metrics are advisory signals. Any enforcement policy requires a separate plugin and separate evidence.
