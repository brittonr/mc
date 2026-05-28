# Delta: Valence combat-loop Steel arrow policy

## ADDED Requirements

### Requirement: Valence combat-loop arrow policy

r[runtime_configuration.valence_combat_loop_steel_policy.arrow_damage_live] Valence CTF projectile-probe combat MUST source arrow damage from the latest Rust-validated Steel policy snapshot after atomic publish succeeds.

#### Scenario: Published policy drives Valence projectile combat

r[runtime_configuration.valence_combat_loop_steel_policy.arrow_damage_live.scenario]
- GIVEN a valid Steel arrow-damage policy snapshot is published
- WHEN the Valence CTF projectile-probe combat paths apply projectile damage
- THEN both the combat-event path and the projectile-interaction path use the published policy decision
- AND milestone/evidence output records the policy id, snapshot generation or hash, damage, clamped flag, and victim health delta
- AND the old projectile damage constant is used only as the default policy input before an operator override is published

### Requirement: Atomic Valence policy publish

r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish] Valence policy reload MUST publish a candidate arrow policy only after sandbox evaluation, typed normalization, decision validation, and apply preparation all succeed.

#### Scenario: Invalid reload preserves active combat policy

r[runtime_configuration.valence_combat_loop_steel_policy.atomic_publish.scenario]
- GIVEN an active Valence arrow policy snapshot is serving combat decisions
- WHEN a candidate Steel policy is malformed, capability-invalid, nondeterministic, type-invalid, range-invalid, or fails representative decision validation
- THEN reload returns diagnostics without publishing the candidate
- AND subsequent Valence projectile-probe combat uses the previous active snapshot
- AND evidence records the rejection reason without leaking secret-like values

### Requirement: Valence policy evidence coverage

r[runtime_configuration.valence_combat_loop_steel_policy.evidence] A Valence combat-loop Steel policy migration MUST NOT be marked complete until tests, checker output, and reviewable evidence prove the live server call sites use the published policy.

#### Scenario: Evidence ties config to live call sites

r[runtime_configuration.valence_combat_loop_steel_policy.evidence.scenario]
- GIVEN tasks claim Valence combat-loop arrow damage is Steel-managed
- WHEN the checker reviews inventory, Steel exports, typed Rust boundary, Valence call-site list, milestone receipt, and BLAKE3 evidence manifest
- THEN each artifact names the same `combat.arrow.*` config paths, hot mutability class, and Valence projectile-probe consumers
- AND unmatched call sites or missing receipts keep the task incomplete
