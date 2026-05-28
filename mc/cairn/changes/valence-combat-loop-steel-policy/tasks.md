# Tasks

## Phase: call-site inventory

- [ ] [serial] Identify every Valence CTF projectile-probe damage consumer and record a call-site list with source spans, current constant/default, and expected Steel config paths. r[runtime_configuration.valence_combat_loop_steel_policy.evidence]
- [ ] [serial] Update runtime config inventory/evidence rows so `combat.arrow.*` explicitly name the Valence combat-loop consumers and remain `hot`. r[runtime_configuration.valence_combat_loop_steel_policy.evidence]

## Phase: policy core

- [ ] [parallel] Add a pure Valence arrow-policy core for context construction, decision validation, default fallback, range checks, clamping, redaction, snapshot diffing, and rollback decisions. r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish]
- [ ] [parallel] Add positive and negative unit tests for default policy, edited non-default policy, invalid/malformed policy, range-invalid output, clamping, and previous-snapshot preservation. r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish]

## Phase: runtime shell

- [ ] [serial] Add a thin Valence runtime shell that loads the Steel module at startup, handles explicit reload requests, validates candidates in the restricted sandbox, and atomically publishes typed policy snapshots. r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish]
- [ ] [serial] Emit Valence milestone/evidence lines for publish success, rejection diagnostics, policy id, snapshot generation/hash, damage, clamped flag, and victim health delta. r[runtime_configuration.valence_combat_loop_steel_policy.arrow_damage_live]

## Phase: combat integration

- [ ] [serial] Replace the melee/combat `projectile_probe_hit` fixed-damage branch with a call through the published Steel policy snapshot. r[runtime_configuration.valence_combat_loop_steel_policy.arrow_damage_live]
- [ ] [serial] Replace `handle_projectile_events` fixed projectile-probe damage with the same published Steel policy helper. r[runtime_configuration.valence_combat_loop_steel_policy.arrow_damage_live]
- [ ] [serial] Keep `PROJECTILE_PROBE_DAMAGE` only as the default policy input/test oracle, not as a hidden alternate path after publish. r[runtime_configuration.valence_combat_loop_steel_policy.arrow_damage_live]

## Phase: verification and evidence

- [ ] [parallel] Add checker logic that parses the call-site list and verifies inventory, Steel exports, typed boundary, Valence consumers, snapshot mutability, and evidence receipts all agree for `combat.arrow.*`. r[runtime_configuration.valence_combat_loop_steel_policy.evidence]
- [ ] [parallel] Add live or live-equivalent protocol-763 evidence showing a non-default Steel arrow damage value changes Valence projectile-probe victim health in both projectile call sites. r[runtime_configuration.valence_combat_loop_steel_policy.arrow_damage_live]
- [ ] [parallel] Add negative evidence showing malformed or range-invalid reload is rejected and the previous Valence combat policy remains active. r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish]
- [ ] [serial] Run focused Valence checks, compatibility-runner tests, evidence manifest validation, Cairn gates, and Cairn validate before archiving. r[runtime_configuration.valence_combat_loop_steel_policy.evidence]

## Notes

- This change is not complete until active Valence/server combat-loop evidence exists; runner-only projectile evidence is insufficient.
- Do not claim vanilla parity or all combat rule migration.
- Do not archive until both projectile-probe call sites are migrated or explicitly removed from scope with spec/task updates.
