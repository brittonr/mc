# Gate optional systems with Bevy run conditions inventory

## Question

Which optional Valence systems with runtime enabled checks are targeted for Bevy run-condition work, and what disabled-mode contract applies to each event reader?

## Inspected evidence

- `valence_server::observability::ObservabilityPlugin` installs optional tick-phase and packet-observation systems only when explicitly added.
- `ObservabilityConfig` owns runtime switches: `enabled`, `emit_tick_phases`, and `emit_network_packets`.
- `emit_network_packet_records` reads `PacketEvent`; tick-phase systems do not read events.
- `AnticheatStatisticsPlugin` was inspected as an optional event-reading plugin, but it has no runtime enabled switch; plugin absence is its disabled mode.
- Cached chunk egress was inspected as an enable flag, but it is per-layer packet rendering state rather than a Bevy system with an event reader.

## Decision

| Target | Runtime config | Event reader | Disabled contract | Toggle expectation |
| --- | --- | --- | --- | --- |
| Tick-phase observability systems | `enabled && emit_tick_phases` | None | Skip with Bevy `run_if`; disabled bodies do not run. | Re-enable emits only future tick-phase records. |
| `emit_network_packet_records` | `enabled && emit_network_packets` | `PacketEvent` | Keep explicit in-system guard and drain; disabled updates clear the same reader and emit no records. | Re-enable does not replay disabled-period packets; new packets are observed. |

## Owner

Valence `valence_server` observability plugin.

## Next action

Use focused `valence_server` observability tests plus schedule hygiene, Cairn gates, Cairn validation, and task-evidence checks as closeout evidence. Preserve non-claims: this is not broad Minecraft compatibility, semantic equivalence, public-server safety, production readiness, or full gameplay correctness evidence.
