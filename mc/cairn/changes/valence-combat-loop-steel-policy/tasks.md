# Tasks

## Phase: call-site inventory

- [x] [serial] Identify every Valence CTF projectile-probe damage consumer and record a call-site list with source spans, current constant/default, and expected Steel config paths. r[runtime_configuration.valence_combat_loop_steel_policy.evidence]
- [x] [serial] Update runtime config inventory/evidence rows so `combat.arrow.*` explicitly name the Valence combat-loop consumers and remain `hot`. r[runtime_configuration.valence_combat_loop_steel_policy.evidence]

## Phase: policy core

- [x] [parallel] Add a pure Valence arrow-policy core for context construction, decision validation, default fallback, range checks, clamping, redaction, and rollback decisions. r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish]
- [x] [parallel] Add positive and negative unit tests for default policy, edited non-default policy, malformed/capability-invalid module, range-invalid candidate values, clamping, redaction, and previous-snapshot preservation. r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish]
- [ ] [parallel] Add explicit snapshot-diff core and tests, or remove diffing from the final design before archive. r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish]
- [ ] [parallel] Add a range-invalid decision-output test if the policy decision path can produce unbounded output, or document why current clamp-before-validation makes that path unreachable. r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish]

## Phase: runtime shell

- [x] [serial] Add a thin Valence runtime shell that loads Steel-compatible module text at startup, handles explicit reload requests, validates candidates through a restricted literal normalizer/capability-token rejector, and atomically publishes typed policy snapshots. r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish]
- [x] [serial] Emit Valence milestone/evidence lines for publish success, rejection diagnostics, policy id, snapshot generation, damage, clamped flag, and victim health delta. r[runtime_configuration.valence_combat_loop_steel_policy.arrow_damage_live]
- [ ] [serial] Replace the restricted literal normalizer with real restricted Steel evaluator execution, or keep the final spec limited to the literal subset before archive. r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish]

## Phase: combat integration

- [x] [serial] Replace the melee/combat `projectile_probe_hit` fixed-damage branch with a call through the published Steel policy snapshot. r[runtime_configuration.valence_combat_loop_steel_policy.arrow_damage_live]
- [x] [serial] Replace `handle_projectile_events` fixed projectile-probe damage with the same published Steel policy helper. r[runtime_configuration.valence_combat_loop_steel_policy.arrow_damage_live]
- [x] [serial] Keep `PROJECTILE_PROBE_DAMAGE` only as the default policy input/test oracle, not as a hidden alternate path after publish. r[runtime_configuration.valence_combat_loop_steel_policy.arrow_damage_live]

## Phase: verification and evidence

- [ ] [parallel] Add checker logic that parses the call-site list and verifies inventory, Steel exports, typed boundary, Valence consumers, snapshot mutability, and evidence receipts all agree for `combat.arrow.*`. r[runtime_configuration.valence_combat_loop_steel_policy.evidence]
- [ ] [parallel] Add live or live-equivalent protocol-763 evidence showing a non-default Steel arrow damage value changes Valence projectile-probe victim health in both projectile call sites. r[runtime_configuration.valence_combat_loop_steel_policy.arrow_damage_live]
- [ ] [parallel] Add negative evidence showing malformed or range-invalid reload is rejected and the previous Valence combat policy remains active. r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish]
- [ ] [serial] Run focused Valence checks, compatibility-runner tests, evidence manifest validation, Cairn gates, and Cairn validate before archiving. r[runtime_configuration.valence_combat_loop_steel_policy.evidence]

## Progress

- Call-site inventory: `docs/evidence/valence-combat-loop-steel-policy-call-sites-2026-05-28.tsv` records both Valence CTF projectile-probe consumers.
- `valence/examples/ctf.rs` now has a pure arrow-policy core, startup/env reload shell with restricted Steel-compatible literal normalization, atomic publish via a typed snapshot, positive and negative unit tests, and both projectile-probe call sites route through `projectile_probe_damage_decision()`.
- `tools/mc-compat-runner/src/main.rs` now passes `MC_COMPAT_PROJECTILE_PROBE` and `MC_COMPAT_STEEL_CONFIG` through to the Valence example for projectile scenarios.

## Notes

- This change is not complete until active Valence/server combat-loop evidence exists; runner-only projectile evidence is insufficient.
- This change does not currently claim real Steel evaluator execution inside Valence; current Valence ingestion is a restricted Steel-compatible literal subset.
- Do not claim vanilla parity or all combat rule migration.
- Do not archive until both projectile-probe call sites are migrated or explicitly removed from scope with spec/task updates.
