# Delta: Armor loadout enchantment status matrix rail

## Requirements

### Requirement: Contract

r[mc_compatibility.armor_loadout_enchantment_status_matrix.contract] The `armor/enchantment/status matrix` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.armor_loadout_enchantment_status_matrix.contract.scope]
- GIVEN `armor-loadout-enchantment-status-matrix` work starts
- WHEN the evidence contract is reviewed
- THEN it names a bounded table of configured armor loadout, enchantment, status-effect, attack type, and expected mitigation rows
- AND it states that all armor permutations, all enchantments, all status effects, exact vanilla balancing outside listed rows, production readiness, and full combat correctness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.armor_loadout_enchantment_status_matrix.checker] A deterministic checker MUST compare normalized metrics before the `armor/enchantment/status matrix` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.armor_loadout_enchantment_status_matrix.checker.rejects]
- GIVEN evidence is missing or mismatches loadout id, equipment slots, enchantment ids/levels, status effects, attack type, pre/post health, damage delta, mitigation delta, and tolerance fields
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.armor_loadout_enchantment_status_matrix.checker.standard]
- GIVEN the row requires matrix checker with positive and negative rows plus paired reference evidence for any vanilla-parity label
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.armor_loadout_enchantment_status_matrix.rail] The harness MUST expose a `armor-loadout-enchantment-status-matrix` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.armor_loadout_enchantment_status_matrix.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `armor-loadout-enchantment-status-matrix` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.armor_loadout_enchantment_status_matrix.evidence] `armor/enchantment/status matrix` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.armor_loadout_enchantment_status_matrix.evidence.reviewable]
- GIVEN the `armor/enchantment/status matrix` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.armor_loadout_enchantment_status_matrix.matrix] Acceptance matrix and current-bundle docs MUST promote only the `armor/enchantment/status matrix` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.armor_loadout_enchantment_status_matrix.matrix.nonclaims]
- GIVEN `armor/enchantment/status matrix` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `armor/enchantment/status matrix` row is marked covered
- AND all armor permutations, all enchantments, all status effects, exact vanilla balancing outside listed rows, production readiness, and full combat correctness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.armor_loadout_enchantment_status_matrix.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.armor_loadout_enchantment_status_matrix.validation.log]
- GIVEN the `armor/enchantment/status matrix` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
