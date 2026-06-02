# Delta: Vanilla combat reference parity rail

## Requirements

### Requirement: Contract

r[mc_compatibility.vanilla_combat_reference_parity.contract] The `vanilla combat parity` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.vanilla_combat_reference_parity.contract.scope]
- GIVEN `vanilla-combat-reference-parity` work starts
- WHEN the evidence contract is reviewed
- THEN it names one bounded combat interaction with configured weapon, armor state, attacker/victim positions, damage delta, and knockback/velocity tolerance
- AND it states that all combat balancing, all weapons, all armor/enchantments/status effects, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, and production readiness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.vanilla_combat_reference_parity.checker] A deterministic checker MUST compare normalized metrics before the `vanilla combat parity` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.vanilla_combat_reference_parity.checker.rejects]
- GIVEN evidence is missing or mismatches attacker identity, victim identity, weapon, armor state, pre/post health, damage delta, velocity vector or knockback displacement, tolerance bounds, and reference version
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.vanilla_combat_reference_parity.checker.standard]
- GIVEN the row requires paired Paper/reference and Valence receipts with normalized metric comparison and explicit tolerance fields
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.vanilla_combat_reference_parity.rail] The harness MUST expose a `vanilla-combat-reference-parity` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.vanilla_combat_reference_parity.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `vanilla-combat-reference-parity` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.vanilla_combat_reference_parity.evidence] `vanilla combat parity` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.vanilla_combat_reference_parity.evidence.reviewable]
- GIVEN the `vanilla combat parity` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.vanilla_combat_reference_parity.matrix] Acceptance matrix and current-bundle docs MUST promote only the `vanilla combat parity` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.vanilla_combat_reference_parity.matrix.nonclaims]
- GIVEN `vanilla combat parity` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `vanilla combat parity` row is marked covered
- AND all combat balancing, all weapons, all armor/enchantments/status effects, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.vanilla_combat_reference_parity.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.vanilla_combat_reference_parity.validation.log]
- GIVEN the `vanilla combat parity` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
