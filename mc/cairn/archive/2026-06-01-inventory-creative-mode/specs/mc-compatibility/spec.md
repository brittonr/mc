# Delta: Creative mode inventory rail

## Requirements

### Requirement: Contract

r[mc_compatibility.inventory_creative_mode.contract] The `creative-mode inventory` row MUST define a bounded deterministic evidence contract before promotion.

#### Scenario: Contract names exact scope

r[mc_compatibility.inventory_creative_mode.contract.scope]
- GIVEN `inventory-creative-mode` work starts
- WHEN the evidence contract is reviewed
- THEN it names one configured creative inventory action under an owned local fixture with explicit permission, item id/count, and resulting slot state
- AND it states that all creative actions, operator/admin safety, public-server creative permissions, all inventory transactions, production readiness, and broad protocol coverage remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.inventory_creative_mode.checker] A deterministic checker MUST compare normalized metrics before `creative-mode inventory` evidence is promoted.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.inventory_creative_mode.checker.rejects]
- GIVEN evidence is missing or mismatches game mode, permission state, creative action type, item id, item count, target slot, client observation, server inventory state, and forbidden survival-only assumptions
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

### Requirement: Evidence standard

r[mc_compatibility.inventory_creative_mode.evidence_standard] `creative-mode inventory` promotion MUST enforce the row-specific evidence standard.

#### Scenario: Evidence standard is required

r[mc_compatibility.inventory_creative_mode.evidence_standard.enforced]
- GIVEN the row requires live receipt with explicit creative-mode fixture and checker fixtures for missing permission or wrong item state
- WHEN evidence lacks that standard
- THEN promotion fails before matrix or current-bundle docs change.

### Requirement: Rail isolation

r[mc_compatibility.inventory_creative_mode.rail] The harness MUST expose `inventory-creative-mode` without changing existing row semantics.

#### Scenario: Existing claims remain unchanged

r[mc_compatibility.inventory_creative_mode.rail.isolated]
- GIVEN existing maintained scenarios and docs
- WHEN `inventory-creative-mode` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required evidence fields.

### Requirement: Reviewable artifacts

r[mc_compatibility.inventory_creative_mode.artifacts] Review-critical `creative-mode inventory` artifacts MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.inventory_creative_mode.artifacts.reviewable]
- GIVEN the row is completed
- WHEN reviewers inspect the repo
- THEN receipts, logs, checker output, BLAKE3 manifests, and any required oracle checkpoints are present under `docs/evidence/`.

### Requirement: Matrix and bundle labels

r[mc_compatibility.inventory_creative_mode.matrix] Acceptance matrix and current bundle MUST promote only the configured `creative-mode inventory` row after evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.inventory_creative_mode.matrix.nonclaims]
- GIVEN `creative-mode inventory` evidence passes
- WHEN docs are updated
- THEN only the configured row is marked covered
- AND all creative actions, operator/admin safety, public-server creative permissions, all inventory transactions, production readiness, and broad protocol coverage remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.inventory_creative_mode.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.inventory_creative_mode.validation.log]
- GIVEN the row is archived
- WHEN validation is reviewed
- THEN repo-local logs show row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
