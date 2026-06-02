# Delta: Projectile weapon variants rail

## Requirements

### Requirement: Contract

r[mc_compatibility.projectile_weapon_variants.contract] The `projectile weapon variants` row MUST define a bounded deterministic evidence contract before promotion.

#### Scenario: Contract names exact scope

r[mc_compatibility.projectile_weapon_variants.contract.scope]
- GIVEN `projectile-weapon-variants` work starts
- WHEN the evidence contract is reviewed
- THEN it names a bounded matrix of configured projectile weapons with use, spawn, hit/miss, damage or no-damage outcome, and per-weapon non-claims
- AND it states that all projectile weapons, projectile travel physics, exact vanilla projectile parity, enchantments/status effects, combat balancing, and production readiness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.projectile_weapon_variants.checker] A deterministic checker MUST compare normalized metrics before `projectile weapon variants` evidence is promoted.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.projectile_weapon_variants.checker.rejects]
- GIVEN evidence is missing or mismatches weapon id, ammunition/item state, use action, projectile spawn, target identity, hit/miss outcome, damage delta when applicable, and server correlation
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

### Requirement: Evidence standard

r[mc_compatibility.projectile_weapon_variants.evidence_standard] `projectile weapon variants` promotion MUST enforce the row-specific evidence standard.

#### Scenario: Evidence standard is required

r[mc_compatibility.projectile_weapon_variants.evidence_standard.enforced]
- GIVEN the row requires matrix checker with one live receipt per promoted weapon row and optional reference parity only when paired evidence exists
- WHEN evidence lacks that standard
- THEN promotion fails before matrix or current-bundle docs change.

### Requirement: Rail isolation

r[mc_compatibility.projectile_weapon_variants.rail] The harness MUST expose `projectile-weapon-variants` without changing existing row semantics.

#### Scenario: Existing claims remain unchanged

r[mc_compatibility.projectile_weapon_variants.rail.isolated]
- GIVEN existing maintained scenarios and docs
- WHEN `projectile-weapon-variants` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required evidence fields.

### Requirement: Reviewable artifacts

r[mc_compatibility.projectile_weapon_variants.artifacts] Review-critical `projectile weapon variants` artifacts MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.projectile_weapon_variants.artifacts.reviewable]
- GIVEN the row is completed
- WHEN reviewers inspect the repo
- THEN receipts, logs, checker output, BLAKE3 manifests, and any required oracle checkpoints are present under `docs/evidence/`.

### Requirement: Matrix and bundle labels

r[mc_compatibility.projectile_weapon_variants.matrix] Acceptance matrix and current bundle MUST promote only the configured `projectile weapon variants` row after evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.projectile_weapon_variants.matrix.nonclaims]
- GIVEN `projectile weapon variants` evidence passes
- WHEN docs are updated
- THEN only the configured row is marked covered
- AND all projectile weapons, projectile travel physics, exact vanilla projectile parity, enchantments/status effects, combat balancing, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.projectile_weapon_variants.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.projectile_weapon_variants.validation.log]
- GIVEN the row is archived
- WHEN validation is reviewed
- THEN repo-local logs show row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
