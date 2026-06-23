# Observability/profiling hooks scope and contract

## Question

Drain `add-observability-profiling-hooks` without copying Hyperion runtime/profiler code into Valence core, while adding optional Valence-owned hooks that are disabled unless explicitly installed.

## Inspected evidence

- `hyperion/README.md`: records Hyperion's tracing/profiling motivation and high-player-count goals; the capacity numbers remain Hyperion-specific evidence, not Valence evidence.
- `hyperion/events/bedwars/src/main.rs`: uses `tracing_subscriber` with an optional/commented `TracyLayer` adapter.
- `hyperion/events/bedwars/src/plugin/stats.rs`: samples tick time and player count inside a Bevy plugin and uses a `tracing::info_span!("stats")` shell.
- `hyperion/crates/hyperion-stats/src/lib.rs`: computes statistics in a separate helper crate, but uses nightly SIMD and is not copied.
- `servers/valence/crates/valence_server/src/event/loop.rs`: exposes `PacketEvent` and Valence event-loop phase schedules.
- `servers/valence/crates/valence_server/src/anticheat.rs`: existing optional-plugin pattern with a pure calculation core and Bevy event shell.

## Hyperion boundary inventory

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/README.md` performance tracing notes | reference | `add-observability-profiling-hooks` | Motivates optional observability vocabulary only; no capacity claim or code copied. | `servers/valence/crates/valence_server/src/observability.rs` docs and tests | Stable Rust only; no runtime, proxy, or profiler import. | `docs/evidence/add-observability-profiling-hooks-valence-server-observability-tests.run.log` | No Valence production scale, Hyperion compatibility, or load capacity claim. |
| `hyperion/events/bedwars/src/main.rs` tracing subscriber / commented Tracy adapter | reference | `add-observability-profiling-hooks` | Confirms profiler adapters should stay outside core hooks. | Optional `ObservabilityExporter` shell trait | No `tracing-tracy` or profiler dependency added; users can export events separately. | exporter-failure fixture in `observability::tests` | No mandatory profiler, no Tracy integration claim. |
| `hyperion/events/bedwars/src/plugin/stats.rs` Bevy stats plugin | reference | `add-observability-profiling-hooks` | Informs Valence-owned optional plugin shape and tick-phase labels. | `ObservabilityPlugin` | No Hyperion `Compose`, gameplay, or Bedwars code copied. | enabled/disabled plugin tests | No Bedwars behavior, no gameplay policy, no player-list stats UI. |
| `hyperion/crates/hyperion-stats/src/lib.rs` SIMD statistics helper | reject | `add-observability-profiling-hooks` | Nightly SIMD helper is unnecessary for initial hooks and would violate stable/minimal scope. | none | Rejected for direct import; Valence uses enum classification only. | scope note plus no dependency change | No statistics algorithm parity or SIMD performance claim. |

## Selected Valence hook scope

- Selected subsystems: tick phase boundaries and serverbound network packet observations from Valence `PacketEvent`.
- Deferred subsystems: chunk, entity, and plugin-specific detailed hooks remain label vocabulary only until separate evidence justifies wiring.
- Disabled mode: `ObservabilityPlugin` is not in `DefaultPlugins`; without it no config resource or observability event resource is installed. `ObservabilityConfig::disabled()` also makes installed systems no-op.
- Adapter boundary: `ObservabilityExporter` is a shell trait over classified records. Exporter failure returns `ObservabilityExportOutcome::Failed` and does not panic or mutate server logic.
- Non-goals: no profiler dependency, no tracing subscriber installation, no public-server safety claim, no production capacity claim, no broad Minecraft compatibility claim, and no semantic equivalence claim.

## Stable names and bounded labels

| record | kind | labels | sensitive fields |
| --- | --- | --- | --- |
| `valence.tick.phase` | span | `subsystem=tick`, `phase in {pre_update,event_loop_pre_update,event_loop_update,event_loop_post_update,post_update}`, `redaction=none` | None included. |
| `valence.network.packet.serverbound` | counter | `subsystem=network`, `direction=serverbound`, `packet_id_class in {known,unknown}`, `redaction=omitted_sensitive_input` | Raw packet payload, client entity, username, UUID, and address are omitted. |
| `valence.observability.exporter.failure` | counter | `subsystem=exporter`, `redaction=omitted_sensitive_input` | Exporter internals and record payloads are not exposed by the failure label. |

Unknown metric names fail closed through `ObservabilityMetricName::parse` with diagnostic `unknown observability metric name`. Raw player identifiers, socket addresses, packet payloads, and user text map to the redaction marker `<redacted>` if a shell needs to represent them.

## Overhead expectations

- Default overhead is zero from this module because the plugin is absent from `DefaultPlugins`.
- Installed-but-disabled overhead is limited to no-op systems that clear packet reads when configured off.
- Installed-and-enabled overhead is bounded to enum classification, event emission, and no raw payload cloning beyond reading the existing `Bytes` handle.
- This is a qualitative local overhead expectation only; no production throughput, latency, or load capacity is claimed.

## Verification evidence

- Baseline advisory-plugin tests: `docs/evidence/add-observability-profiling-hooks-baseline-valence-server-anticheat-tests.run.log` (`exit_status=0`).
- Observability positive/negative fixtures: `docs/evidence/add-observability-profiling-hooks-valence-server-observability-tests.run.log` (`exit_status=0`).
- Default-plugin disabled regression: `docs/evidence/add-observability-profiling-hooks-default-plugin-regression.run.log` (`exit_status=0`).
- Cairn pre-implementation gates: `docs/evidence/add-observability-profiling-hooks-pre-gate-proposal.run.log`, `docs/evidence/add-observability-profiling-hooks-pre-gate-design.run.log`, `docs/evidence/add-observability-profiling-hooks-pre-gate-tasks.run.log`, and `docs/evidence/add-observability-profiling-hooks-pre-validate.run.log` all contain `exit_status=0`.
