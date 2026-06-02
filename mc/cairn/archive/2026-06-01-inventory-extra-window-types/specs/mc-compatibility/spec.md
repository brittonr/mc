# Delta: Inventory extra window types rail

## Requirements

### Requirement: Contract

r[mc_compatibility.inventory_extra_window_types.contract] The `extra inventory window types` row MUST define a bounded deterministic evidence contract before promotion.

#### Scenario: Contract names exact scope

r[mc_compatibility.inventory_extra_window_types.contract.scope]
- GIVEN `inventory-extra-window-types` work starts
- WHEN the evidence contract is reviewed
- THEN it names one additional configured window type with open, click/transfer, close, and final inventory/window state metrics
- AND it states that all window types, crafting/furnace/chest rows already scoped elsewhere, all container transactions, all inventory semantics, and production readiness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.inventory_extra_window_types.checker] A deterministic checker MUST compare normalized metrics before `extra inventory window types` evidence is promoted.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.inventory_extra_window_types.checker.rejects]
- GIVEN evidence is missing or mismatches window type, window id, opened title/type, slot mapping, action item/count, final window slot state, final player inventory state, and server correlation
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

### Requirement: Evidence standard

r[mc_compatibility.inventory_extra_window_types.evidence_standard] `extra inventory window types` promotion MUST enforce the row-specific evidence standard.

#### Scenario: Evidence standard is required

r[mc_compatibility.inventory_extra_window_types.evidence_standard.enforced]
- GIVEN the row requires paired or Valence-scoped receipt with window-type checker and explicit non-claims for other windows
- WHEN evidence lacks that standard
- THEN promotion fails before matrix or current-bundle docs change.

### Requirement: Rail isolation

r[mc_compatibility.inventory_extra_window_types.rail] The harness MUST expose `inventory-extra-window-types` without changing existing row semantics.

#### Scenario: Existing claims remain unchanged

r[mc_compatibility.inventory_extra_window_types.rail.isolated]
- GIVEN existing maintained scenarios and docs
- WHEN `inventory-extra-window-types` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required evidence fields.

### Requirement: Reviewable artifacts

r[mc_compatibility.inventory_extra_window_types.artifacts] Review-critical `extra inventory window types` artifacts MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.inventory_extra_window_types.artifacts.reviewable]
- GIVEN the row is completed
- WHEN reviewers inspect the repo
- THEN receipts, logs, checker output, BLAKE3 manifests, and any required oracle checkpoints are present under `docs/evidence/`.

### Requirement: Matrix and bundle labels

r[mc_compatibility.inventory_extra_window_types.matrix] Acceptance matrix and current bundle MUST promote only the configured `extra inventory window types` row after evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.inventory_extra_window_types.matrix.nonclaims]
- GIVEN `extra inventory window types` evidence passes
- WHEN docs are updated
- THEN only the configured row is marked covered
- AND all window types, crafting/furnace/chest rows already scoped elsewhere, all container transactions, all inventory semantics, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.inventory_extra_window_types.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.inventory_extra_window_types.validation.log]
- GIVEN the row is archived
- WHEN validation is reviewed
- THEN repo-local logs show row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
