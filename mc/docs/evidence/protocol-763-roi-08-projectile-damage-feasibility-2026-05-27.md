# Protocol-763 ROI 08 projectile damage feasibility checkpoint

## Oracle checkpoint

- Question: Can existing Stevenarella and Valence CTF instrumentation support a bounded projectile collision/damage attribution rail?
- Inspected evidence:
  - `valence/examples/ctf.rs` emits `projectile_loadout`, `projectile_use`, `projectile_hit`, and mutates victim health when `MC_COMPAT_PROJECTILE_PROBE` is enabled.
  - `stevenarella/src/server/mod.rs` emits `projectile_probe_use_item_sent`, `projectile_probe_swing_sent`, and generic `update_health health=...` client milestones.
  - `tools/mc-compat-runner/src/main.rs` can correlate multi-client client logs with the Valence server log through `evaluate_scenario` and `evaluate_server_scenario`.
- Decision owner: agent.
- Decision: feasible as a bounded damage-attribution rail if and only if the new scenario requires client projectile use/swing plus a client health update and server `projectile_use`/`projectile_hit` milestones.
- Next action: add the deterministic dry-run gate first, then attempt a live run and keep full projectile physics/all-weapon/exact-vanilla claims false.

## Evidence snippets

```text
42:const PROJECTILE_PROBE_DAMAGE: f32 = 3.0;
806:                    "MC-COMPAT-MILESTONE projectile_loadout username={} slot=0 item=Bow arrows=16",
1328:            PROJECTILE_PROBE_DAMAGE
1335:                "MC-COMPAT-MILESTONE projectile_use attacker={} victim={} item={:?}",
1376:                "MC-COMPAT-MILESTONE projectile_hit attacker={} victim={} damage={:.1} \
1473:        victim_health.0 -= PROJECTILE_PROBE_DAMAGE;
1476:            "MC-COMPAT-MILESTONE projectile_use attacker={} victim={} hand={:?} \
1482:            PROJECTILE_PROBE_DAMAGE
1487:            "MC-COMPAT-MILESTONE projectile_hit attacker={} victim={} \
111:    projectile_probe_use_item_sent: bool,
112:    projectile_probe_swing_sent: bool,
662:            projectile_probe_use_item_sent: false,
663:            projectile_probe_swing_sent: false,
956:            && !self.projectile_probe_use_item_sent
960:            info!("MC-COMPAT-MILESTONE projectile_probe_use_item_sent hand=main sequence=303");
965:            self.projectile_probe_use_item_sent = true;
970:            && !self.projectile_probe_swing_sent
972:            info!("MC-COMPAT-MILESTONE projectile_probe_swing_sent hand=main");
976:            self.projectile_probe_swing_sent = true;
2373:            "MC-COMPAT-MILESTONE update_health health={:.1} food={} saturation={:.1}",
```
